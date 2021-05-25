use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Param {
    #[structopt(parse(from_os_str))]
    infile: PathBuf,

    #[structopt(parse(from_os_str))]
    outfile: PathBuf,
}

fn main() {
    let params = Param::from_args();
    let res = fs::read_to_string(params.infile.clone());

    let data = res.expect("failed to read input file");
    let mut file = match File::create(params.outfile) {
        Err(e) => panic!("Could not create oiutput file {}", e),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Ok(()) => println!(" done"),
        Err(e) => println!("{}", e),
    }
}
