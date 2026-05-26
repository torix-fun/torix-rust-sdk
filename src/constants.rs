use solana_pubkey::*;

pub use torix_program::*;


pub mod torix_program {
    use super::*;

    pub const ID: Pubkey = pubkey!("torXFavtJnaJzW7fz2NVrg9f1j824GitYi69zhmJQBK");

    pub const ROUND_SEED: &str = "round";
    pub const ROUND_VAULT_SEED: &str = "round_vault";
    pub const CURVE_SEED: &str = "curve";
    pub const GLOBAL_CONFIG_SEED: &str = "global_config";
    pub const MINT_AUTHORITY_SEED: &str = "mint_authority";
}