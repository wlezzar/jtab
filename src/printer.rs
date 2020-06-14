use std::error::Error;

use prettytable::{Cell, Row, Table};
use serde_json::Value;
use yaml_rust::{Yaml, YamlEmitter};
use yaml_rust::yaml::{Array, Hash};

type GenericResult<T> = Result<T, Box<dyn Error>>;

pub trait Printer {
    fn print(&self, value: &Value);
}

#[derive(Default)]
pub struct TablePrinter;

fn json_to_yaml(value: &Value) -> Yaml {
    match value {
        Value::Object(obj) => {
            let mut hash = Hash::new();
            for (key, value) in obj {
                hash.insert(Yaml::String(key.to_owned()), json_to_yaml(value));
            }
            Yaml::Hash(hash)
        }
        Value::Array(arr) => {
            let arr = arr.iter().map(|e| json_to_yaml(e)).collect::<Vec<_>>();
            Yaml::Array(Array::from(arr))
        }
        Value::Null => Yaml::Null,
        Value::Bool(e) => Yaml::Boolean(e.to_owned()),
        Value::Number(n) => Yaml::Real(format!("{}", n)),
        Value::String(s) => Yaml::String(s.to_owned())
    }
}

impl TablePrinter {
    fn pprint_table_cell(value: &Value) -> GenericResult<String> {
        match value {
            Value::String(s) => Ok(s.to_string()),
            Value::Object(_) | Value::Array(_) => {
                let mut res = String::new();
                {
                    let yaml_form = json_to_yaml(value);
                    let mut emitter = YamlEmitter::new(&mut res);
                    emitter.dump(&yaml_form)?;
                }
                Ok(res.trim_start_matches("---\n").to_string())
            }
            _ => Ok(serde_json::to_string(value)?)
        }
    }

    fn print_arr(arr: &Vec<Value>) {
        let mut table = Table::new();
        match arr.first() {
            Some(Value::Object(obj)) => {
                let header: Vec<&String> = obj.keys().collect();
                table.add_row(
                    header
                        .iter()
                        .map(|h| Cell::new(h))
                        .collect()
                );
                for row in arr {
                    table.add_row(
                        header
                            .iter()
                            .map(|h| {
                                let h = row.get(h).unwrap_or(&Value::Null);
                                TablePrinter::pprint_table_cell(h)
                                    .expect("don't know why it failed!")
                            })
                            .map(|v| Cell::new(v.as_str()))
                            .collect()
                    );
                }
            }
            Some(_) => {
                table.add_row(Row::new(vec![Cell::new("value")]));
                for row in arr {
                    table.add_row(
                        Row::new(
                            vec![
                                Cell::new(&TablePrinter::pprint_table_cell(row)
                                    .expect("don't know why it failed!"))
                            ]
                        )
                    );
                }
            }
            None => {
                table.add_row(Row::new(vec![Cell::new("empty")]));
            }
        }
        table.print_tty(false);
    }
}

impl Printer for TablePrinter {
    fn print(&self, value: &Value) {
        match value {
            Value::Array(arr) => TablePrinter::print_arr(arr),
            _ => TablePrinter::print_arr(&vec![value.to_owned()])
        }
    }
}

struct JsonPrinter;

impl Printer for JsonPrinter {
    fn print(&self, value: &Value) {
        println!("{}", serde_json::to_string(value).expect("json value should always be stringifiable"))
    }
}