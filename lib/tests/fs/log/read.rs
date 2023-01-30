use mintdb::fs::log::read::*;
use mintdb::fs::log::write::*;
use mintdb::prelude::*;

#[tokio::test]
async fn it_reads_logs() -> Result<()>  {
    log_merge_event("Hello, world!").await?;
    fetch_logs().await?;

    Ok(())
}

#[tokio::test]
async fn it_gets_merge_logs() -> Result<()>  {
    log_merge_event("Hello, world!").await?;
    let res = fetch_merge_logs().await?;
    for log in res {
        assert!(log.contains("MERGE"))
    }
    Ok(())
}

#[tokio::test]
async fn it_gets_create_logs() -> Result<()>  {
    log_create_event("Hello, world!").await?;
    let res = fetch_create_logs().await?;
    for log in res {
        assert!(log.contains("CREATE"))
    }
    Ok(())
}

#[tokio::test]
async fn it_gets_read_logs() -> Result<()>  {
    log_read_event("Hello, world!").await?;
    let res = fetch_read_logs().await?;
    for log in res {
        assert!(log.contains("SELECT"))
    }
    Ok(())
}

#[tokio::test]
async fn it_gets_delete_logs() -> Result<()>  {
    log_delete_event("Hello, world!").await?;
    let res = fetch_delete_logs().await?;
    for log in res {
        assert!(log.contains("DELETE"))
    }
    Ok(())
}

#[tokio::test]
async fn it_gets_warning_logs() -> Result<()>  {
    log_warning("Hello, world!").await?;
    let res = fetch_warning_logs().await?;
    for log in res {
        assert!(log.contains("WARNING"))
    }
    Ok(())
}

#[tokio::test]
async fn it_gets_error_logs() -> Result<()>  {
    log_error("Hello, world!").await?;
    let res = fetch_error_logs().await?;
    for log in res {
        assert!(log.contains("ERROR"))
    }
    Ok(())
}