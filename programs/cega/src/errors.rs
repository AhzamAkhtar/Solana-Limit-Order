use anchor_lang::{error::Error, error_code};

#[error_code]
pub enum CustomErrors {
    #[msg("Zero balance.")]
    ZeroBalance,
    #[msg("Time has expired")]
    TimeHasExpired,
    #[msg("Cant Close, Partial Trade Already Happend")]
    CannotClose,
}
