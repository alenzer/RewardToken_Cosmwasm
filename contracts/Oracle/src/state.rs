use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item};
use terraswap::asset::{AssetInfo};

pub const TOKEN0: Item<AssetInfo> = Item::new("token0");
pub const TOKEN1: Item<AssetInfo> = Item::new("token1");
pub const PAIR: Item<Addr> = Item::new("pair");
pub const BLOCKTIMESTAMP_LAST: Item<Uint128> = Item::new("blocktimestamp_last");
pub const PRICE0: Item<Uint128> = Item::new("price0");
pub const PRICE1: Item<Uint128> = Item::new("price1");