use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;
use std::{
    fs::File,
    io::{BufReader, Result},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Param {
    #[structopt(parse(from_os_str))]
    infile: PathBuf,

    #[structopt(parse(from_os_str))]
    outfile: PathBuf,
}

const BUFF_SIZE: usize = 1024 * 8; // 8kb, can perform faster copy for large files

fn main() -> Result<()> {
    let params = Param::from_args();

    //TODO:
    // Handle when input and output path are same
    // When given path is directory
    let input_file = File::open(params.infile)?;
    let output_file = File::create(params.outfile)?;
    //create a buffered reader for the input file
    let mut buf_reader = BufReader::new(input_file);
    let mut buf_writer = BufWriter::with_capacity(BUFF_SIZE, output_file);

    let mut buf = [0; BUFF_SIZE];

    loop {
        match buf_reader.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                'writer: loop {
                    let mut finished = 0;
                    match buf_writer.write(&buf[finished..n]) {
                        Ok(written) => {
                            finished = finished + written;
                            if finished == written {
                                break 'writer;
                            }
                        }
                        Err(e) => {
                            panic!("Failed to write with an error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error reading file {}", e);
                break;
            }
        }
    }
    //let the main throw the error
    buf_writer.flush()
}
