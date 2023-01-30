use mintdb::kvs::store::Datastore;
use mintdb::prelude::*;
use serde_json::json;
#[tokio::test]
async fn it_dfs_searches() -> Result<()> {
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


    db.add_node("person", "person:3", "person", "person:1", "like").await?;
    db.add_node("person", "person:3", "person", "person:4", "like").await?;
    db.add_node("person", "person:3", "person", "person:2", "like").await?;

    db.add_node("person", "person:4", "person", "person:1", "like").await?;
    db.add_node("person", "person:4", "person", "person:5", "like").await?;

    db.add_node("person", "person:5", "person", "person:1", "like").await?;
    db.add_node("person", "person:5", "person", "person:3", "like").await?;

    
    let mut visited = vec![];
    // Will error on checking node, check add nodes, it needs to add an object
    match db.dfs("person", "person:1", "like", "person:3", &mut visited)? {
        Some(res) => {
            println!("res: {}", res);
            println!("Found target in: {res}");
            assert_eq!(res["id"], "person:5");
        }
        None => {
            println!("res: Not found");

        }
    }


    Ok(())
}