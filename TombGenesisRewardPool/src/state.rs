use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};
use crate::msg::{UserInfo, PoolInfo};
use std::collections::HashMap;

pub const operator: Item<Addr> = Item::new("operator");
pub const tomb: Item<Addr> = Item::new("tomb");
pub const shiba: Item<Addr> = Item::new("shiba");

// Info of each pool.
pub const poolInfo: Item<Vec<PoolInfo>> = Item::new("poolinfo");

// Info of each user that stakes LP tokens.
pub const userInfo: Map<Addr, HashMap<Addr, UserInfo>> = Map::new("userinfo");

// Total allocation points. Must be the sum of all allocation points in all pools.
pub const totalAllocPoint: Item<Uint128> = Item::new("totalAllocPoint");

// The time when TOMB mining starts.
pub const poolStartTime: Item<Uint128> = Item::new("poolStartTime");

// The time when TOMB mining ends.
pub const poolEndTime: Item<Uint128> = Item::new("poolEndTime");
