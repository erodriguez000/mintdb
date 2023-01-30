use mintdb::kvs::store::Datastore;
use mintdb::prelude::*;
use serde_json::json;

#[tokio::test]
async fn db_reads_from_fs() -> Result<()> {
    let tb = "db_fs_read";
    let doc = "db_fs_read:1";
    let data = json!({
        "key": "Value"
    });
    let doc2 = "db_fs_read:2";
    let data2 = json!({
        "key": "Value2"
    });
    {
        let db = Datastore::new().await?;
        db.merge(tb, doc, data).await?;

        let db = Datastore::new().await?;
        db.merge(tb, doc2, data2).await?;
    }
    let db = Datastore::new().await?;

    let res = db.get_one(tb, doc).await?;
    assert_eq!(res["key"], "Value");

    let res = db.get_one(tb, doc2).await?;
    assert_eq!(res["key"], "Value2");
    Ok(())
}

#[tokio::test]
async fn db_deletes_document_in_fs() -> Result<()> {
    let tb = "db_fs_delete";
    let doc = "db_fs_delete:1";
    let data = json!({
        "key": "value"
    });
    let db = Datastore::new().await?;
    db.merge(tb, doc, data).await?;

    let path = "mint.db/ds/db_fs_delete/db_fs_delete:1.bin";
    let res = std::fs::metadata(path);
    assert!(res.is_ok());

    db.delete_document(tb, doc).await?;

    let res = std::fs::metadata(path);
    assert!(res.is_err());

    Ok(())
}
#[tokio::test]
async fn db_deletes_table_in_fs() -> Result<()> {
    let tb = "db_fs_delete_tb";
    let doc = "db_fs_delete_tb:1";
    let data = json!({
        "key": "value"
    });
    let db = Datastore::new().await?;
    db.merge(tb, doc, data).await?;

    let path = "mint.db/ds/db_fs_delete_tb";
    let res = std::fs::metadata(path);
    assert!(res.is_ok());

    db.delete_table(tb).await?;

    let res = std::fs::metadata(path);
    assert!(res.is_err());

    Ok(())
}