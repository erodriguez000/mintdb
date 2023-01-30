use serde_json::{Value, json};
use crate::auth::rule::read::create_read_rule;
use crate::auth::rule::write::create_write_rule;
use crate::{kvs::store::Datastore, auth::rule::Rule};
use crate::prelude::*;

impl Datastore {
    pub async fn add_read_rule(&self, tb:&str, rule: Rule) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            let res = create_read_rule(tbl, rule)?;
            Ok(res)
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn get_read_rules(&self, tb: &str) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        if let Some(tbl) = lock.tables.get(tb) {
            let res = &tbl.read;
            Ok(json!(res))
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn remove_read_rule(&self, tb: &str, rule: Rule) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            for (index, (lhs, op, rhs)) in tbl.read.iter().enumerate() {
                if lhs == &rule[0].0 && op == &rule[0].1 && rhs == &rule[0].2 {
                    tbl.read.remove(index);
                    return Ok(json!(rule))
                }
            }
            return Err(Error::Request)
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn add_write_rule(&self, tb:&str, rule: Rule) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            let res = create_write_rule(tbl, rule)?;
            Ok(res)
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn get_write_rules(&self, tb: &str) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        if let Some(tbl) = lock.tables.get(tb) {
            let res = &tbl.write;
            Ok(json!(res))
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn remove_write_rule(&self, tb: &str, rule: Rule) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            for (index, (lhs, op, rhs)) in tbl.write.iter().enumerate() {
                if lhs == &rule[0].0 && op == &rule[0].1 && rhs == &rule[0].2 {
                    tbl.read.remove(index);
                    return Ok(json!(rule))
                }
            }
            return Err(Error::Request)
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn add_delete_rule(&self, tb:&str, rule: Rule) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            let res = create_write_rule(tbl, rule)?;
            Ok(res)
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn get_delete_rules(&self, tb: &str) -> Result<Value> {
        let lock = self.collections.try_read().unwrap();
        if let Some(tbl) = lock.tables.get(tb) {
            let res = &tbl.delete;
            Ok(json!(res))
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
    pub async fn remove_delete_rule(&self, tb: &str, rule: Rule) -> Result<Value> {
        let mut lock = self.collections.try_write().unwrap();
        if let Some(tbl) = lock.tables.get_mut(tb) {
            for (index, (lhs, op, rhs)) in tbl.delete.iter().enumerate() {
                if lhs == &rule[0].0 && op == &rule[0].1 && rhs == &rule[0].2 {
                    tbl.read.remove(index);
                    return Ok(json!(rule))
                }
            }
            return Err(Error::Request)
        } else {
            return Err(Error::TableNotFound(tb.into()))
        }
    }
}