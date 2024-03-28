use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128};

#[cw_serde]
pub struct InstantiateMsg {pub mcapw: Uint128, pub minter:String}

#[cw_serde]
pub enum ExecuteMsg {
    // For x/tokenfactory
    // CreateDenom { subdenom: String },
    Mint { amount:Uint128, mint_to: String },
    Burn { amount:Uint128, burn_from: String },
    // ChangeAdmin { denom: String, new_admin: String },
    Claim {amount:Uint128},
    SetClaimableAmount {amount:Uint128},
}

// #[cw_serde]
// pub enum QueryMsg {
//     TotalSupply {

//     },
// }
