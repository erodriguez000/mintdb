use mintdb::kvs::store::Datastore;
use mintdb::prelude::*;
use serde_json::json;

#[tokio::test]
async fn db_reads_document() -> Result<()> {
    let db = Datastore::new().await?;
    let tb = "db_read";
    let doc = "db_read:1";
    let data = json!({
        "key": "Value"
    });
    db.merge(tb, doc, data).await?;

    let res = db.get_one(tb, doc).await?;

    assert_eq!(res["key"], "Value");
    Ok(())
}

#[tokio::test]
async fn db_compares() -> Result<()> {
    let db = Datastore::new().await?;
    let tb = "test_compare_car";
    let doc = "test_compare_car:1";
    let data = json!({
        "make": "Mercedes-Benz",
        "model": "G-Wagon",
        "year": 2024
    });
    db.merge(tb, doc, data).await?;
    let doc = "test_compare_car:2";
    let data = json!({
        "make": "Mercedes-Benz",
        "model": "S63 AMG",
        "year": 2024
    });
    db.merge(tb, doc, data).await?;
    let doc = "test_compare_car:3";
    let data = json!({
        "make": "Suzuki",
        "model": "Samauri",
        "year": 1995
    });
    db.merge(tb, doc, data).await?;

    let lhs = "make";
    let op = "==";
    let rhs = json!("Mercedes-Benz");
    let res = db.compare(tb, lhs, op, &rhs).await?.as_array().ok_or(Error::Request)?.to_owned();
    assert!(res.len() == 2);

    let lhs = "make";
    let op = "!=";
    let rhs = json!("Mercedes-Benz");
    let res = db.compare(tb, lhs, op, &rhs).await?
    .as_array().ok_or(Error::Request)?.to_owned();
    assert!(res.len() == 1);

    let lhs = "make";
    let op = "contains";
    let rhs = json!("Benz");
    let res = db.compare(tb, lhs, op, &rhs).await?
    .as_array().ok_or(Error::Request)?.to_owned();
    assert!(res.len() == 2);


    let lhs = "make";
    let op = "icontains";
    let rhs = json!("benz");
    let res = db.compare(tb, lhs, op, &rhs).await?
    .as_array().ok_or(Error::Request)?.to_owned();
    assert!(res.len() == 2);


    Ok(())
}