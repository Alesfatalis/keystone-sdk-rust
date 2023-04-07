use alloc::string::{String, ToString};

pub struct RegistryType<'a>(&'a str, u64);

impl<'a> RegistryType<'_> {
    pub fn get_type(&self) -> String {
        self.0.to_string()
    }
    pub fn get_tag(&self) -> u64 {
        self.1
    }
}

pub const UUID: RegistryType = RegistryType("uuid", 37);
pub const CRYPTO_HDKEY: RegistryType = RegistryType("crypto-hdkey", 303);
pub const CRYPTO_KEYPATH: RegistryType = RegistryType("crypto-keypath", 304);
pub const CRYPTO_COIN_INFO: RegistryType = RegistryType("crypto-coin-info", 305);
pub const CRYPTO_ECKEY: RegistryType = RegistryType("crypto-eckey", 306);
pub const CRYPTO_OUTPUT: RegistryType = RegistryType("crypto-output", 308);
pub const CRYPTO_PSBT: RegistryType = RegistryType("crypto-psbt", 310);
pub const CRYPTO_ACCOUNT: RegistryType = RegistryType("crypto-account", 311);

// Multiple Accounts
pub const CRYPTO_MULTI_ACCOUNTS: RegistryType = RegistryType("crypto-multi-accounts", 1103);

// ETH
pub const ETH_SIGN_REQUEST: RegistryType = RegistryType("eth-sign-request", 401);
pub const ETH_SIGNATURE: RegistryType = RegistryType("eth-signature", 402);
// SOL
pub const SOL_SIGN_REQUEST: RegistryType = RegistryType("sol-sign-request", 1101);
pub const SOL_SIGNATURE: RegistryType = RegistryType("sol-signature", 1102);
// Cosmos
pub const COSMOS_SIGN_REQUEST: RegistryType = RegistryType("sol-sign-request", 4101);
pub const COSMOS_SIGNATURE: RegistryType = RegistryType("sol-signature", 4102);
// Tron
pub const TRON_SIGN_REQUEST: RegistryType = RegistryType("tron-sign-request-kt", 5101);
pub const TRON_SIGNATURE: RegistryType = RegistryType("tron-signature", 5102);