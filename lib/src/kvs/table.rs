use std::collections::{BTreeMap, HashMap};

use serde::{Serialize, Deserialize};

use crate::auth::rule::Rule;

use super::document::Document;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Table {
    pub (crate) id: String,
    pub (crate) documents: BTreeMap<String, Document>,
    pub (crate) schema: HashMap<String, String>,
    pub (crate) read: Rule,
    pub (crate) write: Rule,
    pub (crate) delete: Rule,
}
impl Table {
    pub fn new(id: &str) -> Self {
        Table { id: id.to_string(), ..Default::default() }
    }
}
impl From<TableMetadata> for Table {
    fn from(meta: TableMetadata) -> Self {
        Table { id: meta.id,  schema: meta.schema, read: meta.read, write: meta.write, delete: meta.delete, ..Default::default()}
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TableMetadata {
    pub (crate) id: String,
    pub (crate) schema: HashMap<String, String>,
    pub (crate) read: Rule,
    pub (crate) write: Rule,
    pub (crate) delete: Rule
}
impl TableMetadata {
    pub fn new(id: &str) -> Self {
        TableMetadata { id: id.into(), ..Default::default() }
    }
}
impl From<Table> for TableMetadata {
    fn from(tb: Table) -> Self {
        TableMetadata { 
            id: tb.id, 
            schema: tb.schema, 
            read: tb.read, 
            write: tb.write, 
            delete: tb.delete 
        }
    }
}