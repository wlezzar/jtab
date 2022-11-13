extern crate core;


use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::io;
use std::str::FromStr;

use anyhow::{anyhow, bail};
use structopt::StructOpt;

use printer::{PlainTextTablePrinter, Printer};
use reader::{OneShotJsonReader, StreamingJsonReader, ValueReader};

use crate::printer::{
    ColorizeSpec, HtmlTableFormat, HtmlTablePrinter, JsonTable, PlainTextTableFormat, TableFormat,
    TableHeader,
};
use crate::reader::{CsvReader, CsvReaderOptions};

mod printer;
mod reader;

#[derive(Debug, StructOpt)]
#[structopt(about = "Print any json data as a table from the command line")]
struct Command {
    #[structopt(long, help = "Deprecated: use '--input jsonl' instead")]
    streaming: bool,

    #[structopt(long, short, help = "Select a subset of fields")]
    fields: Option<Vec<String>>,

    #[structopt(long = "--input", short, help = "Format of the input data", default_value = "json")]
    input_format: InputFormat,

    #[structopt(
    long,
    short,
    help = "Add a color spec to a column in the form of: 'col:value:spec'"
    )]
    colorize: Vec<String>,

    #[structopt(
    long,
    short,
    parse(try_from_str = parse_key_val),
    number_of_values = 1
    )]
    /// Options to pass the input reader. Ex: -i csv -o delimiter=';',has_header=false
    ///
    /// Supported options per reader type: csv (delimiter=<char>,has_header=true/false)
    options: Vec<(String, String)>,

    #[structopt(
    long,
    default_value = "default",
    help = "You can use 'default', 'markdown', 'html' or 'html-raw'"
    )]
    format: TableFormat,

    #[structopt(long, help = "Limit the number of printed elements")]
    take: Option<usize>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
    where
        T: FromStr,
        T::Err: Error + 'static,
        U: FromStr,
        U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

impl FromStr for TableFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(TableFormat::PlainText(PlainTextTableFormat::Default)),
            "markdown" => Ok(TableFormat::PlainText(PlainTextTableFormat::Markdown)),
            "html" => Ok(TableFormat::Html(HtmlTableFormat::Styled)),
            "html-raw" => Ok(TableFormat::Html(HtmlTableFormat::Raw)),
            _ => Err(format!("unknown format: {}", s)),
        }
    }
}

#[derive(Debug)]
enum InputFormat {
    Json,
    JsonLines,
    Csv,
}

impl FromStr for InputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "json" => InputFormat::Json,
                "jsonl" => InputFormat::JsonLines,
                "csv" => InputFormat::Csv,
                _ => bail!("Unknown input format: {}", s),
            }
        )
    }
}

impl TryInto<CsvReaderOptions> for Vec<(String, String)> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CsvReaderOptions, Self::Error> {
        let mut options_map: HashMap<_, _> = self.into_iter().collect();
        let options = CsvReaderOptions {
            has_header: options_map.remove("has_header").map(|v| v.parse()).unwrap_or(Ok(true))?,
            delimiter: options_map
                .remove("delimiter")
                .map(|delimiter| {
                    match delimiter.as_bytes().split_first() {
                        None => Err(anyhow!("delimiter must not be empty string")),
                        Some((_, remaining)) if !remaining.is_empty() =>
                            Err(anyhow!("delimiter must be exactly 1 byte. Found: {}", delimiter)),
                        Some((first, _)) => Ok(first.to_owned()),
                    }
                })
                .unwrap_or(Ok(b','))?,
        };

        if !options_map.is_empty() {
            bail!("unknown options for csv reader: {:?}", options_map);
        }

        Ok(options)
    }
}

fn main() -> anyhow::Result<()> {
    let command: Command = Command::from_args();
    let stdin = io::stdin();

    if command.streaming {
        bail!("'--streaming' is deprecated. Use '--input jsonl' instead")
    }

    let data = match command.input_format {
        InputFormat::Json => OneShotJsonReader::new(stdin).read_value(command.take)?,
        InputFormat::JsonLines => StreamingJsonReader::new(stdin.lock()).read_value(command.take)?,
        InputFormat::Csv => CsvReader::new(
            stdin.lock(),
            command.options.clone().try_into()?,
        ).read_value(command.take)?,
    };

    let colorize: Vec<_> = command
        .colorize
        .iter()
        .map(ColorizeSpec::parse)
        .collect::<Result<_, _>>()?;

    let given_headers =
        command.fields.map(|fields| TableHeader::NamedFields { fields });

    let table = JsonTable::new(given_headers, &data);

    match command.format {
        TableFormat::PlainText(format) => {
            PlainTextTablePrinter::new(colorize, format).print(&table)?
        }
        TableFormat::Html(format) => HtmlTablePrinter::new(format).print(&table)?,
    }

    Ok(())
}
