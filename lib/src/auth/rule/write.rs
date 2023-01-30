use serde_json::{Value, json};

use crate::kvs::table::{Table, TableMetadata};
use crate::prelude::*;

use super::Rule;
pub fn create_write_rule(tb: &mut Table, rule: Rule) -> Result<Value> {
    for (_, op, _) in rule.iter() {
        match op.as_str() {
            "==" => continue,
            "!=" => continue,
            _ => return Err(Error::Request)
        }
    }
    let res = json!(rule);
    tb.write.extend(rule);
    Ok(res)
}

pub fn validate_write(tb_metadata: &TableMetadata, ctx: &Value) -> Result<()> {
    for (lhs, op, rhs) in tb_metadata.write.iter() {
        if lhs.starts_with("doc.") {
            continue;
        }
        let rhs = match &ctx[rhs] {
            Value::String(s) => s,
            _ => return Err(Error::Request)
        };
        match op.as_str() {
            "==" => {
                if lhs == rhs {
                    continue;
                } else {
                    return Err(Error::Request)
                }
            }
            "!=" => {
                if lhs != rhs {
                    continue;
                } else {
                    return Err(Error::Request)
                }
            }
            _ => return Err(Error::Request)
        }
    }
    Ok(())
}
pub fn validate_document_write(tb_metadata: &TableMetadata, document: &Value, ctx: &Value) -> Result<()> {
    for (lhs, op, rhs) in tb_metadata.write.iter() {
        if !lhs.starts_with("doc.") {
            continue;
        }
        let lhs = match &document[lhs] {
            Value::String(s) => s,
            _ => return Err(Error::Request)
        };
        let rhs = match &ctx[rhs] {
            Value::String(s) => s,
            _ => return Err(Error::Request)
        };
        match op.as_str() {
            "==" => {
                if lhs == rhs {
                    continue;
                } else {
                    return Err(Error::Request)
                }
            }
            "!=" => {
                if lhs != rhs {
                    continue;
                } else {
                    return Err(Error::Request)
                }
            }
            _ => return Err(Error::Request)
        }
    }
    Ok(())
}