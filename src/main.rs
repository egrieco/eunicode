use ammonia::Builder;
use arboard::Clipboard;
use charname::get_name;
use clap::Parser;
use deunicode::deunicode;
use eunicode::{unicode_block_to_string, unicode_script_to_string};
use limace::Slugifier;
use linkify::LinkFinder;
use prettytable::{Cell as TableCell, Row, Table, format::consts::FORMAT_CLEAN, row};
use rustrict::CensorStr;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read, Write};
use std::marker::PhantomData;
use std::process::exit;
use ucd::{Codepoint, UnicodeBlock, UnicodeCategory};
use unicode_security::{
    GeneralSecurityProfile, RestrictionLevel::ASCIIOnly, RestrictionLevelDetection,
    general_security_profile::IdentifierType, skeleton,
};

const NOT_CHARACTER: &str = "Not Character";
const DEPRECATED: &str = "Deprecated";
const DEFAULT_IGNORABLE: &str = "Default Ignorable";
const NOT_NFKC: &str = "Not NFKC";
const NOT_XID: &str = "Not XID";
const EXCLUSION: &str = "Exclusion";
const OBSOLETE: &str = "Obsolete";
const TECHNICAL: &str = "Technical";
const UNCOMMON_USE: &str = "Uncommon Use";
const LIMITED_USE: &str = "Limited Use";
const INCLUSION: &str = "Inclusion";
const RECOMMENDED: &str = "Recommended";

fn char_identifier_to_string(ident: IdentifierType) -> &'static str {
    match ident {
        IdentifierType::Not_Character => NOT_CHARACTER.into(),
        IdentifierType::Deprecated => DEPRECATED.into(),
        IdentifierType::Default_Ignorable => DEFAULT_IGNORABLE.into(),
        IdentifierType::Not_NFKC => NOT_NFKC.into(),
        IdentifierType::Not_XID => NOT_XID.into(),
        IdentifierType::Exclusion => EXCLUSION.into(),
        IdentifierType::Obsolete => OBSOLETE.into(),
        IdentifierType::Technical => TECHNICAL.into(),
        IdentifierType::Uncommon_Use => UNCOMMON_USE.into(),
        IdentifierType::Limited_Use => LIMITED_USE.into(),
        IdentifierType::Inclusion => INCLUSION.into(),
        IdentifierType::Recommended => RECOMMENDED.into(),
    }
}

fn general_category_to_string(cat: UnicodeCategory) -> String {
    match cat {
        UnicodeCategory::ClosePunctuation => "ClosePunctuation".into(),
        UnicodeCategory::ConnectorPunctuation => "ConnectorPunctuation".into(),
        UnicodeCategory::Control => "Control".into(),
        UnicodeCategory::CurrencySymbol => "CurrencySymbol".into(),
        UnicodeCategory::DashPunctuation => "DashPunctuation".into(),
        UnicodeCategory::DecimalNumber => "DecimalNumber".into(),
        UnicodeCategory::EnclosingMark => "EnclosingMark".into(),
        UnicodeCategory::FinalPunctuation => "FinalPunctuation".into(),
        UnicodeCategory::Format => "Format".into(),
        UnicodeCategory::InitialPunctuation => "InitialPunctuation".into(),
        UnicodeCategory::LetterNumber => "LetterNumber".into(),
        UnicodeCategory::LineSeparator => "LineSeparator".into(),
        UnicodeCategory::LowercaseLetter => "LowercaseLetter".into(),
        UnicodeCategory::MathSymbol => "MathSymbol".into(),
        UnicodeCategory::ModifierLetter => "ModifierLetter".into(),
        UnicodeCategory::ModifierSymbol => "ModifierSymbol".into(),
        UnicodeCategory::NonspacingMark => "NonspacingMark".into(),
        UnicodeCategory::OpenPunctuation => "OpenPunctuation".into(),
        UnicodeCategory::OtherLetter => "OtherLetter".into(),
        UnicodeCategory::OtherNumber => "OtherNumber".into(),
        UnicodeCategory::OtherPunctuation => "OtherPunctuation".into(),
        UnicodeCategory::OtherSymbol => "OtherSymbol".into(),
        UnicodeCategory::ParagraphSeparator => "ParagraphSeparator".into(),
        UnicodeCategory::PrivateUse => "PrivateUse".into(),
        UnicodeCategory::SpaceSeparator => "SpaceSeparator".into(),
        UnicodeCategory::SpacingMark => "SpacingMark".into(),
        UnicodeCategory::Surrogate => "Surrogate".into(),
        UnicodeCategory::TitlecaseLetter => "TitlecaseLetter".into(),
        UnicodeCategory::Unassigned => "Unassigned".into(),
        UnicodeCategory::UppercaseLetter => "UppercaseLetter".into(),
    }
}

/// TypeState state definitions
pub mod string_states {
    use super::StringState;

    pub struct RawInput {}
    pub struct CleanedText {}

    /// RawInput state: may data may contain dangerous characters - allows detect and chars operations to examine content
    impl StringState for RawInput {}

    /// CleanedText state: dangerous characters have been removed - allows strip, defang, censor, and sluggify operations
    impl StringState for CleanedText {}
}

// TODO do we want to seal this trait? probably
pub trait StringState {}

/// TypeState wrapper for text processing
pub struct UnicodeString<S: StringState> {
    text: String,
    _marker: PhantomData<S>,
}

impl UnicodeString<string_states::RawInput> {
    pub fn new(text: String) -> Self {
        Self {
            text,
            _marker: PhantomData,
        }
    }

    /// Normalize Unicode characters to only safe, ASCII text chars and transition to CleanedText state
    pub fn clean(self) -> UnicodeString<string_states::CleanedText> {
        UnicodeString::<string_states::CleanedText> {
            text: deunicode(&self.text),
            _marker: PhantomData,
        }
    }

    fn character_info(index: &usize, c: char) -> Row {
        Row::new(vec![
            TableCell::new(&index.to_string()),
            // NOTE: we're calling deunicode here to avoid printing potentially dangerous characters
            // TODO might want to print all "safe" printable characters to allow better analysis
            TableCell::new(&deunicode(&c.to_string())),
            TableCell::new(&general_category_to_string(c.category())),
            TableCell::new(&unicode_block_to_string(c.block())),
            TableCell::new(&unicode_script_to_string(c.script())),
            TableCell::new(
                c.identifier_type()
                    .map_or("Unknown Character Type", |t| &char_identifier_to_string(t)),
            ),
            TableCell::new(get_name(c as u32)),
        ])
    }

    fn print_char_table(rows: Vec<Row>) {
        let mut table = Table::init(rows);
        table.set_titles(row![
            "Index",
            "Char",
            "Category",
            "Block",
            "Script",
            "Identifier Type",
            "Name"
        ]);
        table.set_format(*FORMAT_CLEAN);
        table.printstd();
    }

    /// Detect dangerous characters (only available on RawInput)
    pub fn detect_dangerous_chars(self) {
        // TODO allow user selection of restriction level
        if !self.text.check_restriction_level(ASCIIOnly) {
            // the ASCIIOnly check is too restrictive as it is intended for identifiers
            let char_names: Vec<_> = skeleton(&self.text)
                .enumerate()
                .flat_map(|(i, c)| {
                    if !(c.is_ascii_graphic() || c.is_ascii_whitespace()) {
                        Some(Self::character_info(&i, c))
                    } else {
                        None
                    }
                })
                .collect();
            // we'll only throw an error if we found non-graphic, non-whitespace characters beyond the ASCII range
            if char_names.len() > 0 {
                eprintln!("String has restricted characters!");
                Self::print_char_table(char_names);
                exit(2)
            }
        }
        // otherwise the string should be safe
        // TODO fix bug detecting strings like "Æneid"
        eprintln!("String is safe");
        println!("{}", self.text);
        exit(0)
    }

    /// Show character info (only available on RawInput)
    pub fn show_character_info(self) -> String {
        let rows: Vec<Row> = skeleton(&self.text)
            .enumerate()
            .map(|(i, c)| Self::character_info(&i, c))
            .collect();
        Self::print_char_table(rows);
        exit(2)
    }
}

impl UnicodeString<string_states::CleanedText> {
    /// Strip HTML tags
    pub fn strip_html(self) -> Self {
        let text = Builder::new()
            .tags(HashSet::default())
            .clean(&self.text)
            .to_string();
        Self {
            text,
            _marker: PhantomData,
        }
    }

    /// De-fang hyperlinks
    pub fn defang_links(self) -> Self {
        // let mut text = String::new();
        let text = LinkFinder::new()
            .spans(&self.text)
            .map(|span| match span.kind() {
                Some(link_kind) => match link_kind {
                    linkify::LinkKind::Url => {
                        // TODO use better "defanging logic"
                        span.as_str()
                            .replace("ftp", "fXp")
                            .replace("http", "hXXp")
                            .replace('.', "[.]")
                    }
                    linkify::LinkKind::Email => {
                        span.as_str().replace('@', "[@]").replace('.', "[.]")
                    }
                    // linkify has the LinkKind enum marked as non-exhaustive so we have to have this catch-all
                    _ => unimplemented!("unsupported link type encountered"),
                },
                None => span.as_str().to_string(),
            })
            .collect();
        Self {
            text,
            _marker: PhantomData,
        }
    }

    /// Censor profanity
    pub fn censor_profanity(self) -> Self {
        Self {
            text: self.text.censor(),
            _marker: PhantomData,
        }
    }

    /// Convert to sluggified text
    pub fn sluggify(self) -> Self {
        Self {
            text: Slugifier::default().slugify(self.text),
            _marker: PhantomData,
        }
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
    let output_text = if args.detect || args.chars {
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
