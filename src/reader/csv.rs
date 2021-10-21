use std::collections::HashMap;
use std::io::{BufRead, Read};

use csv::Reader;
use serde_json::{Error, Value};

use crate::reader::{GenericResult, StreamingValueReader, ValueReader};

pub struct CsvReader<R: BufRead> {
    reader_builder: csv::ReaderBuilder,
    read: R,
}

impl<R: BufRead> CsvReader<R> {
    pub fn new(read: R, delimiter: u8, no_header: bool) -> Self {
        let mut reader_builder = csv::ReaderBuilder::default();
        reader_builder
            .delimiter(delimiter)
            .has_headers(!no_header);

        CsvReader {
            reader_builder,
            read,
        }
    }
}

type CsvRecord = HashMap<String, String>;

impl<R: BufRead> ValueReader for CsvReader<R> {
    fn read_value(&mut self, take: Option<usize>) -> GenericResult<Value> {
        let take = take.unwrap_or(100);
        let read = &mut self.read;
        let mut reader = self.reader_builder.from_reader(read);

        let elements: Vec<Value> =
            reader
                .deserialize::<CsvRecord>()
                .take(take)
                .flat_map(|line| match line {
                    Ok(line) => {
                        match serde_json::to_value(line) {
                            Ok(parsed) => Some(parsed),
                            Err(err) => {
                                eprintln!("error parsing csv record: {}", err.to_string());
                                None
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("error reading row: {}", err.to_string());
                        None
                    }
                })
                .collect();

        Ok(Value::Array(elements))
    }
}

