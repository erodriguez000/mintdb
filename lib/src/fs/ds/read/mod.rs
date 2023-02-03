use serde_cbor::from_reader;
use serde_json::json;
use crate::kvs::{store::Datastore, table::TableMetadata};
use crate::kvs::document::Document;
use crate::prelude::*;

use super::write::{open_directory, open_document_file};

pub async fn read_from_fs(db: &Datastore) -> Result<()> {
    check_ds_dir().await?;
    let tbls = std::fs::read_dir("mint.db/ds")
    .unwrap()
    .map(|res| res.unwrap())
    .filter(|dir| dir.file_type().unwrap().is_dir())
    .map(|dir| dir.file_name().into_string().unwrap())
    .collect::<Vec<String>>();
    for tb in tbls {
        read_table(db, &tb).await?;
    }
    Ok(())
}

pub async fn read_table(db: &Datastore, tb: &str) -> Result<()> {
    let path = f!("mint.db/ds/{}", tb);
    open_dir(&path).await?;
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let pb = entry.path();
        let mut file = match std::fs::File::open(&pb) {
            Ok(f) => f,
            Err(_) => {
                std::fs::File::create(pb.clone())?
            }
        };

        let doc_id = pb.file_stem()
        .ok_or(Error::Request)?
        .to_str()
        .ok_or(Error::Request)?;

        let document: Document = from_reader(&mut file).unwrap_or_default();

        db.merge_auth(tb, doc_id, json!(document.data)).await?;
    }
    Ok(())
}

pub async fn read_document(tb:&str, doc: &str) -> Result<Document> {
    open_directory(tb).await?;
    let mut file = open_document_file(tb, doc).await?;
    let document: Document = from_reader(&mut file).unwrap_or_default();
    Ok(document)
}
pub async fn read_tb_metadata(tb: &str) -> Result<TableMetadata> {
    open_directory(tb).await?;
    let mut file = open_document_file(tb, "meta").await?;
    let tb_metadata: TableMetadata = from_reader(&mut file).unwrap_or_default();
    Ok(tb_metadata)

}
pub async fn open_dir(path: &str) -> Result<()> {
    match std::fs::metadata("mint.db/") {
        Ok(_) => {
            match std::fs::metadata("mint.db/ds") {
                Ok(_) => {
                    match std::fs::metadata(path) {
                        Ok(_) => Ok(()),
                        Err(_) => {
                            std::fs::create_dir_all(path)?;
                            Ok(())
                        }
                    }
                }
                Err(_) => {
                    std::fs::create_dir_all(path)?;
                    Ok(())
                }
            }
        }
        Err(_) => {
            std::fs::create_dir_all(path)?;
            Ok(())
        }
    }
}
pub async fn check_ds_dir() -> Result<()> {
    match std::fs::metadata("mint.db") {
        Ok(_) => {
            match std::fs::metadata("mint.db/ds") {
                Ok(_) => {

                }
                Err(_) => {
                    std::fs::create_dir_all("mint.db/ds")?;
                }
            }
        }
        Err(_) => {
            std::fs::create_dir_all("mint.db/ds")?;
        }
    };
    Ok(())
}