use serde_json::{Value, json};
use crate::auth::schema;
use crate::kvs::store::Datastore;
use crate::prelude::*;

impl Datastore {
    pub async fn add_schema_auth(&self, tb: &str, schema: &Value) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            schema::create_schema(tbl, schema)?;
            return Ok(json!(schema))
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn delete_schema_auth(&self, tb: &str) -> Result<()> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            tbl.schema.clear();
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
        Ok(())
    }
}