use mintdb::kvs::store::Datastore;
use mintdb::prelude::*;
use serde_json::json;

#[tokio::test]

async fn it_bfs_searches() -> Result<()> {
    let db = Datastore::new().await?;
    db.put("person", "person:1", "test_key", json!("value")).await?;
    db.put("person", "person:2", "test_key", json!("value")).await?;
    db.put("person", "person:3", "test_key", json!("value")).await?;
    db.put("person", "person:4", "test_key", json!("value")).await?;
    db.put("person", "person:5", "test_key", json!("value")).await?;


    db.add_node("person", "person:1", "person", "person:2", "like").await?;
    db.add_node("person", "person:1", "person", "person:4", "like").await?;

    db.add_node("person", "person:2", "person", "person:1", "like").await?;
    db.add_node("person", "person:2", "person", "person:4", "like").await?;
    db.add_node("person", "person:2", "person", "person:5", "like").await?;



    db.add_node("person", "person:3", "person", "person:1", "like").await?;
    db.add_node("person", "person:3", "person", "person:4", "like").await?;
    db.add_node("person", "person:3", "person", "person:2", "like").await?;

    db.add_node("person", "person:4", "person", "person:1", "like").await?;
    db.add_node("person", "person:4", "person", "person:5", "like").await?;

    db.add_node("person", "person:5", "person", "person:1", "like").await?;
    db.add_node("person", "person:5", "person", "person:3", "like").await?;
    
    let mut visited = vec![];
    let res = db.bfs("person", "person:1", "like", "person:5", &mut visited).await?;
    match res {
        Some(val) => {
            println!("Found target in: {val}");
            assert_eq!(val["id"], "person:2");
        }
        None => {
            return Err(Error::Request);
        }
    };
    Ok(())
}