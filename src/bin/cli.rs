use save_files::client::opts;
use structopt::StructOpt;

fn main(){
    let args: opts::Args = opts::Opts::from_args().try_into().unwrap();

    println!("{:?}", args);
}
