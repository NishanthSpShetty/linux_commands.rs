use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;
use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Result},
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

    // 1. Handle when input and output path are same
    if params.infile == params.outfile {
        return Err(Error::new(
            ErrorKind::Other,
            "source and target path are same.",
        ));
    }

    //TODO:
    //2. When given source path is directory
    let in_path = params.infile.as_path();
    let input_file = File::open(in_path).expect("failed to open source file");
    // 3. When given target path is directory
    let output_file: File;

    //is_dir will return true if path is a directory
    if !params.outfile.is_dir() {
        output_file = File::create(params.outfile).expect("failed to create output file");
    } else {
        //we need to create target file inside the given target path
        let mut output_path = PathBuf::new();
        output_path.push(params.outfile.as_path());
        output_path.push(in_path);

        println!("{:?}", output_path);
        output_file = File::create(output_path).expect("filed to creat output file");
    }

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
                //Write all content read from the file in the following writer loop
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
