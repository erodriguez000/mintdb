#![allow(unused)]
use serde_json::Value;
use serde_json::json;
use crate::kvs::document;
use crate::kvs::store::Datastore;
use crate::kvs::table::TableMetadata;
use crate::fs::ds;
use crate::prelude::*;
use super::rule;
use super::schema;
pub async fn validate_read_request(tb: &str, ctx: &Value) -> Result<()> {
    let tb_metadata = ds::read::read_tb_metadata(tb).await?;
    if !tb_metadata.read.is_empty() {
        rule::read::validate_read(&tb_metadata, ctx)?;
    }
    Ok(())
}
pub async fn validate_create_request(tb: &str, doc: &str, data: &Value, ctx: &Value) -> Result<()> {
 let tb_metadata = ds::read::read_tb_metadata(tb).await?;
 if !tb_metadata.write.is_empty(){
    let document = ds::read::read_document(tb, doc).await?;
    rule::write::validate_document_write(&tb_metadata, &json!(document.data), ctx)?;
    rule::write::validate_write(&tb_metadata, ctx)?;
 }
 if !tb_metadata.schema.is_empty() {
     schema::validate_schema_create(&tb_metadata, data)?;
 }
 Ok(())
}
pub async fn validate_merge_request(tb: &str, doc: &str, data: &Value, ctx: &Value) -> Result<()> {
    let tb_metadata = ds::read::read_tb_metadata(tb).await?;
    if !tb_metadata.write.is_empty() {
        let document = ds::read::read_document(tb, doc).await?;
        rule::write::validate_write(&tb_metadata, ctx)?;
        rule::write::validate_document_write(&tb_metadata, &json!(document.data), ctx)?;
    }
    if !tb_metadata.schema.is_empty() {
        schema::validate_schema_merge(&tb_metadata, data)?;
    }
    Ok(())
}
pub async fn validate_put_request(tb: &str, doc: &str, key: &str, value: &Value, ctx: &Value) -> Result<()> {
    let tb_metadata = ds::read::read_tb_metadata(tb).await?;
    if !tb_metadata.write.is_empty() {
        let document = ds::read::read_document(tb, doc).await?;
        rule::write::validate_document_write(&tb_metadata, &json!(document.data), ctx)?;
        rule::write::validate_write(&tb_metadata, ctx)?;
    }
    if !tb_metadata.schema.is_empty() {
        schema::validate_schema_key(&tb_metadata, key, &value)?;
    }
    Ok(())
}
pub async fn validate_delete_request(tb: &str, doc: &str, ctx: &Value) -> Result<()> {
    let tb_metadata = ds::read::read_tb_metadata(tb).await?;
    if !tb_metadata.delete.is_empty() {
        let document = ds::read::read_document(tb, doc).await?;
        rule::delete::validate_document_delete(&tb_metadata, &json!(document.data), ctx)?;
        rule::delete::validate_delete(&tb_metadata, ctx)?;
    }
    Ok(())
}