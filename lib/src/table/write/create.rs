#![allow(unused)]
use serde_json::{Value, json};
use crate::auth::schema;
use crate::kvs::store::Datastore;
use crate::kvs::table::Table;
use crate::prelude::*;
impl Datastore {
    pub(crate) async fn create_table_auth(&self, tb: &str) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if lock.tables.contains_key(tb) {
            return Err(Error::TableExists(tb.into()));
        }
        let tbl = Table::new(tb);
        lock.tables.insert(tb.into(), tbl);
        Ok(json!(f!("Table '{tb}' created")))
    }
    pub(crate) async fn create_tb_with_schema_auth(&self, tb: &str, schema: Value) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if lock.tables.contains_key(tb) {
            return Err(Error::TableExists(tb.into()))
        }
        let mut tbl = Table::new(tb);
        schema::create_schema(&mut tbl, &schema)?;
        lock.tables.insert(tb.into(), tbl);
        Err(Error::Request)
    }
}