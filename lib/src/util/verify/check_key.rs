use crate::prelude::*;
const ROOT_PATH: &str = "mint.db/ds";
pub fn document_exists(tb: &str, doc: &str) -> bool {
    match std::fs::metadata(f!("{ROOT_PATH}/{tb}/{doc}.bin")) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn tb_exists(tb: &str) -> bool {
    match std::fs::metadata(f!("{ROOT_PATH}/{tb}")) {
        Ok(_) => {
           true
        }
        Err(_) => {
            false
        }
    }
}

pub fn check_exists(tb: &str, doc:&str) -> bool {
    match std::fs::metadata(f!("{ROOT_PATH}/{tb}")) {
        Ok(_) => {
            match std::fs::metadata(f!("{ROOT_PATH}/{tb}/{doc}.bin")) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => {
            false
        }
    }
}