use mintdb::kvs::store::Datastore;
use mintdb::prelude::*;
use serde_json::json;
#[tokio::test]
async fn tx_is_atomic() -> Result<()> {
    let tb = "tx_acct";
    let doc = "tx_acct:1";
    let key = "balance";
    let balance = 1000.00;
    let data = json!({"balance": balance});
    
    let doc2 = "tx_acct:2";
    let balance2 = 500.00;
    let data2 = json!({"balance": balance2});
    
    let amt = 700.00;
    let db = Datastore::new().await?;
    db.put_document(tb, doc, data).await?;
    db.put_document(tb, doc2, data2).await?;

    let mut tx = db.transaction().await?;
    tx.begin();
    tx.credit(tb, doc, key, amt).await?;
    let r = tx.debit(tb, doc2, key, amt).await;
    assert!(r.is_err());
    let r = tx.commit().await;
    assert!(r.is_err());

    let res = db.get_one(tb, doc).await?;
    assert_eq!(res["balance"], balance);

    let res = db.get_one(tb, doc2).await?;
    assert_eq!(res["balance"], balance2);


    Ok(())
}

#[tokio::test]
async fn tx_credits_and_debits() -> Result<()> {
    let tb = "tx_acct";
    let doc = "tx_acct:1";
    let key = "balance";
    let balance = 1000.00;
    let data = json!({"balance": balance});
    
    let doc2 = "tx_acct:2";
    let balance2 = 500.00;
    let data2 = json!({"balance": balance2});

    let amt = 1000.00;
    let db = Datastore::new().await?;
    db.put_document(tb, doc, data).await?;
    db.put_document(tb, doc2, data2).await?;

    let mut tx = db.transaction().await?;
    tx.begin();
    tx.debit(tb, doc, key, amt).await?;
    tx.credit(tb, doc2, key, amt).await?;
    tx.commit().await?;

    let res = db.get_one(tb, doc).await?;
    assert_eq!(res["balance"], (balance - amt));

    let res = db.get_one(tb, doc2).await?;
    assert_eq!(res["balance"], (balance2 + amt));
    Ok(())
}