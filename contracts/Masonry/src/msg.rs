use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub TOMB: String,
    pub POOLSTARTTIME: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Add {
        alloc_point: Uint128,
        token: Addr,
        with_update: bool,
        last_reward_time: Uint128
    },
    Set {
        pid: Uint128,
        alloc_point: Uint128,
    },
    MassUpdatePools{},
    UpdatePool{
        pid: Uint128
    },
    Deposit{
        pid: Uint128,
        amount: Uint128
    },
    Withdraw{
        pid: Uint128,
        amount: Uint128
    },
    EmergencyWithdraw{
        pid: Uint128
    },
    SetOperator{
        operator: Addr
    },
    GovernanceRecoverUnsupported{
        token: Addr,
        amount: Uint128,
        to: Addr
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner{ },
    GetGeneratedReward{
        from_time: Uint128,
        to_time: Uint128
    },
    PendingTomb{
        pid: Uint128,
        user: Addr
    },
    GetPoolInfo{ },
    GetUserInfo{ 
        pid: Uint128,
        user: Addr
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Masonseat {
    pub last_snapshot_index: Uint128,
    pub reward_earned: Uint128,
    pub epoch_timer_start: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MasonrySnapshot {
    pub time: Uint128,
    pub reward_received: Uint128,
    pub reward_per_share: Uint128
}