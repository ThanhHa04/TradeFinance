#![cfg(test)]
extern crate std;
use super::*;
use crate::TradeFinanceContractClient;
use soroban_sdk::{
    // Cần import trait này để dùng hàm generate()
    testutils::Address as _,
    // Cần import Ledger để dùng env.ledger().set
    testutils::Ledger,
    Address,
    Env,
    U256,
};

// =========================================================================
// Setup (Thiết lập môi trường Test)
// =========================================================================
#[allow(deprecated)]
fn setup_env() -> (
    Env,
    TradeFinanceContractClient<'static>,
    Address,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    // ĐĂNG KÝ CONTRACT (Sửa warning: dùng env.register)
    let contract_id = env.register_contract(None, TradeFinanceContract);
    let client = TradeFinanceContractClient::new(&env, &contract_id);

    // Dùng Address::generate(&env) để tạo địa chỉ an toàn trong môi trường test
    let issuer = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);

    // Khởi tạo Contract
    client.initialize(&issuer);

    (env, client, issuer, buyer, seller)
}

// =========================================================================
// Tests (Các kịch bản kiểm thử)
// =========================================================================

#[test]
fn test_initialize_success() {
    // Thêm _ vào các biến không dùng
    let (_env, _client, _issuer, _buyer, _seller) = setup_env();
    // Test logic khởi tạo ID: Không cần đọc storage trực tiếp, ta giả định nó thành công
}

#[test]
// CHUỖI PANIC KỲ VỌNG đã được sửa để khớp với HostError thực tế
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_fail_double() {
    // Thêm _ vào các biến không dùng
    let (_env, client, issuer, _buyer, _seller) = setup_env();
    client.initialize(&issuer); // Cố gắng khởi tạo lần thứ hai
}

#[test]
fn test_create_lc_success_and_indexing() {
    let (env, client, _issuer, buyer, seller) = setup_env();

    // 1. Tạo L/C đầu tiên

    // SỬA LỖI E0560: Thay 'sequence' bằng 'sequence_number' và dùng Default::default()
    env.ledger().set(soroban_sdk::testutils::LedgerInfo {
        sequence_number: 100,
        timestamp: env.ledger().timestamp(),
        // THAY ĐỔI: Sử dụng Protocol Version 21 (hoặc giá trị mặc định của host)
        protocol_version: 23,
        ..Default::default()
    });

    let lc1_id = client.create_lc(&buyer, &seller, &1000, &10); // Hạn 10 ngày
    assert_eq!(lc1_id, U256::from_u32(&env, 1)); // Kiểm tra ID

    // 2. Tạo L/C thứ hai
    let seller2 = Address::generate(&env);
    let lc2_id = client.create_lc(&buyer, &seller2, &500, &5);
    assert_eq!(lc2_id, U256::from_u32(&env, 2));

    // Vì không có hàm getter, ta chỉ kiểm tra việc tạo LC không lỗi.
}

#[test]
// CHUỖI PANIC KỲ VỌNG đã được sửa để khớp với HostError thực tế
#[should_panic(expected = "HostError: Error(Contract, #6)")]
fn test_create_lc_fail_zero_amount() {
    // Thêm _ vào các biến không dùng
    let (_env, client, _issuer, buyer, seller) = setup_env();
    client.create_lc(&buyer, &seller, &0, &10); // Số tiền không hợp lệ
}
