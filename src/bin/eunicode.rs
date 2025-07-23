use arboard::Clipboard;
use clap::Parser;
use eunicode::unicode_string::UnicodeString;
use std::{
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

/// eunicode is a text processing CLI and library that helps sanitize text
/// by removing the naughty bits to make strings good and safe.
#[derive(Parser)]
#[command(name = "eunicode")]
#[command(
    about = "A text processing CLI and library that helps sanitize text by removing the naughty bits to make strings good and safe"
)]
#[command(version)]
struct Args {
    /// Input text (if not provided via stdin or clipboard)
    #[arg(trailing_var_arg = true)]
    input: Vec<String>,

    /// Output files to write results to
    #[arg(long = "output", value_name = "FILE")]
    output_files: Vec<String>,

    /// Also copy output to clipboard (suppresses stdout unless redirected)
    #[arg(long)]
    clipboard: bool,

    /// Normalize Unicode characters to only safe, ASCII text chars
    #[arg(long)]
    clean: bool,

    /// Remove HTML tags
    #[arg(long)]
    strip: bool,

    /// De-fang hyperlinks
    #[arg(long)]
    defang: bool,

    /// Replace profanity with placeholders
    #[arg(long)]
    censor: bool,

    /// Convert text into chars suitable for a URI slug or filename
    #[arg(long)]
    slugify: bool,

    /// Detect dangerous characters in the input
    #[arg(long)]
    detect: bool,

    /// Show characters present in input, their names, and code points
    #[arg(long)]
    chars: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Get input text and wrap in RawInput state
    let input_text = get_input_text(&args)?;
    let raw_input = UnicodeString::new(input_text);

    // Handle operations that require RawInput state first
    if args.detect || args.chars {
        // these functions are diagnostic and print their output directly
        if args.detect {
            raw_input.detect_dangerous_chars();
        } else if args.chars {
            raw_input.show_character_info();
        }
        exit(0)
    } else {
        // Clean the raw input. If we aren't doing analysis, we always want to clean the text.
        let mut cleaned = raw_input.clean();

        // Apply additional text manipulations
        // TODO reorder these to minimize wasted computation
        if args.strip {
            cleaned = cleaned.strip_html();
        }
        if args.defang {
            cleaned = cleaned.defang_links();
        }
        if args.censor {
            cleaned = cleaned.censor_profanity();
        }
        if args.slugify {
            cleaned = cleaned.sluggify();
        }

        // Handle output
        write_output(&args, &cleaned.into_string())?;
    };

    Ok(())
}

/// Get input text from stdin, CLI args, or clipboard in that order of preference
fn get_input_text(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    // Check if stdin has data (not a TTY and has content)
    if !atty::is(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if !buffer.is_empty() {
            return Ok(buffer);
        }
    }

    // Use CLI arguments if provided
    if !args.input.is_empty() {
        return Ok(args.input.join(" "));
    }

    // Fall back to clipboard
    let mut clipboard = Clipboard::new()?;
    let clipboard_text = clipboard.get_text()?;
    Ok(clipboard_text)
}

/// Write output to appropriate destinations based on args
fn write_output(args: &Args, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stdout_is_redirected = !atty::is(atty::Stream::Stdout);

    // Write to output files
    for file_path in &args.output_files {
        let mut file = File::create(file_path)?;
        file.write_all(text.as_bytes())?;
    }

    // Handle clipboard output
    if args.clipboard {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(text)?;

        // Only write to stdout if it's redirected when clipboard flag is used
        if stdout_is_redirected {
            print!("{}", text);
        }
    } else {
        // Normal stdout output when clipboard flag is not used
        print!("{}", text);
    }

    Ok(())
}
