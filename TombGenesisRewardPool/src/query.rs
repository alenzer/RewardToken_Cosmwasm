#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult,
    Uint128, QueryRequest, BankQuery,
    Coin, AllBalanceResponse,
};

use cw20::{ Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse };

use crate::msg::{QueryMsg, UserInfo, PoolInfo};
use crate::state::{operator, tomb, shiba, poolInfo, userInfo, totalAllocPoint, poolStartTime, poolEndTime};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {

        QueryMsg::GetOwner{ } => {
            let owner = operator.load(deps.storage).unwrap();
            to_binary(&owner)
        }

    }
}
