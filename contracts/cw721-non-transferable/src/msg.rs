use cosmwasm_schema::cw_serde;
use cw721::{
    msg::Cw721QueryMsg,
    state::{DefaultOptionCollectionMetadataExtension, DefaultOptionNftMetadataExtension},
};

#[cw_serde]
pub struct InstantiateMsg<TCollectionMetadataExtension> {
    pub admin: Option<String>,
    /// Name of the collection metadata
    pub name: String,
    /// Symbol of the collection metadata
    pub symbol: String,
    /// Optional extension of the collection metadata
    pub collection_metadata_extension: TCollectionMetadataExtension,
    pub minter: Option<String>,
    pub creator: Option<String>,
    pub withdraw_address: Option<String>,
}

#[cw_serde]
pub enum QueryMsg {
    Admin {},

    // -- below copied from Cw721QueryMsg
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    #[deprecated(since = "0.19.0", note = "Please use GetCollectionMetadata instead")]
    /// Deprecated: use GetCollectionMetadata instead! Will be removed in next release!
    ContractInfo {},

    GetCollectionMetadata {},

    #[deprecated(since = "0.19.0", note = "Please use GetMinterOwnership instead")]
    /// Deprecated: use GetMinterOwnership instead! Will be removed in next release!
    Minter {},

    GetMinterOwnership {},

    GetCreatorOwnership {},

    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    GetWithdrawAddress {},
}

impl From<QueryMsg>
    for Cw721QueryMsg<DefaultOptionNftMetadataExtension, DefaultOptionCollectionMetadataExtension>
{
    fn from(
        msg: QueryMsg,
    ) -> Cw721QueryMsg<DefaultOptionNftMetadataExtension, DefaultOptionCollectionMetadataExtension>
    {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            #[allow(deprecated)]
            QueryMsg::ContractInfo {} => Cw721QueryMsg::GetCollectionMetadata {},
            QueryMsg::GetCollectionMetadata {} => Cw721QueryMsg::GetCollectionMetadata {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            #[allow(deprecated)]
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            QueryMsg::GetMinterOwnership {} => Cw721QueryMsg::GetMinterOwnership {},
            QueryMsg::GetCreatorOwnership {} => Cw721QueryMsg::GetCreatorOwnership {},
            QueryMsg::GetWithdrawAddress {} => Cw721QueryMsg::GetWithdrawAddress {},
            QueryMsg::AllOperators { .. } => unreachable!("AllOperators is not supported!"),
            QueryMsg::Approval { .. } => unreachable!("Approval is not supported!"),
            QueryMsg::Approvals { .. } => unreachable!("Approvals is not supported!"),
            QueryMsg::Admin { .. } => unreachable!("Approvals is not supported!"),
        }
    }
}

#[cw_serde]
pub struct AdminResponse {
    pub admin: Option<String>,
}
