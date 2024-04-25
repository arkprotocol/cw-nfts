// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Cw721Contract<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TMetadataExtension,
    // Message passed for updating metadata.
    TMetadataExtensionMsg,
    // Extension defined in CollectionInfo.
    TCollectionInfoExtension,
    TCollectionInfoExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    pub config: Cw721Config<
        'a,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >,
}

impl<
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Contract<
        'static,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
        }
    }
}
