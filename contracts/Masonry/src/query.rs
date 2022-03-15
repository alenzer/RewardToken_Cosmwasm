#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult, Addr,
    Uint128
};

use crate::msg::{QueryMsg, Masonseat};
use crate::state::{OPERATOR, TOMB, SHARE, TOTALSUPPLY, INITIALIZED, BALANCES,
    TREASURY, MASONS, MASONRY_HISTORY, WITHDRAW_LOCKUP_EPOCHS, REWARD_LOCKUP_EPOCHS};
use crate::contract::{get_latest_snapshot, get_last_snapshot_of, balance_of};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::LatestSnapshotIndex{ } => {
            let masonry_history = MASONRY_HISTORY.load(deps.storage)?;
            to_binary(&(masonry_history.len() -1))
        },
        QueryMsg::GetLastSnapshotIndexOf{ mason } => {
            let _mason = MASONS.load(deps.storage, mason)?;
            to_binary(&_mason.last_snapshot_index)
        },
        QueryMsg::CanWithdraw{ mason } => {
            let _mason = MASONS.load(deps.storage, mason)?;
            let withdraw_lockup_epochs = WITHDRAW_LOCKUP_EPOCHS.load(deps.storage)?;
            let treasury = TREASURY.load(deps.storage)?;

            // let epoch = deps.querier.query_wasm_smart(treasury, msg: )
            // let res = (_mason.epochTimerStart + withdrawLockupEpochs) <= treasury.epoch();
            to_binary(&Uint128::zero())
        },
        QueryMsg::CanClaimReward{ mason: Addr } => {
            to_binary(&Uint128::zero())
        },
        QueryMsg::Epoch{ } => {
            to_binary(&Uint128::zero())
        },
        QueryMsg::NextEpochPoint{ } => {
            to_binary(&Uint128::zero())
        },
        QueryMsg::GetTombPrice{ } => {
            to_binary(&Uint128::zero())
        },
        QueryMsg::RewardPerShare{ } => {
            get_latest_snapshot().reward_per_share
        },
        QueryMsg::Earned{ mason } => {
            let latest_rps = get_latest_snapshot().reward_per_share;
            let stored_rps = get_last_snapshot_of(mason).reward_per_share;
            let balance = balance_of(deps.storage, mason);
            let mason = MASONS.load(deps.storage, mason).unwrap();
            return balance * (latest_rps-stored_rps) / ((1u64).pow(18u64)) + mason.reward_earned;
        }
    }
}
