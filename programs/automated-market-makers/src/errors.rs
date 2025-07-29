use anchor_lang::error_code;
use constant_product_curve::CurveError;

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
    FeeExceeded,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::SlippageLimitExceeded => AmmError::SlippageExceeded,
            CurveError::InsufficientBalance => AmmError::InsufficientLiquidity,
            _ => AmmError::InvalidAmount,
        }
    }
}