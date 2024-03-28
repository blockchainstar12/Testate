use cosmwasm_std::{Uint128, Uint256};
use cw_storage_plus::Map;

pub const TOKEN_SUPPLY: Map<&str, Uint256> = Map::new("token_supply");

pub const MCAPW: Map<&str, Uint128> = Map::new("maximum_claimable_amount_per_wallet");

pub const CLAIMED_LIST: Map<&str, Uint128> = Map::new("claimed_list");