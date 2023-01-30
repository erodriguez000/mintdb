#![allow(unused)]
use std::collections::HashMap;

use crate::{
    kvs::table::{Table, TableMetadata},
    prelude::*,
};
use serde_json::{from_value, json, Map, Value};

pub fn validate_schema_key(tb_metadata: &TableMetadata, key: &str, value: &Value) -> Result<()> {
    if tb_metadata.schema.contains_key(key) {
        match value {
            Value::Object(_) => {
                if tb_metadata.schema[key] == "Object" {
                } else {
                    return Err(Error::SchemaInvalidDataType(key.into(), "Object".into()));
                }
            }
            Value::Array(_) => {
                if tb_metadata.schema[key] == "Array" {
                } else {
                    return Err(Error::SchemaInvalidDataType(key.into(), "Array".into()));
                }
            }
            Value::String(_) => {
                if tb_metadata.schema[key] == "String" {
                } else {
                    return Err(Error::SchemaInvalidDataType(key.into(), "String".into()));
                }
            }
            Value::Number(_) => {
                if tb_metadata.schema[key] == "Number" {
                } else {
                    return Err(Error::SchemaInvalidDataType(key.into(), "Number".into()));
                }
            }
            _ => return Err(Error::Request),
        }
    } else {
        return Err(Error::SchemaInvalidKeyTable(key.into(), tb_metadata.id.to_string()));
    }
    Ok(())
}

pub fn validate_schema_merge(
    tb_metadata: &TableMetadata,
    data: &Value,
) -> Result<Map<String, Value>> {
    let data = data.as_object().ok_or(Error::Request)?.to_owned();
    for (key, value) in data.iter() {
        if tb_metadata.schema.contains_key(key) {
            match &data[key] {
                Value::Object(_) => {
                    if tb_metadata.schema[key] == "Object" {
                        continue;
                    } else {
                        return Err(Error::SchemaInvalidDataType(key.into(), "Object".into()));
                    }
                }
                Value::Array(_) => {
                    if tb_metadata.schema[key] == "Array" {
                        continue;
                    } else {
                        return Err(Error::SchemaInvalidDataType(key.into(), "Array".into()));
                    }
                }
                Value::String(_) => {
                    if tb_metadata.schema[key] == "String" {
                        continue;
                    } else {
                        return Err(Error::SchemaInvalidDataType(key.into(), "String".into()));
                    }
                }
                Value::Number(_) => {
                    if tb_metadata.schema[key] == "Number" {
                        continue;
                    } else {
                        return Err(Error::SchemaInvalidDataType(key.into(), "Number".into()));
                    }
                }
                _ => return Err(Error::Request),
            }
        } else {
            return Err(Error::SchemaInvalidKeyTable(key.into(), tb_metadata.id.to_string()))
        }
    }
    Ok(data)
}
pub fn validate_schema_create(
    tb_metadata: &TableMetadata,
    data: &Value,
) -> Result<Map<String, Value>> {
    let data = data.as_object().ok_or(Error::Request)?.to_owned();
    for (key, value) in &tb_metadata.schema {
        if data.contains_key(key) {
            match &data[key] {
                Value::Object(_) => {
                    if value == "Object" {
                        continue;
                    } else {
                        return Err(Error::SchemaInvalidDataType(key.into(), "Object".into()));
                    }
                }
                Value::Array(_) => {
                    if value == "Array" {
                        continue;
                    } else {
                        return Err(Error::SchemaInvalidDataType(key.into(), "Array".into()));
                    }
                }
                Value::String(_) => {
                    if value == "String" {
                        continue;
                    } else {
                        return Err(Error::SchemaInvalidDataType(key.into(), "String".into()));
                    }
                }
                Value::Number(_) => {
                    if value == "Number" {
                        continue;
                    } else {
                        return Err(Error::SchemaInvalidDataType(key.into(), "Number".into()));
                    }
                }
                _ => return Err(Error::Request),
            }
        } else {
            return Err(Error::SchemaMissingKeyofType(key.into(), value.into()));
        }
    }
    Ok(data)
}

pub fn create_schema(tb: &mut Table, schema: &Value) -> Result<()> {
    let schema: HashMap<String, String> = from_value(json!(schema))?;
    for (key, value) in schema.iter() {
        match value.as_str() {
            "Object" => (),
            "Array" => (),
            "String" => (),
            "Number" => (),
            _ => return Err(Error::Request),
        }
    }
    tb.schema = schema;
    Ok(())
}
