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
