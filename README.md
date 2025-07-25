# eunicode

`eunicode` is a text processing CLI and library that helps sanitize text by removing the naughty bits to make strings good and safe:

1. No unexpected unicode characters to prevent homograph attacks.
2. No dangerous characters (zero-width, text direction changes, etc.) used to hide dangerous text in innocent looking strings.
3. "De-fanging" links so that sharing links to IoCs and malware analysis repositories can't be accidentally clicked.

This was built partly as a demo for a talk about NewType and TypeState patterns, and also because I couldn't find a simple CLI to sanitize strings even though there are some great Rust crates to do exactly that.

## Naming

From the prefix _eu-_:

1. **eu-**: Used to form taxonomic names corresponding to English vernacular names beginning with true

2. **eu-**: Pronunciation: /iːu̟/. Origin: Ancient Greek: εὖ (eû). Meaning: "good", "well"; also extended via Neo-Latin to mean "true". Used in a variety of ways, often to indicate well-preserved specimens, well-developed bones, "truer" examples of fossil forms, or simply admiration on the part of the discoverer.

Also, from _eunuch_

1. **eunuch**: n 1: a man who has been castrated and is incapable of reproduction; "eunuchs guarded the harem"

2. **eunuch**: Such a man employed as harem guard or in certain (mainly Eastern) monarchies (e.g. late Roman and Chinese Empires) as court or state officials.

## Features

### Output

- [x] **--output** `<FILE>`: Output files to write results to
- [x] **--clipboard**: Also copy output to clipboard (suppresses stdout unless redirected)

### Sanitization

- [x] **--clean**: Normalize Unicode characters to only safe, ASCII text chars. Uses [deunicode](https://crates.io/crates/deunicode).
- [x] **--strip**: Remove HTML tags. Uses [ammonia](https://crates.io/crates/ammonia).
- [x] **--defang**: De-fang hyperlinks. Uses [linkify](https://crates.io/crates/linkify) to find links with custom code to de-fang them.
- [x] **--censor**: Replace profanity with placeholders. Uses [rustrict](https://crates.io/crates/rustrict).
- [x] **--slugify**: Convert text into chars suitable for a URI slug or filename. Uses [limace](https://crates.io/crates/limace).
- [x] **--keep-colors**: Keep CSI SGR codes to allow text formatting in the terminal. Uses a custom implementation via [termwiz](https://crates.io/crates/termwiz).

### Inspection

- [x] **--detect**: Detect dangerous characters in the input
- [x] **--chars**: Show characters present in input, their names, and code points
- [ ] Highlight confusables: change the color and make potentially confusable characters bold.

### Testing

- [x] **--raw-bytes**: Emit the raw bytes after ANSI stripping but without further cleaning (mostly useful for testing)

## Caveats

- **clean** is overly aggressive and emits only ASCII text. We probably need to allow different levels of cleaning.
- **strip** removes all html tags. We might want to allow for some exceptions.
- **censor** uses the [rustrict](https://github.com/finnbear/rustrict/) library, so it will not censor words that rustrict misses. This can be tested via the [swear-words](https://github.com/chucknorris-io/swear-words) repo with the command `eunicode --censor < ~/Repos/swear-words/en` or similarly with other lists of profanity.
