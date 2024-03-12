use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    message: Option<String>,
}

fn main() {
    let mut cli = Args::parse();
    if cli.message.is_none() {
        cli.message = Some("y".to_string());
    }
    loop {
        println!("{}", cli.message.as_ref().unwrap());
    }
}