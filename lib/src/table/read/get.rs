#![allow(unused)]
use serde_json::{Value, json};
use crate::kvs::store::Datastore;
use crate::kvs::table::TableMetadata;
use crate::prelude::*;
impl Datastore {
    pub(crate) async fn get_tb_keys_auth(&self) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        let res = lock.tables.keys().cloned().collect::<Vec<String>>();
        Ok(json!(res))
    }
    pub(crate) async fn get_table_data_auth(&self, tb: &str) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        let res = lock.tables.get(tb).ok_or(Error::Request)?;
        Ok(json!(res))
    }
    pub(crate) async fn get_table_schema_auth(&self, tb: &str) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        let res = lock.tables.get(tb).ok_or(Error::TableNotFound(tb.into()))?;
        Ok(json!(res.schema))
    }
    pub(crate) async fn get_table_rules_auth(&self, tb: &str) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        let tbl = lock.tables.get(tb).ok_or(Error::TableNotFound(tb.into()))?;
        let res = json!({
            "read": tbl.read,
            "write": tbl.write,
            "delete": tbl.delete, 
        });
        Ok(res)
    }
    pub(crate) async fn get_tb_metadata_auth(&self, tb: &str) -> Result<TableMetadata> {
        let lock = self.collections.try_read().unwrap();
        let tbl_metadata: TableMetadata = lock.tables.get(tb).ok_or(Error::TableNotFound(tb.into()))?.to_owned().into();
        Ok(tbl_metadata)
    }
}