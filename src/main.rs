use save_files::opts::{Args, Opts};
use structopt::StructOpt;

fn main() {
    let arg: Args = Opts::from_args().try_into().unwrap();
    println!("{:?}", arg);
}
