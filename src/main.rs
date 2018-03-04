extern crate graphml2mm;
#[macro_use]
extern crate structopt;

use graphml2mm::*;
use structopt::*;
use std::*;
use std::path::*;
use std::fs::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "grahml2mm")]
struct Opt {
    /// input file
    #[structopt(name = "IN", parse(from_os_str))]
    graphml_path: PathBuf,

    /// output file
    #[structopt(name = "OUT", parse(from_os_str))]
    mm_path: PathBuf
}

type AppResult = result::Result<(), AppError>;

#[derive(Debug)]
pub enum AppError{
    AppReaderError(graph::reader::ReaderError),
    AppIOError(std::io::Error)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Application Error")
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::AppIOError(err)
    }
}

impl From<graph::reader::ReaderError> for AppError {
    fn from(err: graph::reader::ReaderError) -> AppError {
        AppError::AppReaderError(err)
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::AppIOError(_) => { "IO Error" }
            AppError::AppReaderError(_) => { "Reader Error" }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AppError::AppIOError( ref err) => { Some(err) }
            AppError::AppReaderError(ref err) => { Some(err) }
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("Start processing...");
    let result = run(&opt.graphml_path, &opt.mm_path);
    match result {
        Ok(_) => { println!("DONE: File {:?} was successfully converted and written to {:?}", opt.graphml_path, opt.mm_path); }
        Err( AppError::AppIOError (err @ _)) => { println!("ERROR: {:?}", err); }
        Err( AppError::AppReaderError (err @ _)) => { println!("ERROR: {:?}", err); }
    }
}

fn run(input_path: &PathBuf, output_path: &PathBuf) -> AppResult {
    let events = graph::reader::graphml_reader(File::open(input_path)?)?;
    graph::writer::mm_writer(&mut File::create(output_path)?, &events)?;
    Ok(())
}
