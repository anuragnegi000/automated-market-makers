use anchor_lang::error_code;
use constant_product_curve::CurveError;


#[error_code]
pub enum AmmError{
    #[msg("Invalid amount provided")]
    InvalidAmount,
    #[msg("Slippage tollerance exceeded")]
    SlippageExceeded,
    #[msg("Insufficient liquidity pool")]
    InvalidLiquidity,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Fee Exceeded")]
    FeeExceeded
}

