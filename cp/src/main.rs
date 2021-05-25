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

fn main() -> Result<()> {
    let params = Param::from_args();

    //TODO:
    // Handle when input and output path are same
    // When given path is directory
    let input_file = File::open(params.infile)?;
    let output_file = File::create(params.outfile)?;
    //create a buffered reader for the input file
    let mut buf_reader = BufReader::new(input_file);
    let mut buf_writer = BufWriter::with_capacity(1024, output_file);

    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        match buf_reader.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                match buf_writer.write(&buf) {
                    Ok(written) => {
                        //FIXME: "should handle by writing in loop");
                        if n != written {
                            panic!(
                                "unable to write the read content to ne file Read {}bytes, Written {}bytes",
                                n, written
                            );
                        }
                    }
                    Err(e) => {
                        panic!("Failed to write with an error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error reading file {}", e);
                break;
            }
        }
    }

    //Nothing to do here, all good
    Ok(())
}
