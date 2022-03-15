#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult, Addr,
    Uint128
};

use crate::msg::{QueryMsg,};
use crate::state::{OPERATOR, TOMB, };


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsInitialized{ } => {
            
        },
        
    }
}
