use mintdb::kvs::store::Datastore;
use mintdb::prelude::*;
use serde_json::json;
#[tokio::test]
async fn it_adds_nodes() -> Result<()> {
    let db = Datastore::new().await?;
    db.put("person", "person:1", "test_key", json!("value")).await?;
    db.put("person", "person:2", "test_key", json!("value")).await?;
    db.put("person", "person:3", "test_key", json!("value")).await?;

    db.add_node("person", "person:1", "person", "person:2", "like").await?;
    db.add_node("person", "person:2", "person", "person:3", "like").await?;
    db.add_node("person", "person:3", "person", "person:1", "like").await?;

    let res = db.get_one("person", "person:1").await?;
    let res = res.as_object().unwrap();
    assert!(res.contains_key("rel:like"));

    let res = db.get_one("person", "person:2").await?;
    let res = res.as_object().unwrap();
    assert!(res.contains_key("rel:like"));

    let res = db.get_one("person", "person:3").await?;
    let res = res.as_object().unwrap();
    assert!(res.contains_key("rel:like"));

    Ok(())
}
