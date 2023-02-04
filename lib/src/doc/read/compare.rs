use serde_json::{Value, json};

use crate::kvs::store::Datastore;
use crate::prelude::*;

impl Datastore {
    pub(crate) async fn compare_auth(&self, tb: &str, lhs: &str, op: &str, rhs: &Value) -> Result<Value> {
        let documents = self.get_many(tb).await?.as_array().ok_or(Error::Request)?.to_owned();
        let mut res = vec![];
        match op {
            "==" => {
                for document in documents {
                    match &document[lhs] {
                        Value::String(s) => {
                            match rhs {
                                Value::String(rhs) => {
                                    if s == rhs {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }   
                        Value::Number(n) => {
                            match rhs {
                                Value::Number(rhs) => {
                                    if n == rhs {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }
                        _ => continue
                    }
                }
            }
            "!=" => {
                for document in documents {
                    match &document[lhs] {
                        Value::String(s) => {
                            match rhs {
                                Value::String(rhs) => {
                                    if s != rhs {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }   
                        Value::Number(n) => {
                            match rhs {
                                Value::Number(rhs) => {
                                    if n != rhs {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }
                        _ => continue
                    }
                }
            }
            ">" => {
                for document in documents {
                    match &document[lhs] {
                        Value::String(s) => {
                            match rhs {
                                Value::String(rhs) => {
                                    if s > rhs {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }   
                        Value::Number(n) => {
                            match rhs {
                                Value::Number(rhs) => {
                                    if n.as_f64().ok_or(Error::Request)? > rhs.as_f64().ok_or(Error::Request)? {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }
                        _ => continue
                    }
                }
            }
            ">=" => {
                for document in documents {
                    match &document[lhs] {
                        Value::String(s) => {
                            match rhs {
                                Value::String(rhs) => {
                                    if s >= rhs {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }   
                        Value::Number(n) => {
                            match rhs {
                                Value::Number(rhs) => {
                                    if n.as_f64().ok_or(Error::Request)? >= rhs.as_f64().ok_or(Error::Request)? {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }
                        _ => continue
                    }
                }
            }
            "<=" => {
                for document in documents {
                    match &document[lhs] {
                        Value::String(s) => {
                            match rhs {
                                Value::String(rhs) => {
                                    if s <= rhs {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }   
                        Value::Number(n) => {
                            match rhs {
                                Value::Number(rhs) => {
                                    if n.as_f64().ok_or(Error::Request)? <= rhs.as_f64().ok_or(Error::Request)? {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }
                        _ => continue
                    }
                }
            }
            "<" => {
                for document in documents {
                    match &document[lhs] {
                        Value::String(s) => {
                            match rhs {
                                Value::String(rhs) => {
                                    if s < rhs {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }   
                        Value::Number(n) => {
                            match rhs {
                                Value::Number(rhs) => {
                                    if n.as_f64().ok_or(Error::Request)? < rhs.as_f64().ok_or(Error::Request)? {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }
                        _ => continue
                    }
                }
            }
            "contains" => {
                for document in documents {
                    match &document[lhs] {
                        Value::String(s) => {
                            match rhs {
                                Value::String(rhs) => {
                                    if s.contains(rhs) {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }   
                        _ => continue
                    }
                }
            }
            "icontains" => {
                for document in documents {
                    match &document[lhs] {
                        Value::String(s) => {
                            match rhs {
                                Value::String(rhs) => {
                                    if s.to_lowercase().contains(&rhs.to_lowercase()) {
                                        res.push(json!(document))
                                    }
                                }
                                _ => continue
                            }
                        }   
                        _ => continue
                    }
                }
            }
            _ => {

            }
        }
        Ok(json!(res))
    }
}