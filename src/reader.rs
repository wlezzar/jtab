use std::error::Error;
use std::fs::read;
use std::io::{BufRead, Read};

use serde_json::{Map, Value};

pub trait ValueReader {
    fn read_value(self, take: Option<usize>) -> anyhow::Result<Value>;
}

pub struct OneShotJsonReader<R: Read> {
    read: R,
}

impl<R: Read> OneShotJsonReader<R> {
    pub fn new(read: R) -> Self {
        OneShotJsonReader { read }
    }
}

impl<R: Read> ValueReader for OneShotJsonReader<R> {
    fn read_value(self, take: Option<usize>) -> anyhow::Result<Value> {
        let value = serde_json::from_reader::<_, Value>(self.read)?;
        if let (Some(take), Value::Array(arr)) = (take, &value) {
            Ok(Value::Array(
                arr.iter().take(take).map(|e| e.to_owned()).collect(),
            ))
        } else {
            Ok(value)
        }
    }
}

pub struct StreamingJsonReader<R: BufRead> {
    buf_read: R,
}

impl<R: BufRead> StreamingJsonReader<R> {
    pub fn new(buf_read: R) -> StreamingJsonReader<R> {
        StreamingJsonReader { buf_read }
    }
}

impl<R: BufRead> ValueReader for StreamingJsonReader<R> {
    fn read_value(self, take: Option<usize>) -> anyhow::Result<Value> {
        let take = take.unwrap_or(100);
        let elements: Vec<Value> = self
            .buf_read
            .lines()
            .take(take)
            .flat_map(|line| match line {
                Ok(line) => match serde_json::from_str::<Value>(line.as_str()) {
                    Ok(parsed) => Some(parsed),
                    Err(err) => {
                        eprintln!("error parsing row: {}", err.to_string());
                        None
                    }
                },
                Err(err) => {
                    eprintln!("error reading row: {}", err.to_string());
                    None
                }
            })
            .collect();

        Ok(Value::Array(elements))
    }
}

pub struct CsvReader<R: BufRead> {
    has_header: bool,
    buf_read: R,
}

impl<R: BufRead> CsvReader<R> {
    pub fn new(buf_read: R, has_header: bool) -> Self {
        CsvReader {
            has_header,
            buf_read,
        }
    }
}

impl<R: BufRead> ValueReader for CsvReader<R> {
    fn read_value(self, take: Option<usize>) -> anyhow::Result<Value> {
        let mut reader = csv::ReaderBuilder::default()
            .has_headers(true)
            .from_reader(self.buf_read);

        let headers = if self.has_header {
            Some(reader.headers()?.clone())
        } else {
            None
        };

        let mut results = Vec::new();

        for row in reader.records().take(take.unwrap_or(100)) {
            let mut value = Map::new();
            let row = row?;
            for (index, col_value) in row.iter().enumerate() {
                let col_name =
                    headers
                        .as_ref()
                        .and_then(|headers| headers.get(index))
                        .map(|v| v.to_string())
                        .unwrap_or(format!("_c{}", index));

                value.insert(
                    col_name,
                    Value::String(col_value.to_string()),
                );
            }

            results.push(Value::Object(value));
        }

        Ok(Value::Array(results))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::reader::{anyhow::Result, CsvReader};
    use crate::ValueReader;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let data = vec![
            "city,country,pop",
            "Boston,United States,4628910",
            "Dallas,United States,2156212",
        ].join("\n");

        let mut rdr = CsvReader {
            has_header: true,
            buf_read: std::io::Cursor::new(data),
        };

        let value = rdr.read_value(None)?;

        assert_eq!(
            value,
            json!([
                {"city": "Boston", "country": "United States", "pop": "4628910"},
                {"city": "Dallas", "country": "United States", "pop": "2156212"},
            ]),
        );

        Ok(())
    }
}