use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::io;
use std::iter::Map;
use std::num::ParseIntError;
use std::str::FromStr;

use structopt::StructOpt;

use printer::{PlainTextTablePrinter, Printer};
use reader::{OneShotValueReader, StreamingValueReader, ValueReader};

use crate::printer::{ColorizeSpec, HtmlTableFormat, HtmlTablePrinter, JsonTable, PlainTextTableFormat, TableFormat, TableHeader};

mod printer;
mod reader;

pub type GenericResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, StructOpt)]
#[structopt(about = "Print any json data as a table from the command line")]
struct Command {
    #[structopt(long, short, help = "Select a subset of fields")]
    fields: Option<Vec<String>>,

    #[structopt(long, short, help = "Add a color spec to a column in the form of: 'col:value:spec'")]
    colorize: Vec<String>,

    #[structopt(long, default_value = "default", help = "You can use 'default', 'markdown', 'html' or 'html-raw'")]
    format: TableFormat,

    #[structopt(long, help = "Limit the number of printed elements")]
    take: Option<usize>,

    #[structopt(subcommand)]
    content_type: ContentSubcommand,
}

#[derive(Debug, StructOpt)]
enum ContentSubcommand {
    Csv {
        #[structopt(short, long)]
        delimiter: String,
        #[structopt(long = "--no-header")]
        no_header: bool,
    },
    Json {
        #[structopt(long, help = "Expect one json per line")]
        streaming: bool,
    },
}

impl FromStr for TableFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(TableFormat::PlainText(PlainTextTableFormat::Default)),
            "markdown" => Ok(TableFormat::PlainText(PlainTextTableFormat::Markdown)),
            "html" => Ok(TableFormat::Html(HtmlTableFormat::Styled)),
            "html-raw" => Ok(TableFormat::Html(HtmlTableFormat::Raw)),
            _ => Err(format!("unknown format: {}", s))
        }
    }
}

fn main() -> GenericResult<()> {
    let command: Command = Command::from_args();
    let stdin = io::stdin();

    let mut reader: Box<dyn ValueReader> = match command.content_type {
        ContentSubcommand::Csv { delimiter, no_header } => {
            let delimiter = match delimiter.as_str() {
                r#"\t"# => '\t' as u8,
                _ => parse_delimiter(delimiter)?
            };

            Box::new(CsvReader::new(stdin.lock(), delimiter, no_header))
        }
        ContentSubcommand::Json { streaming } => {
            match streaming {
                true => Box::new(StreamingValueReader::new(stdin.lock())),
                false => Box::new(OneShotValueReader::new(stdin))
            }
        }
    };

    let data = reader.read_value(command.take)?;

    let colorize: Vec<_> =
        command.colorize.iter().map(|c| ColorizeSpec::parse(c)).collect::<Result<_, _>>()?;

    let given_headers = match command.fields {
        Some(fields) => Some(TableHeader::NamedFields { fields }),
        None => None
    };

    let table = JsonTable::new(given_headers, &data);

    match command.format {
        TableFormat::PlainText(format) => PlainTextTablePrinter::new(colorize, format).print(&table)?,
        TableFormat::Html(format) => HtmlTablePrinter::new(format).print(&table)?,
    }

    Ok(())
}

fn parse_delimiter(str: String) -> Result<u8, String> {
    let mut chars = str.chars();

    match chars.next() {
        None => Err("empty delimiter".to_string()),
        Some(delimiter) if chars.next().is_none() => Ok(delimiter as u8),
        _ => Err(format!("delimiter string contains more than one character: {}", str))
    }
}