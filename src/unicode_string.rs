use crate::convert::{
    char_identifier_to_string, general_category_to_string, unicode_block_to_string,
    unicode_script_to_string,
};
use ammonia::Builder;
use charname::get_name;
use deunicode::deunicode;
use limace::Slugifier;
use linkify::LinkFinder;
use prettytable::{Cell as TableCell, Row, Table, format::consts::FORMAT_CLEAN, row};
use rustrict::CensorStr;
use std::{collections::HashSet, marker::PhantomData, process::exit};
use ucd::Codepoint;
use unicode_security::{
    GeneralSecurityProfile, RestrictionLevel::ASCIIOnly, RestrictionLevelDetection, skeleton,
};

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

    pub fn to_string(&self) -> &str {
        &self.text
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
            TableCell::new(general_category_to_string(c.category())),
            TableCell::new(unicode_block_to_string(c.block())),
            TableCell::new(unicode_script_to_string(c.script())),
            TableCell::new(
                c.identifier_type()
                    .map_or("Unknown Character Type", |t| char_identifier_to_string(t)),
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
            if !char_names.is_empty() {
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
