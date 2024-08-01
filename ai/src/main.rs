
use ai::{run, Args, Parser};


fn main() {
    let args = Args::parse();
    run(args).unwrap();
}