use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // looking for this pattern
    pattern: String,
    // path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("Welcome to runt");
    let args = Cli::from_args();
}
