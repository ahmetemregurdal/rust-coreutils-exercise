use clap::{Parser, ArgAction};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    message: Option<String>,
    #[arg(short = 'n', long, action = ArgAction::SetTrue, help = "Do not output the trailing newline")]
    no_newline: bool,
    #[arg(short = 'e', long, help = "Enable interpretation of backslash escapes", action = ArgAction::SetTrue)]
    enable_backslash: bool,
    #[arg(short = 'E', long, help = "Disable interpretation of backslash escapes(default)", action = ArgAction::SetTrue)]
    disable_backslash: bool,
}

fn main() {
    let mut cli = Args::parse();
    if cli.message.is_none() {
        cli.message = Some("".to_string());
    }
    if !cli.enable_backslash {
        if !cli.no_newline {
            println!("{}", cli.message.unwrap());
        } else {
            print!("{}", cli.message.unwrap());
        }
    } else {
        let mut new_message: String = "".to_string();
        let original_text_vec: Vec<char> = cli.message.unwrap().chars().collect();
        let mut skip = false;
        for i in 0..(original_text_vec.len() - 1) {
            if skip {
                skip = false;
                continue;
            } else if i == original_text_vec.len() - 1 {
                new_message.push(original_text_vec[i]);
            } else if original_text_vec[i] != '\\' {
                new_message.push(original_text_vec[i]);
            } else {
                skip = true;
                if original_text_vec[i + 1] == 'n' {
                    new_message.push('\n');
                } else if original_text_vec[i + 1] == '\\' {
                    new_message.push('\\');
                } else if original_text_vec[i + 1] == 'b' {
                    new_message.push('\x08');
                } else if original_text_vec[i + 1] == 'a' {
                    new_message.push('\x07');
                } else if original_text_vec[i + 1] == 'c' {
                    break;
                } else if original_text_vec[i + 1] == 'e' {
                    new_message.push('\x1b');
                } else if original_text_vec[i + 1] == 'f' {
                    new_message.push('\x0c');
                } else if original_text_vec[i + 1] == 'r' {
                    new_message.push('\r');
                } else if original_text_vec[i + 1] == 't' {
                    new_message.push('\t');
                } else if original_text_vec[i + 1] == 'v' {
                    new_message.push('\x0b');
                }
                // TODO: Support gor \0NNN and \xHH
            }
        }
        if cli.no_newline {
            print!("{new_message}");
        } else {
            println!("{new_message}");
        }
    }
}