use crate::error::ContractError;
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{TOKEN_SUPPLY, MCAPW, CLAIMED_LIST};
use cosmwasm_std::{
    entry_point, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError,
    Uint256,StdResult, Deps,BankMsg, Coin
};
use nibiru_std::proto::cosmos::{self, base};
use nibiru_std::proto::{nibiru, NibiruStargateMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let contract_addr: String = env.contract.address.into();
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.minter))?;

    let cosmos_msg: CosmosMsg = nibiru::tokenfactory::MsgCreateDenom {
        sender: contract_addr,
        subdenom:"utestate".to_string(),
    }
    .into_stargate_msg();

    let _ = MCAPW.save(deps.storage, "mcapw", &msg.mcapw);
    Ok(Response::new()
        // .add_event()
        .add_message(cosmos_msg))
    // Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let contract_addr: String = env.contract.address.into();
    match msg {
        // ExecuteMsg::CreateDenom { subdenom } => {
        //     let cosmos_msg: CosmosMsg = nibiru::tokenfactory::MsgCreateDenom {
        //         sender: contract_addr,
        //         subdenom,
        //     }
        //     .into_stargate_msg();

        //     Ok(Response::new()
        //         // .add_event()
        //         .add_message(cosmos_msg))
        // }

        ExecuteMsg::SetClaimableAmount {amount} => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            let _ = MCAPW.save(deps.storage, "mcapw", &amount);
            Ok(Response::new())
        }

        ExecuteMsg::Mint { amount, mint_to } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;

            let cosmos_msg: CosmosMsg = nibiru::tokenfactory::MsgMint {
                sender: contract_addr.clone(),
                // TODO feat: cosmwasm-std Coin should implement into()
                // base::v1beta1::Coin.
                coin: Some(cosmos::base::v1beta1::Coin {
                    denom: "tf/".to_owned()+&contract_addr+"/utestate",
                    amount: amount.into(),
                }),
                mint_to,
            }
            .into_stargate_msg();

            // let denom_parts: Vec<&str> = coin.denom.split('/').collect();
            // if denom_parts.len() != 3 {
            //     return Err(StdError::GenericErr {
            //         msg: "invalid denom input".to_string(),
            //     }
            //     .into());
            // }

            // let subdenom = denom_parts[2];
            // let supply_key = subdenom;
            let supply_key = "utestate";
            let token_supply =
                TOKEN_SUPPLY.may_load(deps.storage, supply_key)?;
            match token_supply {
                Some(supply) => {
                    let new_supply = supply + Uint256::from(amount);
                    TOKEN_SUPPLY.save(deps.storage, supply_key, &new_supply)
                }?,
                None => TOKEN_SUPPLY.save(
                    deps.storage,
                    supply_key,
                    &Uint256::from(amount),
                )?,
            }

            Ok(Response::new()
                // .add_event()
                .add_message(cosmos_msg))
        }

        ExecuteMsg::Burn { amount, burn_from } => {
            cw_ownable::assert_owner(deps.storage, &info.sender)?;
            let cosmos_msg: CosmosMsg = nibiru::tokenfactory::MsgBurn {
                sender: contract_addr.clone(),
                // TODO cosmwasm-std Coin should implement into()
                // base::v1beta1::Coin.
                coin: Some(base::v1beta1::Coin {
                    denom: "tf/".to_owned()+&contract_addr+"/utestate",
                    amount: amount.into(),
                }),
                burn_from,
            }
            .into_stargate_msg();
            Ok(Response::new()
                // .add_event()
                .add_message(cosmos_msg))
        }

        ExecuteMsg::Claim {amount} => {
            let claimable =
            MCAPW.may_load(deps.storage, "mcapw")?;
            match claimable {
                Some(supply) => {
                    if supply < amount.into() {
                        return Err(StdError::GenericErr {
                            msg: "Not claimable amount".to_string(),
                        }
                        .into());
                    }
                },
                None => {}
            }
            let claimed = CLAIMED_LIST.may_load(deps.storage, &info.sender.to_string())?;
            match claimed {
                Some(_claim) => {
                    return Err(StdError::GenericErr {
                        msg: "Claimed already".to_string(),
                    }
                    .into());
                }
                None => {
                    let _ = CLAIMED_LIST.save(deps.storage, &info.sender.to_string(), &amount);
                }
            }

            Ok(Response::new()
            .add_message(BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: vec![Coin{denom:"tf/".to_owned()+&contract_addr+"/utestate", amount}]
            }))
        }

        // ExecuteMsg::ChangeAdmin { denom, new_admin } => {
        //     let cosmos_msg: CosmosMsg = nibiru::tokenfactory::MsgChangeAdmin {
        //         sender: contract_addr,
        //         denom: denom.to_string(),
        //         new_admin: new_admin.to_string(),
        //     }
        //     .into_stargate_msg();
        //     Ok(Response::new()
        //         // .add_event()
        //         .add_message(cosmos_msg))
        // }
    }
}

// TODO
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Option<Uint256>> {
    // todo!()
    match msg {
        QueryMsg::TotalSupply {} => {
            let token_supply =
                TOKEN_SUPPLY.may_load(deps.storage, "utestate")?;
            Ok(token_supply)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::contract::instantiate;
    use crate::error::ContractError;
    use crate::msgs::InstantiateMsg;
    use cosmwasm_std::testing::{mock_env, mock_info};

    use cosmwasm_std as cw;
    use cosmwasm_std::DepsMut;
    use cw::testing::mock_dependencies;

    fn init(deps: DepsMut) -> Result<cw::Response, ContractError> {
        instantiate(deps, mock_env(), mock_info("none", &[]), InstantiateMsg {mcapw:5000000000, minter:"nibi1pl6r92ncwyqa6s3cdxjzprnsg5snn2mare34f0".to_string()})
    }

    #[test]
    fn init_runs() -> Result<(), ContractError> {
        let mut deps = mock_dependencies();
        let _env = mock_env();
        let _ = init(deps.as_mut())?;
        Ok(())
    }
}
