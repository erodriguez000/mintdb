use std::collections::BTreeMap;
use std::fs::metadata;
use serde_cbor::to_writer;
use serde_json::json;
use serde_json::Value;
use crate::kvs::document::Document;
use crate::kvs::table::TableMetadata;
use crate::prelude::*;

pub async fn commit_document(tb: &str, doc: &str, data: Value) -> Result<()> {
    let data= data.as_object()
    .ok_or(Error::Request)?
    .iter().map(|(k, v)| (k.to_owned(), v.to_owned()))
    .collect::<BTreeMap<String, Value>>();
    let mut file = open_document_file(tb, doc).await?;
    let mut document = Document::new(doc);
    document.data = data.to_owned();
    match to_writer(&mut file, &json!(document)) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::BinError(e))
    }
}


pub async fn delete_document(tb: &str, doc: &str) -> Result<()> {
    let path = f!("mint.db/ds/{tb}/{doc}.bin");
    match metadata(&path) {
        Ok(_) => {
            std::fs::remove_file(path)?;
            Ok(())
        }
        Err(e) => {
            Err(Error::IOError(e))
        }
    }
}
pub async fn commit_table(tb: TableMetadata) -> Result<()> {
    let path = f!("mint.db/ds/{}/meta.bin", &tb.id);
    open_directory(&tb.id).await?;
    let mut file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .open(&path)?;
    match to_writer(&mut file, &json!(tb)) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::BinError(e))
    }
}


pub async fn delete_table(tb: &str) -> Result<()> {
    let path = f!("mint.db/ds/{tb}");
    match metadata(&path) {
        Ok(_) => {
            std::fs::remove_dir_all(path)?;
            Ok(())
        }
        Err(e) => Err(Error::IOError(e))
    }
}

pub async fn open_directory(tb: &str) -> Result<()> {
    let path = f!("mint.db/ds/{tb}");
    match std::fs::metadata("mint.db") {
        Ok(_) => {
            match metadata("mint.db/ds/") {
                Ok(_) => {
                    match metadata(&path) {
                        Ok(_) => {
                            Ok(())
                        }
                        Err(_) => {
                            std::fs::create_dir_all(path)?;
                            Ok(())
                        }
                    }
                }
                Err(_) => {
                    std::fs::create_dir_all(&path)?;
                    Ok(())
                }
            }
        }
        Err(_) => {
            let path = f!("mint.db/ds/{tb}");
            std::fs::create_dir_all(path)?;
            Ok(())
        }
    }
}

pub async fn open_document_file(tb: &str, doc: &str) -> Result<std::fs::File> {
    open_directory(tb).await?;
    let path = format!("mint.db/ds/{}/{}.bin", tb, doc);
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;
    Ok(file)
}