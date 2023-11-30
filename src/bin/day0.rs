use clap::{self, Parser};

#[derive(Parser, Debug)]
#[command(author="Vampire Exec", version="0.0", about=format!("solution for {}", file!()), long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world! {:?}", args);
}
