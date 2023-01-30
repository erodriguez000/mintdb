use mintdb::prelude::*;
use mintdb::fs::log::write::*;

#[tokio::test]
async fn log_writes_() -> Result<()> {
    log_event("it writes!").await?;
    log_merge_event("It writes a merge event").await?;
    log_create_event("It writes create events!").await?;
    log_delete_event("It writes delete events!").await?;
    log_read_event("It writes a read event!").await?;
    log_warning("It writes a warning event in yellow").await?;
    log_error("It writes an error event in red!").await?;
    Ok(())
}