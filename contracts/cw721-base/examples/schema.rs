use cosmwasm_schema::{export_schema_with_title, remove_schemas, schema_for};
use cosmwasm_std::Empty;
use cw721::state::{DefaultOptionCollectionInfoExtension, DefaultOptionMetadataExtension};
use cw721_base::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    // entry points - generate always with title for avoiding name suffixes like "..._empty_for_..." due to generics
    export_schema_with_title(
        &schema_for!(InstantiateMsg<DefaultOptionCollectionInfoExtension>),
        &out_dir,
        "InstantiateMsg",
    );
    export_schema_with_title(
        &schema_for!(
            ExecuteMsg::<DefaultOptionMetadataExtension, Empty, DefaultOptionCollectionInfoExtension>
        ),
        &out_dir,
        "ExecuteMsg",
    );
    export_schema_with_title(
        &schema_for!(QueryMsg<DefaultOptionMetadataExtension, DefaultOptionCollectionInfoExtension>),
        &out_dir,
        "QueryMsg",
    );
    export_schema_with_title(&schema_for!(MigrateMsg), &out_dir, "MigrateMsg");
}
