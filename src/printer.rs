use std::error::Error;

use prettytable::{Cell, Row, Table};
use serde_json::Value;

type GenericResult<T> = Result<T, Box<dyn Error>>;

pub trait Printer {
    fn print(&self, value: &Value);
}

#[derive(Default)]
pub struct TablePrinter;

impl TablePrinter {
    fn pprint_table_cell(value: &Value, depth: usize) -> GenericResult<String> {
        match value {
            Value::Object(obj) => {
                let mut res = String::new();
                for (key, value) in obj {
                    res.push_str(
                        format!(
                            "{}: {}",
                            key, TablePrinter::pprint_table_cell(value, depth + 1)?
                        ).as_str()
                    );
                    if depth < 1 {
                        res.push_str("\n")
                    }
                }
                Ok(res)
            }
            Value::Array(arr) => {
                let mut res = String::new();
                for value in arr {
                    res.push_str(
                        format!(
                            "{:indent$}- {}",
                            TablePrinter::pprint_table_cell(value, depth + 1)?, indent = depth
                        ).as_str()
                    );
                }
                Ok(res)
            }
            Value::String(s) => Ok(s.to_string()),
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
                            .map(|h| TablePrinter::pprint_table_cell(row.get(h).unwrap_or(&Value::Null), 0).expect("don't know why it failed!"))
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
                                Cell::new(&TablePrinter::pprint_table_cell(row, 0).expect("don't know why it failed!"))
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
        println!("{}", serde_json::to_string(value).expect("json value is always stringifiable"))
    }
}