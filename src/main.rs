use arboard::Clipboard;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::exit;

/// TypeState wrapper for text processing
/// RawInput state - allows detect and chars operations
pub struct RawInput {
    text: String,
}

/// CleanedText state - allows strip, defang, and censor operations
pub struct CleanedText {
    text: String,
}

impl RawInput {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    /// Clean the text and transition to CleanedText state
    pub fn clean(self) -> CleanedText {
        CleanedText {
            text: clean_text(self.text),
        }
    }

    /// Detect dangerous characters (only available on RawInput)
    pub fn detect_dangerous_chars(self) -> String {
        detect_dangerous_chars(self.text)
    }

    /// Show character info (only available on RawInput)
    pub fn show_character_info(self) -> String {
        show_character_info(self.text)
    }

    /// Convert to sluggified text directly from raw input
    pub fn sluggify(self) -> String {
        sluggify_text(self.text)
    }

    /// Get the inner text
    pub fn into_string(self) -> String {
        self.text
    }
}

impl CleanedText {
    /// Strip HTML tags
    pub fn strip_html(self) -> Self {
        Self {
            text: strip_html(self.text),
        }
    }

    /// De-fang hyperlinks
    pub fn defang_links(self) -> Self {
        Self {
            text: defang_links(self.text),
        }
    }

    /// Censor profanity
    pub fn censor_profanity(self) -> Self {
        Self {
            text: censor_profanity(self.text),
        }
    }

    /// Convert to sluggified text
    pub fn sluggify(self) -> String {
        sluggify_text(self.text)
    }

    /// Get the inner text
    pub fn into_string(self) -> String {
        self.text
    }
}

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
    sluggify: bool,

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
    let raw_input = RawInput::new(input_text);

    // Handle operations that require RawInput state first
    let output_text = if args.detect {
        raw_input.detect_dangerous_chars()
    } else if args.chars {
        raw_input.show_character_info()
    } else if args.sluggify && !args.clean {
        // Sluggify directly from raw input if not cleaning first
        raw_input.sluggify()
    } else {
        // Process through the normal pipeline
        let mut processed = if args.clean {
            Some(raw_input.clean())
        } else {
            None
        };

        // If we have cleaned text, apply operations that work on CleanedText
        if let Some(cleaned) = processed {
            let mut result = cleaned;

            if args.strip {
                result = result.strip_html();
            }
            if args.defang {
                result = result.defang_links();
            }
            if args.censor {
                result = result.censor_profanity();
            }

            if args.sluggify {
                result.sluggify()
            } else {
                result.into_string()
            }
        } else {
            // No cleaning was done, just return the original text
            eprintln!(
                "Please choose an operation. We are quitting without printing to avoid printing potentially dangerous textual values."
            );
            exit(1)
        }
    };

    // Handle output
    write_output(&args, &output_text)?;

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

// Operation functions - all use todo!() as requested

/// Normalize Unicode characters to only safe, ASCII text chars
fn clean_text(input: String) -> String {
    todo!()
}

/// Remove HTML tags
fn strip_html(input: String) -> String {
    todo!()
}

/// De-fang hyperlinks
fn defang_links(input: String) -> String {
    todo!()
}

/// Replace profanity with placeholders
fn censor_profanity(input: String) -> String {
    todo!()
}

/// Convert text into chars suitable for a URI slug or filename
fn sluggify_text(input: String) -> String {
    todo!()
}

/// Detect dangerous characters in the input
fn detect_dangerous_chars(input: String) -> String {
    todo!()
}

/// Show characters present in input, their names, and code points
fn show_character_info(input: String) -> String {
    todo!()
}
