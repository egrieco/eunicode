# TODO

We need to deduplicate the below tasks.

- [x] Strip ANSI chars
- [ ] Join with str
- [ ] Squeeze whitespace
- [ ] Chomp
- [ ] Sort numeric
- [ ] Sort by semver (numeric might be enough)
- [ ] Sort by regex field
- [ ] Sort by list from other file

Some of these are redundant with `tr` and other standard Unix tools, but should be included for convenience since this can place the result back on the clipboard easily.

- [ ] Chomp to remove trailing whitespace runs
- [ ] Trim to remove whitespace runs
- [ ] Squeeze spaces
- [ ] Join lines: replace all newline characters (and adjacent spaces) with a space
- [ ] Quote, to wrap output in quotes for use in the shell
- [ ] Add more unicode info see `chr` and `char`
