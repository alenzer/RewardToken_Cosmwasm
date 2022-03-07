#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, to_binary, DepsMut, Env, MessageInfo, Response,
    Uint128, CosmosMsg, WasmMsg, Storage
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, UserInfo, PoolInfo};
use crate::state::{operator, tomb, shiba, poolInfo, userInfo, totalAllocPoint, poolStartTime, poolEndTime};

// version info for migration info
const CONTRACT_NAME: &str = "TombGenesisRewardPool";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const TOMB_PER_SECOND: u128 = 96_450_000_000_000_000; //0.09645 ether; // 25000 TOMB / (72h * 60min * 60s);
const RUNNING_TIME: u128 = 259_200; //3 days;
const TOTAL_REWARDS: u128 = 25_000_000_000_000_000_000_000; //25000 ether;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.poolStartTime < Uint128::from(env.block.time.seconds()){
        return Err(ContractError::Late{})
    }
    
    tomb.save(deps.storage, &deps.api.addr_validate(msg.tomb.as_str())?)?;
    shiba.save(deps.storage, &deps.api.addr_validate(msg.shiba.as_str())?)?;
    poolStartTime.save(deps.storage, &msg.poolStartTime)?;

    let pool_end_time: Uint128 = msg.poolStartTime + Uint128::from(RUNNING_TIME);
    poolEndTime.save(deps.storage, &pool_end_time)?;

    operator.save(deps.storage, &info.sender)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

fn balance_of(deps: &DepsMut, _token: &Addr, _address: &Addr) -> u128 {
    let token_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        _token,
        &Cw20QueryMsg::Balance{
            address: _address.to_string(),
        }
    ).unwrap();
    token_balance.balance.u128()
}
fn check_pool_duplicate(deps: &DepsMut, _token: Addr) -> bool {
    let pool_info: Vec<PoolInfo> = poolInfo.load(deps.storage).unwrap();
    let length = pool_info.len();
    for pid in 0 .. length-1  {
        if pool_info[pid].token == _token{
            return true;
        }
    }
    false
}
fn mass_update_pools(deps: DepsMut, env: Env) {
    let pool_info: Vec<PoolInfo> = poolInfo.load(deps.storage).unwrap();
    let length = pool_info.len();

    let mut _deps = deps;
    for pid in 0 .. length-1 {
        update_pool(_deps.branch(), &env, pid);
    }
}
fn update_pool(deps: DepsMut, env: &Env, _pid: usize) {
    let pool_info: Vec<PoolInfo> = poolInfo.load(deps.storage).unwrap();
    let mut pool = pool_info[_pid].clone();
    if Uint128::from(env.block.time.seconds()) <= pool.lastRewardTime {
        return;
    }

    let token_supply: u128 = balance_of(&deps, &pool.token, &env.contract.address);
    if token_supply == 0 {
        pool.lastRewardTime = Uint128::from(env.block.time.seconds());
        return;
    }

    let mut total_alloc_point = totalAllocPoint.load(deps.storage).unwrap();
    if !pool.isStarted {
        pool.isStarted = true;
        total_alloc_point = total_alloc_point + pool.allocPoint;
        totalAllocPoint.save(deps.storage, &total_alloc_point).unwrap();
    }
    if total_alloc_point > Uint128::zero() {
        // uint256 _generatedReward = get_generated_reward(pool.lastRewardTime, block.timestamp);
        // uint256 _tombReward = _generatedReward.mul(pool.allocPoint).div(totalAllocPoint);
        // pool.accTombPerShare = pool.accTombPerShare.add(_tombReward.mul(1e18).div(token_supply));
    }
    pool.lastRewardTime = Uint128::from(env.block.time.seconds());
}
fn get_generated_reward(deps: DepsMut, from_time: Uint128, to_time: Uint128) -> u128 {
    if from_time >= to_time {
        return 0;
    }

    let mut pool_end_time = poolEndTime.load(deps.storage).unwrap();
    let pool_start_time = poolStartTime.load(deps.storage).unwrap();

    if to_time >= pool_end_time {
        if from_time >= pool_end_time{ 
            return 0;
        }
        if from_time <= pool_start_time {
            pool_end_time = (pool_end_time - pool_start_time) * Uint128::from(TOMB_PER_SECOND);
        }
        else {
            pool_end_time = (pool_end_time - from_time) * Uint128::from(TOMB_PER_SECOND);
        }

        poolEndTime.save(deps.storage, &pool_end_time).unwrap();
        return pool_end_time.u128();
    } else {
        if to_time <= pool_start_time { 
            return 0;
        }

        if from_time <= pool_start_time {
          return (to_time - pool_start_time).u128() * TOMB_PER_SECOND;
        }

        return (to_time - from_time).u128() * TOMB_PER_SECOND;
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Add{ alloc_point, token, with_update, last_reward_time}
            => try_add(deps, env, info, alloc_point, token, with_update, last_reward_time ),

        ExecuteMsg::Set{ pid, alloc_point}
            => try_set(deps, env, info, pic, alloc_point ),
    }
}

pub fn try_set(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pid: Uint128,
    alloc_point: Uint128
)
    -> Result<Response, ContractError>
{

    Ok(Response::new()
        .add_attribute("action", "set"))
}
pub fn try_add(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    alloc_point: Uint128,
    token: Addr,
    with_update: bool,
    last_reward_time: Uint128
) 
    -> Result<Response, ContractError>
{
    if check_pool_duplicate(&deps, token.clone()) == true {
        return Err(ContractError::AlreadyExistingPool {})
    }

    let mut _deps = deps; 
    if with_update == true {
        mass_update_pools(_deps.branch(), env.clone());
    }

    let pool_start_time = poolStartTime.load(_deps.storage)?;
    let mut _last_reward_time = last_reward_time;
    let blocktime = Uint128::from(env.block.time.seconds());

    if blocktime < pool_start_time {
        // chef is sleeping
        if last_reward_time == Uint128::zero() {
            _last_reward_time = pool_start_time;
        } else {
            if last_reward_time < pool_start_time {
                _last_reward_time = pool_start_time;
            }
        }
    } else {
        // chef is cooking
        if last_reward_time == Uint128::zero() || last_reward_time < blocktime {
            _last_reward_time = blocktime;
        }
    }

    let is_started: bool =
            (last_reward_time <= pool_start_time) ||
            (last_reward_time <= blocktime);

    let mut pool_info = poolInfo.load(_deps.storage)?;
    pool_info.push(PoolInfo{
        token : token,
        allocPoint : alloc_point,
        lastRewardTime : _last_reward_time,
        accTombPerShare : Uint128::zero(),
        isStarted : is_started
        });

    if is_started == true {
        let mut total_alloc_point = totalAllocPoint.load(_deps.storage)?;
        total_alloc_point = total_alloc_point + alloc_point;
        totalAllocPoint.save(_deps.storage, &total_alloc_point)?;
    }

    Ok(Response::new()
        .add_attribute("action", "add"))                                
}

