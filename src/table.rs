use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, Lines, Read};

use anyhow::Context;
use csv::{StringRecordsIntoIter, StringRecordsIter};
use serde_json::{Map, Value};

type JsonRowsIterator = Box<dyn Iterator<Item=anyhow::Result<Value>>>;

trait JsonRowsReader {
    fn rows(self) -> JsonRowsIterator;
}

trait JsonRows<T: Iterator<Item=anyhow::Result<Value>>> {
    fn rows(self) -> T;
}

pub enum TableHeader {
    NamedFields { fields: Vec<String> },
    SingleUnnamedColumn,
}

pub struct Table<T: Iterator<Item=Value>> {
    headers: Option<TableHeader>,
    rows: T,
}

struct CsvReader<R: Read + 'static> {
    read: R,
}

impl<R: Read + 'static> CsvReader<R> {
    fn build_iterator(self) -> anyhow::Result<JsonRowsIterator> {
        let mut rdr = csv::ReaderBuilder::default()
            .has_headers(true)
            .from_reader(self.read);

        let headers = rdr.headers().context("Error parsing csv header")?
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(
            Box::new(
                rdr.into_records().map(move |record| {
                    match record {
                        Ok(record) => {
                            let mut value = Map::new();
                            for (index, header) in headers.iter().enumerate() {
                                value.insert(
                                    header.to_string(),
                                    record
                                        .get(index)
                                        .map(|v| serde_json::Value::String(v.to_string()))
                                        .unwrap_or(serde_json::Value::Null));
                            }
                            Ok(Value::Object(value))
                        }
                        Err(err) => {
                            Err(anyhow::Error::new(err).context(
                                format!("Error parsing reading csv record")
                            ))
                        }
                    }
                })
            )
        )
    }
}

impl<R: Read + 'static> JsonRowsReader for CsvReader<R> {
    fn rows(self) -> JsonRowsIterator {
        match self.build_iterator() {
            Ok(it) => it,
            Err(err) => Box::new(std::iter::once(Err(err)))
        }
    }
}


struct FancyCsvReader<R: Read + 'static> {
    options: Option<String>,
    // Todo
    read: R,
}

struct FancyCsvReaderIter<R: Read + 'static> {
    reader: Option<csv::Reader<R>>,
    state: Option<FancyCsvReaderIterState<R>>,
}

struct FancyCsvReaderIterState<R> {
    headers: Vec<String>,
    records: StringRecordsIntoIter<R>,
}

impl<R: Read + 'static> Iterator for FancyCsvReaderIter<R> {
    type Item = anyhow::Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            None => {
                let mut rdr = self.reader.take().expect("Supposed to be initialized!");

                let headers = match rdr.headers() {
                    Ok(headers) => headers.iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>(),
                    Err(err) => return Some(Err(anyhow::Error::new(err))),
                };

                let records = rdr.into_records();

                self.state.replace(
                    FancyCsvReaderIterState {
                        headers,
                        records,
                    }
                );

                self.next()
            }
            Some(FancyCsvReaderIterState { records, headers }) => match records.next() {
                None => None,
                Some(Ok(record)) => {
                    let mut value = Map::new();
                    for (index, header) in headers.iter().enumerate() {
                        value.insert(
                            header.to_string(),
                            record
                                .get(index)
                                .map(|v| Value::String(v.to_string()))
                                .unwrap_or(Value::Null));
                    }
                    Some(Ok(Value::Object(value)))
                }
                Some(Err(err)) => todo!(),
            }
        }
    }
}

impl<R: Read + 'static> JsonRows<FancyCsvReaderIter<R>> for FancyCsvReader<R> {
    fn rows(self) -> FancyCsvReaderIter<R> {
        FancyCsvReaderIter {
            reader: Some(
                csv::ReaderBuilder::default()
                    .has_headers(true)
                    .from_reader(self.read)
            ),
            state: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::table::{CsvReader, FancyCsvReader, JsonRows, JsonRowsReader};

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let data = vec![
            "city,country,pop",
            "Boston,United States,4628910",
            "Dallas,United States,2156212",
        ].join("\n");

        let mut rdr = FancyCsvReader {
            options: None,
            read: std::io::Cursor::new(data),
        };

        let rows = rdr.rows().collect::<anyhow::Result<Vec<_>>>()?;

        assert_eq!(
            rows,
            vec![
                json!({"city": "Boston", "country": "United States", "pop": "4628910"}),
                json!({"city": "Dallas", "country": "United States", "pop": "2156212"}),
            ],
        );

        Ok(())
    }


    // #[test]
    // fn it_works() -> anyhow::Result<()> {
    //     let data = vec![
    //         "city,country,pop",
    //         "Boston,United States,4628910",
    //         "Dallas,United States,2156212",
    //     ].join("\n");
    //
    //     let mut rdr = CsvReader { read: std::io::Cursor::new(data) };
    //     let rows = rdr.rows().collect::<anyhow::Result<Vec<_>>>()?;
    //
    //     assert_eq!(
    //         rows,
    //         vec![
    //             json!({"city": "Boston", "country": "United States", "pop": "4628910"}),
    //             json!({"city": "Dallas", "country": "United States", "pop": "2156212"}),
    //         ],
    //     );
    //
    //     Ok(())
    // }
}