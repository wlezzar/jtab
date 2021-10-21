use std::error::Error;
use std::io::{BufRead, Read};

use serde_json::Value;

use crate::GenericResult;

pub mod csv;

pub trait ValueReader {
    fn read_value(&mut self, take: Option<usize>) -> GenericResult<Value>;
}

pub struct OneShotValueReader<R: Read> {
    read: R,
}

impl<R: Read> OneShotValueReader<R> {
    pub fn new(read: R) -> Self {
        OneShotValueReader { read }
    }
}

impl<R: Read> ValueReader for OneShotValueReader<R> {
    fn read_value(&mut self, take: Option<usize>) -> GenericResult<Value> {
        let read = &mut self.read;
        let value = serde_json::from_reader::<_, Value>(read)?;
        if let (Some(take), Value::Array(arr)) = (take, &value) {
            Ok(Value::Array(arr.iter().take(take).map(|e| e.to_owned()).collect()))
        } else {
            Ok(value)
        }
    }
}

pub struct StreamingValueReader<R: BufRead> {
    buf_read: R,
}

impl<R: BufRead> StreamingValueReader<R> {
    pub fn new(buf_read: R) -> StreamingValueReader<R> {
        StreamingValueReader { buf_read }
    }
}

impl<R: BufRead> ValueReader for StreamingValueReader<R> {
    fn read_value(&mut self, take: Option<usize>) -> GenericResult<Value> {
        let take = take.unwrap_or(100);
        let elements: Vec<Value> =
            self.buf_read
                .by_ref()
                .lines()
                .take(take)
                .flat_map(|line| match line {
                    Ok(line) => {
                        match serde_json::from_str::<Value>(line.as_str()) {
                            Ok(parsed) => Some(parsed),
                            Err(err) => {
                                eprintln!("error parsing row: {}", err.to_string());
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