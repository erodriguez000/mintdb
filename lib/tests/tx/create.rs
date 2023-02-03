use mintdb::{kvs::store::Datastore, prelude::*, util::verify::check_key::document_exists};
use serde_json::json;
use serde_json::Value;

#[tokio::test]
async fn tx_create_c() -> Result<()> {
    let db = Datastore::new().await?;
    let mut tx = db.transaction().await?;
    let tb = "tx_create_c";
    let doc = "tx_create_c:1";
    let key = "key_to_delete";
    let data = json!({"key": "value", key: "value"});
    if document_exists(tb, doc) {
        db.delete_document(tb, doc).await?;
    }
    let doc2 = "tx_create_c:2";
    let data2 = json!({"key": "value", key: "value"});
    {
        tx.begin();
        tx.create_document(tb, doc, data).await?;
        tx.merge(tb, doc2, data2).await?;
        tx.commit().await?;

        tx.begin();
        tx.delete_key(tb, doc, key).await?;
        tx.delete_key(tb, doc2, key).await?;
        tx.commit().await?;
    }
    let res = db.get_one(tb, doc).await?;
    assert_eq!(res[key], Value::Null);

    let res = db.get_one(tb, doc2).await?;
    assert_eq!(res[key], Value::Null);
    Ok(())
}
