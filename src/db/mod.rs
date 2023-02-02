use std::sync::Arc;
use mintdb::Datastore;
use once_cell::sync::OnceCell;
use mintdb::prelude::*;

pub static DS: OnceCell<Arc<Datastore>> = OnceCell::new();

pub async fn init() -> Result<()> {
    let dbs = Datastore::new().await?;
    let _ = DS.set(Arc::new(dbs));
    Ok(())
}