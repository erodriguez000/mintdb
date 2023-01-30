use mintdb::kvs::store::Datastore;
use mintdb::auth;
use mintdb::prelude::*;
#[tokio::test]
async fn db_signs_up() -> Result<()> {
    let username = "lucy@gmail.com";
    let password = "abc123";
    let db = Datastore::new().await?;

    let jwt = db.sign_up(username, password).await?;
    println!("{}", jwt.token);

    let jwt = db.sign_in(username, password).await?;
    println!("{}", jwt.token);


    auth::token::jwt::verify_jwt(&jwt.token, "secret")?;
    let res = db.sign_out(&jwt.token).await;
    println!("{res:?}");
    Ok(())
}
