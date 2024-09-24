use anchor_lang::{error::Error, error_code};

#[error_code]
pub enum CustomErrors {
    #[msg("Amount should be greater than Zero")]
    ZeroAmount,
    #[msg("Time has expired")]
    TimeHasExpired,
    #[msg("Cant Close, Partial Trade Already Happend")]
    CannotClose,
}
