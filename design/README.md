# Design Docs

This directory contains design documentation which explains why certain decisions were made regarding the overall architecture of this program and the crates used.

## Potential Crates

Here are some relevant crates. Most categories contain crates with duplicate functionality and a few are unmaintained. We'll need to decide which exact crate to use from each category.

Crates are listed somewhat in order of most viable/full-featured/maintained to least.

### Removing Non-ASCII and Dangerous Characters

- [deunicode](https://lib.rs/crates/deunicode): Convert Unicode strings to pure ASCII by intelligently transliterating them. Suppors Emoji and Chinese.
- [decancer](https://lib.rs/crates/decancer): A library that removes common unicode confusables/homoglyphs from strings.
- [any_ascii](https://lib.rs/crates/any_ascii): Unicode to ASCII transliteration
- [uniaxe](https://lib.rs/crates/uniaxe): A Rust crate to replace Unicode letters with Ascii equivalents
- [superfold](https://lib.rs/crates/superfold): A multilingual Rust library and CLI to process UTF-8 strings to exclude diacritics and fold non-phonetic graphemes into their phonetic ASCII representation.
- [secular](https://lib.rs/crates/secular): No Diacr!
- [unidecode](https://lib.rs/crates/unidecode): Provides pure ASCII transliterations of Unicode strings.

### Detecting Non-ASCII and Dangerous Characters

- [unicop](https://lib.rs/crates/unicop): Tool for scanning source code for potentially malicious unicode code points. Helps prevent Trojan source bidi attacks, homoglyph attacks, invisible character attacks etc. Intended to run manually or in CI to help with supply chain security.
- [unicode-security](https://lib.rs/crates/unicode-security): Detect possible security problems with Unicode usage according to Unicode Technical Standard #39 rules.
- [invisible_unicode](https://lib.rs/crates/invisible_unicode): Library for finding invisible unicode characters.
- [unicode_skeleton](https://lib.rs/crates/unicode_skeleton): This crate detects unicode strings that look nearly identical once rendered, but do not compare as equal. It defines "confusable" and "skeleton" based on Unicode Standard Annex #39.

### Confusables and Homoglyphs

- [confusables](https://lib.rs/crates/confusables): Utilities around Unicode confusables/homoglyphs
- [homoglyphs](https://lib.rs/crates/homoglyphs): generate all homoglyphs for a given input sentence
- [omekasy](https://lib.rs/crates/omekasy): Decorate alphanumeric characters in your input with various font; special characters in Unicode

### HTML Stripping

- [ammonia](https://lib.rs/crates/ammonia): HTML Sanitization
- [strip-tags](https://lib.rs/crates/strip-tags): Strip HTML and PHP tags from strings

### De-fanging URLs

- [‘defang’ search // Lib.rs](https://lib.rs/search?q=defang)
- [dfang](https://lib.rs/crates/dfang): A tool to defang IOCs.
- [rfang](https://lib.rs/crates/rfang): A tool to refang IOCs.
- [urlsafe](https://lib.rs/crates/urlsafe): A tool to defang and re-arm malicious URLs
- [linkify](https://lib.rs/crates/linkify): Finds URLs and email addresses in plain text. Takes care to get the boundaries right with surrounding punctuation like parentheses.

### Removing Profanity

- [#censor // Lib.rs](https://lib.rs/keywords/censor)
- [rustrict](https://lib.rs/crates/rustrict): rustrict is a profanity filter for Rust
- [censor](https://lib.rs/crates/censor): A simple text profanity filter
- [stfu](https://lib.rs/crates/stfu): Shut The Ferris Up - profanity filtering for Rust
- [profane-rs](https://lib.rs/crates/profane-rs): Check Messages For Profanity/Swearing.

### Slugifying Text

- [#slug // Lib.rs](https://lib.rs/keywords/slug)
- [limace](https://lib.rs/crates/limace): Slugify some strings
- [slug](https://lib.rs/crates/slug): Convert a unicode string to a slug
- [stubble](https://lib.rs/crates/stubble): A command-line tool for generating content stubs
- [slugify-rs](https://lib.rs/crates/slugify-rs): A rust library to generate slugs from strings
- [el-slugify](https://lib.rs/crates/el-slugify): URL slug generator utility. Slugs are generated efficiently, fast, they are transliterated and sanitized for use in URLs.

### Character Identification

- [chars](https://lib.rs/crates/chars): A commandline tool to display information about unicode characters
- [uniwhat](https://lib.rs/crates/uniwhat): Display the unicode characters text
- [find_unicode](https://lib.rs/crates/find_unicode): Find Unicode characters, the easy way!
- [charname](https://lib.rs/crates/charname): Incredibly simple library that just gives you the Unicode name for a character.
- [charclass](https://lib.rs/crates/charclass): Library crate to define and modify unicode character classes
- [u-plus](https://lib.rs/crates/u-plus): Pretty Unicode code point literals: U+12345
