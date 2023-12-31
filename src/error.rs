use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not whitelisted collection")]
    NotWhitelisted {},

    #[error("Unauthorized address")]
    Unauthorized {},

    #[error("Already unstaked")]
    AlreadyUnstaked {},

    #[error("Reward already claimed")]
    RewardAlreadyClaimed {},

    #[error("Wrong index")]
    WrongIndex {},

    #[error("Unknown")]
    Unknown {},

    #[error("Not enough reward pool")]
    NotEnoughRewardPool {},

    #[error("Not enough fee collected")]
    NotEnoughFeeCollected {},

    #[error("Not unstaked")]
    NotUnstaked {},

    #[error("Not enough unstake fee")]
    NotEnoughUnstakeFee {},
}
