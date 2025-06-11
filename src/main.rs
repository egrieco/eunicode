use clap::Parser;
use arboard::Clipboard;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "eunicode")]
#[command(about = "A Unicode character utility")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Convert text to Unicode codepoints
    Encode {
        /// Text to encode (reads from stdin if not provided)
        text: Option<String>,
        /// Copy result to clipboard
        #[arg(short, long)]
        copy: bool,
    },
    /// Convert Unicode codepoints to text
    Decode {
        /// Codepoints to decode (reads from stdin if not provided)
        codepoints: Option<String>,
        /// Copy result to clipboard
        #[arg(short, long)]
        copy: bool,
    },
    /// Show information about a Unicode character
    Info {
        /// Character to analyze
        character: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { text, copy } => {
            let input = get_input(text);
            let result = encode_text(&input);
            output_result(&result, copy);
        }
        Commands::Decode { codepoints, copy } => {
            let input = get_input(codepoints);
            let result = decode_codepoints(&input);
            output_result(&result, copy);
        }
        Commands::Info { character } => {
            show_character_info(&character);
        }
    }
}

fn get_input(arg: Option<String>) -> String {
    match arg {
        Some(text) => text,
        None => {
            if atty::is(atty::Stream::Stdin) {
                eprintln!("Error: No input provided and stdin is a terminal");
                std::process::exit(1);
            }
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
            buffer.trim().to_string()
        }
    }
}

fn output_result(result: &str, copy: bool) {
    println!("{}", result);
    
    if copy {
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        clipboard.set_text(result).expect("Failed to copy to clipboard");
        eprintln!("Copied to clipboard");
    }
}

fn encode_text(text: &str) -> String {
    todo!("Implement text to Unicode codepoints conversion")
}

fn decode_codepoints(codepoints: &str) -> String {
    todo!("Implement Unicode codepoints to text conversion")
}

fn show_character_info(character: &str) {
    todo!("Implement character information display")
}
