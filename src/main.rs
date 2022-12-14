use structopt::StructOpt;

mod opt;
mod leaderboards;
mod answers;
mod quiz;
mod topics_csv;
mod files;
mod helpers;

fn main() {
    let args = opt::Opt::from_args();
    if let Err(e) = opt::Opt::run(args) {
        println!("Failed to start: {}", e);
    }
}
