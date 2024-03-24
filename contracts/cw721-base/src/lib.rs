pub mod error;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

pub use crate::state::Cw721Contract;
pub use cw721::*;

// These types are re-exported so that contracts interacting with this
// one don't need a direct dependency on cw_ownable to use the API.
//
// `Action` is used in `Cw721ExecuteMsg::UpdateMinterOwnership` and `Cw721ExecuteMsg::UpdateCreatorOwnership`, `Ownership` is
// used in `Cw721QueryMsg::GetMinterOwnership`, `Cw721QueryMsg::GetCreatorOwnership`, and `OwnershipError` is used in
// `Cw721ContractError::Ownership`.
pub use cw_ownable::{Action, Ownership, OwnershipError};

use cosmwasm_std::Empty;

// Version info for migration
pub const CONTRACT_NAME: &str = "crates.io:cw721-base";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[deprecated(
    since = "0.19.0",
    note = "Please use `DefaultOptionNftExtension` instead"
)]
pub type Extension = DefaultOptionalNftExtension;

pub mod entry {

    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
    use cw721::{
        error::Cw721ContractError,
        msg::{Cw721ExecuteMsg, Cw721InstantiateMsg, Cw721MigrateMsg, Cw721QueryMsg},
        traits::{Cw721Execute, Cw721Query},
        DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg,
        DefaultOptionalNftExtension, DefaultOptionalNftExtensionMsg,
    };

    // This makes a conscious choice on the various generics used by the contract
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721InstantiateMsg<DefaultOptionalCollectionExtensionMsg>,
    ) -> Result<Response, Cw721ContractError> {
        let contract = Cw721Contract::<
            DefaultOptionalNftExtension,
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtension,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
            Empty,
            Empty,
        >::default();
        contract.instantiate_with_version(deps, &env, &info, msg, CONTRACT_NAME, CONTRACT_VERSION)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
        >,
    ) -> Result<Response, Cw721ContractError> {
        let contract = Cw721Contract::<
            DefaultOptionalNftExtension,
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtension,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
            Empty,
            Empty,
        >::default();
        contract.execute(deps, &env, &info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(
        deps: Deps,
        env: Env,
        msg: Cw721QueryMsg<DefaultOptionalNftExtension, DefaultOptionalCollectionExtension, Empty>,
    ) -> Result<Binary, Cw721ContractError> {
        let contract = Cw721Contract::<
            DefaultOptionalNftExtension,
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtension,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
            Empty,
            Empty,
        >::default();
        contract.query(deps, &env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(
        deps: DepsMut,
        env: Env,
        msg: Cw721MigrateMsg,
    ) -> Result<Response, Cw721ContractError> {
        let contract = Cw721Contract::<
            DefaultOptionalNftExtension,
            DefaultOptionalNftExtensionMsg,
            DefaultOptionalCollectionExtension,
            DefaultOptionalCollectionExtensionMsg,
            Empty,
            Empty,
            Empty,
        >::default();
        contract.migrate(deps, env, msg, CONTRACT_NAME, CONTRACT_VERSION)
    }
}
