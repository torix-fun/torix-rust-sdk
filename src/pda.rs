use solana_pubkey::Pubkey;
use spl_token_2022_interface::ID as SPL_TOKEN_2022_PROGRAM_ID;
use spl_associated_token_account_interface::program::ID as SPL_ATA_PROGRAM_ID;
use crate::constants::{
    *, 
    ID as TORIX_PROGRAM_ID
};


pub fn derive_round() -> Pubkey {
    Pubkey::find_program_address(
        &[
            ROUND_SEED.as_ref() 
        ], 
        &TORIX_PROGRAM_ID
    ).0
}

pub fn derive_round_vault(round: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            ROUND_VAULT_SEED.as_ref(), 
            round.as_ref(), 
        ], 
        &TORIX_PROGRAM_ID
    ).0
}

pub fn derive_curve(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            CURVE_SEED.as_ref(),
            mint.as_ref(),
        ],
        &TORIX_PROGRAM_ID
    )
}

pub fn derive_mint_authority() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            MINT_AUTHORITY_SEED.as_ref(),
        ],
        &TORIX_PROGRAM_ID
    )
}

pub fn derive_curve_token_account(curve: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            curve.as_ref(),
            SPL_TOKEN_2022_PROGRAM_ID.as_ref(),
            mint.as_ref(),
        ],
        &SPL_ATA_PROGRAM_ID,
    ).0
}