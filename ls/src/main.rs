use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Param {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn ls_file(path: &PathBuf) {
    println!("Reading file {:?}", path);
}

fn ls_dir(path: &PathBuf) {
    let readdir = path.read_dir().expect("failed to read directory");
    for direntry in readdir {
        ls_file(&direntry.unwrap().path());
    }
}

fn main() {
    let params = Param::from_args();

    if params.path.is_dir() {
        ls_dir(&params.path);
    } else {
        ls_file(&params.path);
    }
}
