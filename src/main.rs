use std::error::Error;
use std::io;
use std::str::FromStr;

use structopt::StructOpt;

use printer::{PlainTextTablePrinter, Printer};
use reader::{OneShotValueReader, StreamingValueReader, ValueReader};

use crate::printer::{ColorizeSpec, HtmlTableFormat, HtmlTablePrinter, JsonTable, PlainTextTableFormat, TableFormat, TableHeader};


mod printer;
mod reader;

#[derive(Debug, StructOpt)]
#[structopt(about = "Print any json data as a table from the command line")]
struct Command {
    #[structopt(long, help = "receive one json per line")]
    streaming: bool,

    #[structopt(long, short, help = "select a subset of fields")]
    fields: Option<Vec<String>>,

    #[structopt(long, short, help = "add a color spec to a column in the form of: 'col:value:spec'")]
    colorize: Vec<String>,

    #[structopt(long, default_value = "default", help = "You can use 'default', 'markdown' or 'html")]
    format: TableFormat,

    #[structopt(long, help = "limit the number of printed elements")]
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
            _ => Err(format!("unknown format: {}", s))
        }
    }
}

type GenericResult<T> = Result<T, Box<dyn Error>>;

fn main() -> GenericResult<()> {
    let command: Command = Command::from_args();
    let stdin = io::stdin();

    let data =
        if command.streaming {
            StreamingValueReader::new(stdin.lock()).read_value(command.take)?
        } else {
            OneShotValueReader::new(stdin).read_value(command.take)?
        };

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