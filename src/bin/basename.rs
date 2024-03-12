use clap::{Parser, ArgAction};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'a', long, help = "Support multiple arguments and treat each as a NAME", action = ArgAction::SetTrue)]
    multiple: bool,
    #[arg(short = 's', long, help = "Remove trailing SUFFIX; implies -a")]
    suffix: String,
    #[arg(short = 'z', long, help = "End each output line with NUL, not newline", action = ArgAction::SetTrue)]
    zero: bool,
    strings: Vec<String>,
}
fn main() {
    let mut cli = Args::parse();
}