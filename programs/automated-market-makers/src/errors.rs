use anchor_lang::error_code;

#[error_code]
pub enum AmmError{
    #[msg("Invalid amount provided")]
    InvalidAmount,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Insufficient liquidity pool")]
    InvalidLiquidity,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Fee Exceeded")]
    FeeExceeded
}