// src/scripts/walletLogic.js

// --- ĐỊA CHỈ VAI TRÒ GIẢ LẬP (SỬ DỤNG PUBLIC KEY THẬT CỦA BẠN TRÊN TESTNET) ---
const ISSUER_ADDRESS = "GBKM37VU3PHNA4VBVVUUPJ25TQOOMUNUSTR27WPNTUM4EVZCS52FMG3P";
const BUYER_TEST_ADDRESS = "GABUYERPUBLICKEYxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
const SELLER_TEST_ADDRESS = "GBSellerPUBLICKEYxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
// --- END ĐỊA CHỈ VAI TRÒ GIẢ LẬP ---

/**
 * Xác định vai trò của người dùng dựa trên public key.
 */
const determineRole = (publicKey) => {
    let roles = [];

    if (publicKey === ISSUER_ADDRESS) {
        return 'Issuer/Bank';
    }

    if (publicKey === BUYER_TEST_ADDRESS) {
        roles.push('Buyer');
    }
    if (publicKey === SELLER_TEST_ADDRESS) {
        roles.push('Seller');
    }

    return roles.length > 0 ? roles.join(' / ') : 'General User';
};

// Hàm xử lý kết nối (ĐANG GIẢ LẬP)
const handleConnect = async () => {
    const connectBtn = document.getElementById('connect-btn');
    const displayContainer = document.getElementById('wallet-display-container');

    if (connectBtn) {
        connectBtn.disabled = true;
        connectBtn.textContent = 'Đang kết nối...';
    }

    try {
        // 🔥🔥🔥 ĐÂY LÀ DÒNG GIẢ LẬP CẦN THAY ĐỔI ĐỂ TEST VAI TRÒ KHÁC 🔥🔥🔥
        // Thay thế bằng BUYER_TEST_ADDRESS hoặc SELLER_TEST_ADDRESS để test vai trò khác
        const publicKey = ISSUER_ADDRESS;

        // 1. Xác định Vai trò
        const role = determineRole(publicKey);

        // 2. Lưu trạng thái vào localStorage
        localStorage.setItem('connectedAddress', publicKey);
        localStorage.setItem('userRole', role);

        // 3. Cập nhật giao diện
        if (displayContainer) {
            displayContainer.innerHTML = `
                <p>Đã kết nối: <code>${publicKey.substring(0, 10)}...${publicKey.substring(publicKey.length - 5)}</code></p>
                <p style="font-weight: bold;">Vai trò: <span>${role}</span></p>
            `;
        }

        // GỬI SỰ KIỆN TÙY CHỈNH để index.astro xử lý chuyển hướng
        window.dispatchEvent(new CustomEvent('walletConnected', { detail: { role } }));

    } catch (error) {
        console.error("Lỗi kết nối ví:", error);
        if (connectBtn) {
            connectBtn.disabled = false;
            connectBtn.textContent = 'Kết nối Ví Freighter';
            alert("Kết nối thất bại. Vui lòng kiểm tra console.");
        }
    }
};

// Gắn sự kiện vào nút sau khi DOM tải xong
document.addEventListener('DOMContentLoaded', () => {
    const connectBtn = document.getElementById('connect-btn');
    if (connectBtn) {
        connectBtn.addEventListener('click', handleConnect);
    }
});