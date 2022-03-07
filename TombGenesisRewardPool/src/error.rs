use cosmwasm_std::StdError;
use thiserror::Error;
use cosmwasm_std::{Uint128};

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Late")]
    Late {},

    #[error("TombGenesisPool: existing pool?")]
    AlreadyExistingPool {},
}
