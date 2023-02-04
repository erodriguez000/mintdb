use std::{collections::BTreeMap, sync::Arc};
use crate::prelude::*;
use serde::{Serialize, Deserialize};
use std::sync::RwLock;
#[allow(unused)]
use crate::dev;
use crate::fs::ds::read;
use super::table::Table;


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Collection {
    pub tables: BTreeMap<String, Table>
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Datastore {
    pub collections: Arc<RwLock<Collection>>
}

impl Datastore {
    /// Creates a new Datastore instance
    /// # Examples
    /// ```
    /// use mintdb::Datastore;
    /// #[tokio::main]
    /// async fn main() -> Result<(), mintdb::Error> {
    ///     let db = Datastore::new().await?;
    ///     Ok(())
    /// }    
    /// 
    /// ```
    pub async fn new() -> Result<Self> {
        let db = Datastore::default();
        
        // dev::dev_seed(&mut db).await?;

        read::read_from_fs(&db).await?;
        
        Ok(db)
    }
}