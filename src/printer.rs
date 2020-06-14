use std::error::Error;

use prettytable::{Attr, Cell, color, Row, Table};
use serde_json::Value;
use yaml_rust::{Yaml, YamlEmitter};
use yaml_rust::yaml::{Array, Hash};

type GenericResult<T> = Result<T, Box<dyn Error>>;

pub enum TableHeader {
    NamedFields { fields: Vec<String> },
    SingleUnnamedColumn,
}

pub struct JsonTable {
    headers: TableHeader,
    values: Vec<Vec<Value>>,
}

impl JsonTable {
    pub fn new(headers: Option<TableHeader>, root: &Value) -> JsonTable {
        let rows: Vec<Value> = match root {
            Value::Array(arr) => arr.to_owned(), // TODO: is it possible to avoid cloning here?
            _ => vec![root.to_owned()]
        };

        let headers = headers.unwrap_or_else(|| infer_headers(&rows));
        let mut values = Vec::new();

        match &headers {
            TableHeader::NamedFields { fields } => {
                for row in rows {
                    values.push(
                        fields
                            .iter()
                            .map(|h| row.get(h).unwrap_or(&Value::Null).to_owned())
                            .collect()
                    )
                }
            }
            TableHeader::SingleUnnamedColumn => {
                for row in rows {
                    values.push(vec![row.to_owned()])
                }
            }
        }
        JsonTable { headers, values }
    }
}

fn infer_headers(arr: &Vec<Value>) -> TableHeader {
    match arr.first() {
        Some(Value::Object(obj)) => TableHeader::NamedFields {
            fields: obj.keys().map(|h| h.to_owned()).collect()
        },
        _ => TableHeader::SingleUnnamedColumn,
    }
}

pub trait Printer {
    fn print(&self, data: &JsonTable) -> GenericResult<()>;
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
}

impl Printer for TablePrinter {
    fn print(&self, data: &JsonTable) -> GenericResult<()> {
        let mut table = Table::new();

        // header row
        table.add_row(
            Row::new(
                match &data.headers {
                    TableHeader::NamedFields { fields } => {
                        fields
                            .iter()
                            .map(|f| Cell::new(f).style_spec("bFc"))
                            .collect()
                    }
                    TableHeader::SingleUnnamedColumn => vec![Cell::new("value")],
                }
            )
        );

        // data rows
        for value in &data.values {
            let mut row = Row::empty();
            for element in value {
                let formatted = TablePrinter::pprint_table_cell(&element)?;
                row.add_cell(Cell::new(formatted.as_str()))
            }
            table.add_row(row);
        }

        table.printstd();
        Ok(())
    }
}
