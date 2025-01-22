use anchor_lang::error_code;

#[error_code]
pub enum RoundError {
    #[msg("Round: Not allowed owner")]
    NotAllowedOwner,

    #[msg("Round: Invalid Round Index")]
    InvalidRoundIndex,

    #[msg("Round: Over max slot count")]
    OverMaxSlot,

    #[msg("Round: Already buy slot")]
    AlreadyBuySlot,

    #[msg("Round: Already finished")]
    AlreadyFinish,

    #[msg("Round: Already claim")]
    AlreadyClaim,

    #[msg("Round: Current round is processing now")]
    Processing,

    #[msg("Round: The account is not initialized")]
    UninitializedAccount,

    #[msg("Round: Fee is over the max fee")]
    MaxFeeError,

    #[msg("Round: Amount should be great than zero")]
    ZeroAmount,

    #[msg("Round: The reference address is not correct")]
    InvalidReference
}