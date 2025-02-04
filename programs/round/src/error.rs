use anchor_lang::error_code;

#[error_code]
pub enum RoundError {
    #[msg("Round: Not allowed owner")]
    NotAllowedOwner,

    #[msg("Round: Invalid Round Index")]
    InvalidRoundIndex,

    #[msg("Round: user should buy at least 4 slots to be enalbe the chad mod")]
    NotEnoughAmountForChadMod,

    #[msg("Round: The round was not finished yet.")]
    RoundNotFinished,

    #[msg("Round: Over max slot count")]
    OverMaxSlot,

    #[msg("Round: The amount is small to buy with chad mod. It should be 4 at least")]
    SmallAmount,

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
    InvalidReference, 

    #[msg("Round: The user can't claim if chad mod is enabled")]
    UnAbleToClaim
}