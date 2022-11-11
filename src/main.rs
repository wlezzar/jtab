extern crate core;

use std::error::Error;
use std::io;
use std::str::FromStr;

use anyhow::bail;
use structopt::StructOpt;

use printer::{PlainTextTablePrinter, Printer};
use reader::{OneShotJsonReader, StreamingJsonReader, ValueReader};

use crate::printer::{
    ColorizeSpec, HtmlTableFormat, HtmlTablePrinter, JsonTable, PlainTextTableFormat, TableFormat,
    TableHeader,
};
use crate::reader::CsvReader;

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
    default_value = "default",
    help = "You can use 'default', 'markdown', 'html' or 'html-raw'"
    )]
    format: TableFormat,

    #[structopt(long, help = "Limit the number of printed elements")]
    take: Option<usize>,
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


fn main() -> anyhow::Result<()> {
    let command: Command = Command::from_args();
    let stdin = io::stdin();

    if command.streaming {
        bail!("'--streaming' is deprecated. Use '--input jsonl' instead")
    }

    let data = match command.input_format {
        InputFormat::Json => OneShotJsonReader::new(stdin).read_value(command.take)?,
        InputFormat::JsonLines => StreamingJsonReader::new(stdin.lock()).read_value(command.take)?,
        InputFormat::Csv => CsvReader::new(stdin.lock(), true).read_value(command.take)?,
    };

    let colorize: Vec<_> = command
        .colorize
        .iter()
        .map(|c| ColorizeSpec::parse(c))
        .collect::<Result<_, _>>()?;

    let given_headers = match command.fields {
        Some(fields) => Some(TableHeader::NamedFields { fields }),
        None => None,
    };

    let table = JsonTable::new(given_headers, &data);

    match command.format {
        TableFormat::PlainText(format) => {
            PlainTextTablePrinter::new(colorize, format).print(&table)?
        }
        TableFormat::Html(format) => HtmlTablePrinter::new(format).print(&table)?,
    }

    Ok(())
}
