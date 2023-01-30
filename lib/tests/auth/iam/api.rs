use mintdb::{prelude::*, kvs::store::Datastore};
#[tokio::test]
async fn db_creates_api_token() -> Result<()> {
    let username = "lucy@gmail.com";
    let password = "abc123";
    let db = Datastore::new().await?;
    db.sign_up(username, password).await?;
    let token = db.create_api_token(username).await?;
    println!("{token}");
    Ok(())
}