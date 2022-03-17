#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, DepsMut, Env, MessageInfo, Response, QuerierWrapper, Uint128, 
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{TOKEN0, TOKEN1, PAIR, PRICE0, PRICE1};
use terraswap::asset::{AssetInfo, Asset};
use terraswap::pair::{QueryMsg as PairQueryMsg, SimulationResponse, PoolResponse};
use terraswap::querier::{simulate, query_pair_info};

// version info for migration info
const CONTRACT_NAME: &str = "Oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


pub fn get_price(querier: &QuerierWrapper, _pair: Addr, asset_info: &AssetInfo) -> Uint128 {
    let offer_asset = Asset{
        info: asset_info.clone(),
        amount: Uint128::from(1u128)
    };
    let sim_res: SimulationResponse = simulate( querier, _pair, &offer_asset ).unwrap();

    sim_res.return_amount
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let pair = msg.pair;
    PAIR.save(deps.storage, &pair)?;

    let pair_info: PoolResponse = deps.querier.query_wasm_smart(
        pair.clone(),
        &PairQueryMsg::Pool{}
    ).unwrap();

    let token0 = pair_info.assets[0].info.clone();
    let token1 = pair_info.assets[0].info.clone();
    TOKEN0.save( deps.storage, &token0)?;
    TOKEN1.save( deps.storage, &token1)?;

    PRICE0.save(deps.storage, &get_price(&deps.querier, pair.clone(), &token0))?;
    PRICE1.save(deps.storage, &get_price(&deps.querier, pair, &token1))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Update {  } => try_update(deps),
    }
}

pub fn try_update(deps:DepsMut) 
    -> Result<Response, ContractError>
{
    let token0 = TOKEN0.load(deps.storage)?;
    let token1 = TOKEN1.load(deps.storage)?;
    let pair = PAIR.load(deps.storage)?;

    TOKEN0.save( deps.storage, &token0)?;
    TOKEN1.save( deps.storage, &token1)?;

    PRICE0.save(deps.storage, &get_price(&deps.querier, pair.clone(), &token0))?;
    PRICE1.save(deps.storage, &get_price(&deps.querier, pair, &token1))?;

    Ok(Response::new())
}