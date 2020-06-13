use std::default::Default;
use std::error::Error;
use std::io;

use structopt::StructOpt;

use printer::{Printer, TablePrinter};
use reader::{OneShotValueReader, StreamingValueReader, ValueReader};

mod printer;
mod reader;

#[derive(Debug, StructOpt)]
struct Command {
    #[structopt(long)]
    streaming: bool,

    #[structopt(long)]
    take: Option<usize>,
}

type GenericResult<T> = Result<T, Box<dyn Error>>;

fn main() -> GenericResult<()> {
    let command: Command = Command::from_args();
    let stdin = io::stdin();
    let value =
        if command.streaming {
            StreamingValueReader::new(stdin.lock()).read_value(command.take)?
        } else {
            OneShotValueReader::new(stdin).read_value(command.take)?
        };

    TablePrinter::default().print(&value);
    Ok(())
}
