
use clap::Parser;
use teatime::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, defa)]
    dbfile: String,

}

fn main() {

    let args = Args::parse();

    let d = Duration::default();
    println!("{:}", d);
}
