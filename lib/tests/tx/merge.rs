use mintdb::kvs::store::Datastore;
use mintdb::prelude::*;
use serde_json::json;

#[tokio::test]
async fn tx_merges() -> Result<()> {
    let tb = "tx_merge";
    let doc = "tx_merge:1";
    let data = json!({
        "name": "Lucy"
    });

    let db = Datastore::new().await?;
    let mut tx = db.transaction().await?;
    
    tx.begin();
    tx.merge(tb, doc, data).await?;
    tx.commit().await?;

    let data = json!({
        "city": "Miami"
    });
    tx.begin();
    tx.merge(tb, doc, data).await?;
    tx.commit().await?;

    let res = db.get_one(tb, doc).await?;
    assert_eq!(res["city"], "Miami");

    Ok(())
}