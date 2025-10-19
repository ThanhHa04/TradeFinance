#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, symbol_short, Address, Env, U256,
};

// =========================================================================
// 1. Cấu trúc Dữ liệu & Key (Data Structures & Keys)
// =========================================================================

#[contracttype]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// KHẮC PHỤC LỖI E0277: Implement trait IntoVal cho Enum lỗi để dùng với panic_with_error!
// TradeError::AlreadyInitialized sẽ được chuyển thành Soroban Error
pub enum TradeError {
    AlreadyInitialized = 1,
    LcNotFound = 2,
    InvalidAmount = 6,
}

#[contracttype]
#[derive(Clone)]
pub struct LetterOfCredit {
    pub buyer: Address,           // Người mua (Bên mở L/C)
    pub seller: Address,          // Người bán (Bên thụ hưởng)
    pub amount: i128,             // Số tiền thanh toán
    pub expires_at: u64,          // Thời gian hết hạn L/C (Sequence)
    pub documents_verified: bool, // Trạng thái xác minh tài liệu
    pub disbursed: bool,          // Trạng thái giải ngân
}

#[contracttype]
// KHẮC PHỤC LỖI E0532, E0732, E0308:
// Không thể đặt giá trị (discriminant) cho biến thể có dữ liệu (tuple variant) như Lc(U256).
// Chỉ cần loại bỏ `= 0`, `= 1`, `= 2` ở đây.
pub enum DataKey {
    Issuer,
    NextLcId,
    Lc(U256),
}

// =========================================================================
// 2. Contract và Logic chính (Contract & Core Logic)
// =========================================================================

#[contract]
pub struct TradeFinanceContract;

#[contractimpl]
impl TradeFinanceContract {
    /// Khởi tạo Contract, chỉ định người phát hành (Issuer/Ngân hàng)
    pub fn initialize(env: Env, issuer: Address) {
        let storage = env.storage().persistent();
        if storage.has(&DataKey::Issuer) {
            // Lỗi E0277 đã được khắc phục nhờ IntoVal
            panic_with_error!(&env, TradeError::AlreadyInitialized);
        }

        storage.set(&DataKey::Issuer, &issuer);

        // KHẮC PHỤC LỖI E0061: Hàm U256::from_u32() yêu cầu &Env là tham số đầu tiên
        storage.set(&DataKey::NextLcId, &U256::from_u32(&env, 1)); // Bắt đầu ID từ 1
    }

    /// Tạo một Letter of Credit mới
    pub fn create_lc(
        env: Env,
        buyer: Address,
        seller: Address,
        amount: i128,
        days_valid: u32,
    ) -> U256 {
        buyer.require_auth(); // Người mua phải ký để mở L/C

        if amount <= 0 {
            // Lỗi E0277 đã được khắc phục nhờ IntoVal
            panic_with_error!(&env, TradeError::InvalidAmount);
        }

        let storage = env.storage().persistent();

        // Lấy ID tiếp theo và tăng giá trị ID cho lần sau
        let current_id: U256 = storage.get(&DataKey::NextLcId).unwrap();

        // KHẮC PHỤC LỖI E0061: Hàm U256::from_u32() yêu cầu &Env
        let next_id = current_id.add(&U256::from_u32(&env, 1));

        // Tính thời gian hết hạn theo Ledger Sequence
        let expires_at = env.ledger().sequence() + (days_valid as u32) * 17280;

        let new_lc = LetterOfCredit {
            // SỬA: Dùng .clone() để new_lc nhận bản sao, giữ lại quyền sở hữu cho biến buyer, seller
            buyer: buyer.clone(),
            seller: seller.clone(),

            amount,
            expires_at: expires_at.into(),
            documents_verified: false,
            disbursed: false,
        };

        storage.set(&DataKey::Lc(current_id.clone()), &new_lc);
        storage.set(&DataKey::NextLcId, &next_id);

        // Phát sự kiện L/C được tạo
        // (Cảnh báo về hàm publish bị đánh dấu lỗi thời là bình thường)
        env.events().publish(
            (symbol_short!("lc_creat"), current_id.clone()),
            (buyer, seller, amount),
        );

        current_id
    }
}

impl From<TradeError> for soroban_sdk::Error {
    fn from(e: TradeError) -> Self {
        soroban_sdk::Error::from_contract_error(e as u32)
    }
}

#[cfg(test)]
mod test;
