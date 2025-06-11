# Context

`eunicode` is a text processing CLI and library that helps sanitize text by removing the naughty bits to make strings good and safe.

Use the above description in the help output for the program.

## Code

- Make a CLI app that uses the `clap` library for argument processing.
- Write this following Rust best practices and idioms.
- All code should compile without errors or warnings, and pass `clippy` lints.
- Add comments where necessary to explain why something is being done if the approach is non-standard or likely to confuse an intermediate programmer.
- Add `rustdoc` documentation to structs, functions, modules, traits, etc.

It should have the following functionality.

### Input

1. Detect if STDIN has been redirected into the app and accept the input if so.
2. If there is no input on STDIN, concatenate all bare CLI arguments with spaces and operate on that input as if it had been provide on STDIN.
3. If neither of the above are available, pull the current textual data from the clipboard and use that as input.

### Output

1. Output should default to STDOUT, however if we detect that user the has redirected output to a file we should send it there.
2. If one or more `--output=` flags have been given, the output should also be written to the specified files.
3. If `--clipboard` is specified, output should also be placed on the clipboard. However, unless STDOUT has been redirected, it should not be written to STDOUT.

### Operations

- The functionality below corresponds to command flags that invoke functions to do their work.
- The functions should not contain any actual functionality or code beyond the rust `todo!()` macro.

1. `--clean` - Normalize Unicode characters to only safe, ASCII text chars
2. `--strip` - Remove HTML tags (we might allow white-listing later)
3. `--defang` - De-fang hyperlinks
4. `--censor` - Replace profanity with placeholders
5. `--sluggify`- Convert the text into chars suitable for a URI slug or filename
6. `--detect` - Detect dangerous characters in the input (homoglyphs, invisible , etc.)
7. `--chars` - Show the characters present in the input, their names, and code points
