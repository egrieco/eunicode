# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.5 (2026-02-20)

### Chore

 - <csr-id-72be97c7fc666d6e72069635203af0581c64c9f5/> Update incompatible deps
   This addresses a security issue in ammonia:
   RUSTSEC-2025-0071: Incorrect handling of embedded SVG and MathML leads to mutation XSS after removal
   https://rustsec.org/advisories/RUSTSEC-2025-0071.html

### Documentation

 - <csr-id-9d2dc2b1e16e40c3a764fbccc360de6fbca3a733/> Add features section

### New Features

 - <csr-id-0b5e0d60b339c58d0bc41186cf31355985ea342e/> Add the --raw-bytes flag

### Refactor

 - <csr-id-b3a57935785c9bd3331a3551a79db928574ccf95/> replace atty crate with std::io::IsTerminal

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 210 calendar days.
 - 211 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update Cargo.lock ([`473293b`](https://github.com/egrieco/eunicode/commit/473293b5648b09de68185d6a0efa97531d2ee13c))
    - Replace atty crate with std::io::IsTerminal ([`b3a5793`](https://github.com/egrieco/eunicode/commit/b3a57935785c9bd3331a3551a79db928574ccf95))
    - Update incompatible deps ([`72be97c`](https://github.com/egrieco/eunicode/commit/72be97c7fc666d6e72069635203af0581c64c9f5))
    - Add Tim Bray's Bad Unicode reference ([`84d0fcc`](https://github.com/egrieco/eunicode/commit/84d0fcc84415823180dfcb07eb28c9bb2222f8bb))
    - Add additional pass-through characters ([`d958320`](https://github.com/egrieco/eunicode/commit/d958320d582ac866ce31619c39df10801e934da4))
    - Add features section ([`9d2dc2b`](https://github.com/egrieco/eunicode/commit/9d2dc2b1e16e40c3a764fbccc360de6fbca3a733))
    - Add the --raw-bytes flag ([`0b5e0d6`](https://github.com/egrieco/eunicode/commit/0b5e0d60b339c58d0bc41186cf31355985ea342e))
</details>

## v0.1.4 (2025-07-23)

<csr-id-23c4095b30dbf2d171ccb4fa342d1c670cc62060/>

### Refactor

 - <csr-id-23c4095b30dbf2d171ccb4fa342d1c670cc62060/> Switch to Vec<Action>
   Hopefully this fixes the issue with the terminal escapes getting incorrectly rendered when called as a library.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release eunicode v0.1.4 ([`6687496`](https://github.com/egrieco/eunicode/commit/6687496f41c7bd0fc32ed07fe38d9738a9ab994d))
    - Switch to Vec<Action> ([`23c4095`](https://github.com/egrieco/eunicode/commit/23c4095b30dbf2d171ccb4fa342d1c670cc62060))
</details>

## v0.1.3 (2025-07-23)

### New Features

 - <csr-id-8f5111fb05df5ffa433129ccfdedacbf5bd8f69e/> Add from_bytes to RawBytes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release eunicode v0.1.3 ([`cd38461`](https://github.com/egrieco/eunicode/commit/cd384614d6457b60c97b0bdb4a5133d29ad4535f))
    - Add from_bytes to RawBytes ([`8f5111f`](https://github.com/egrieco/eunicode/commit/8f5111fb05df5ffa433129ccfdedacbf5bd8f69e))
</details>

## v0.1.2 (2025-07-23)

<csr-id-f280e6ef0ee916a72d04ab69f4fc6b1aed2ef7c9/>
<csr-id-ee51c1d930cbdbafe2026ca129785a4882fd59b7/>

### New Features

 - <csr-id-fc8380a162274a187fb04dbffd8b183256a32838/> add proper SGR CSI code rendering for `--keep-colors` flag
 - <csr-id-0e0ac7e207e004834845b5d2ce49b4b154dd7a73/> Add raw bytes and ANSI escape parsing

### Bug Fixes

 - <csr-id-0aee7345c2ef978a4de1180d8368bb9151457359/> Fix code from aider
   There was an extra "m" character that should not have been there.
   Also, let's use the constant from termwiz rather than hard coding the value.
 - <csr-id-0604fd298fa3ccd209111de03e02ef7748849a63/> Auto-fix clippy lints

### Other

 - <csr-id-f280e6ef0ee916a72d04ab69f4fc6b1aed2ef7c9/> Add nix shell and rust toolchain

### Refactor

 - <csr-id-ee51c1d930cbdbafe2026ca129785a4882fd59b7/> Reorganize bin and lib

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 2 calendar days.
 - 29 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release eunicode v0.1.2 ([`676acb2`](https://github.com/egrieco/eunicode/commit/676acb29bb09b879d9da816fe3993b6fb645188c))
    - Add design documents ([`7c5a64c`](https://github.com/egrieco/eunicode/commit/7c5a64c9435e5b8da82d895ce4f5296123fd465f))
    - Fix code from aider ([`0aee734`](https://github.com/egrieco/eunicode/commit/0aee7345c2ef978a4de1180d8368bb9151457359))
    - Add proper SGR CSI code rendering for `--keep-colors` flag ([`fc8380a`](https://github.com/egrieco/eunicode/commit/fc8380a162274a187fb04dbffd8b183256a32838))
    - Add raw bytes and ANSI escape parsing ([`0e0ac7e`](https://github.com/egrieco/eunicode/commit/0e0ac7e207e004834845b5d2ce49b4b154dd7a73))
    - Auto-fix clippy lints ([`0604fd2`](https://github.com/egrieco/eunicode/commit/0604fd298fa3ccd209111de03e02ef7748849a63))
    - Reorganize bin and lib ([`ee51c1d`](https://github.com/egrieco/eunicode/commit/ee51c1d930cbdbafe2026ca129785a4882fd59b7))
    - Add nix shell and rust toolchain ([`f280e6e`](https://github.com/egrieco/eunicode/commit/f280e6ef0ee916a72d04ab69f4fc6b1aed2ef7c9))
</details>

## v0.1.1 (2025-06-23)

<csr-id-ababa478bd90223fdf6fa9e2f43fcfd11eeb3109/>
<csr-id-5e2117cb7e1527143fb7f80cec8d7176a02bfe19/>
<csr-id-d3c9daed2e2ef6bbbe292464fd456d7e491666f2/>
<csr-id-e75258242d21ce8dd41ab310b1bae718fd05a71a/>
<csr-id-0651576cb720f0b77aab79038bcede217247d6c7/>
<csr-id-ed7457e64be109afd0adce4ac8e6d13b01fc7d77/>

### New Features

 - <csr-id-4df90f3b40868cce888230a9560bac4bece1bdad/> Add Unicode script
 - <csr-id-ba52a3dd47ba2d04f07b709db9322d11f0364f71/> Add Unicode block
 - <csr-id-441173a5fd86509444e6ffca5ac6cf9dc6b56f80/> add unicode block display to character analysis table
 - <csr-id-cf4ad8dfe520c457c25c8ddeda7d2c60c9a610f9/> Add table based layout
 - <csr-id-788a18695ef7686601e0dec18c6c767267591d62/> Add unicode UCD category
   Also dry out some of the info code.

### Refactor

 - <csr-id-ababa478bd90223fdf6fa9e2f43fcfd11eeb3109/> Improve return types
 - <csr-id-5e2117cb7e1527143fb7f80cec8d7176a02bfe19/> Move type conversion to lib
 - <csr-id-d3c9daed2e2ef6bbbe292464fd456d7e491666f2/> Move category conversion to lib
 - <csr-id-e75258242d21ce8dd41ab310b1bae718fd05a71a/> move `unicode_block_to_string` function to `lib.rs`
 - <csr-id-0651576cb720f0b77aab79038bcede217247d6c7/> Switch to UCD crate
 - <csr-id-ed7457e64be109afd0adce4ac8e6d13b01fc7d77/> Improve efficiency of row creation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release.
 - 11 days passed between releases.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release eunicode v0.1.1 ([`864978e`](https://github.com/egrieco/eunicode/commit/864978e362244c0d48b83f14a74b0bc29264a042))
    - Improve return types ([`ababa47`](https://github.com/egrieco/eunicode/commit/ababa478bd90223fdf6fa9e2f43fcfd11eeb3109))
    - Move type conversion to lib ([`5e2117c`](https://github.com/egrieco/eunicode/commit/5e2117cb7e1527143fb7f80cec8d7176a02bfe19))
    - Move category conversion to lib ([`d3c9dae`](https://github.com/egrieco/eunicode/commit/d3c9daed2e2ef6bbbe292464fd456d7e491666f2))
    - Add Unicode script ([`4df90f3`](https://github.com/egrieco/eunicode/commit/4df90f3b40868cce888230a9560bac4bece1bdad))
    - Add Unicode block ([`ba52a3d`](https://github.com/egrieco/eunicode/commit/ba52a3dd47ba2d04f07b709db9322d11f0364f71))
    - Move `unicode_block_to_string` function to `lib.rs` ([`e752582`](https://github.com/egrieco/eunicode/commit/e75258242d21ce8dd41ab310b1bae718fd05a71a))
    - Add unicode block display to character analysis table ([`441173a`](https://github.com/egrieco/eunicode/commit/441173a5fd86509444e6ffca5ac6cf9dc6b56f80))
    - Switch to UCD crate ([`0651576`](https://github.com/egrieco/eunicode/commit/0651576cb720f0b77aab79038bcede217247d6c7))
    - Improve efficiency of row creation ([`ed7457e`](https://github.com/egrieco/eunicode/commit/ed7457e64be109afd0adce4ac8e6d13b01fc7d77))
    - Add table based layout ([`cf4ad8d`](https://github.com/egrieco/eunicode/commit/cf4ad8dfe520c457c25c8ddeda7d2c60c9a610f9))
    - Add unicode UCD category ([`788a186`](https://github.com/egrieco/eunicode/commit/788a18695ef7686601e0dec18c6c767267591d62))
</details>

## v0.1.0 (2025-06-11)

<csr-id-f811a245a70c313fdd10d685f993c6d7d89a4272/>
<csr-id-0a3c60df9e0e49c7be124074bc1e485c1775bc16/>
<csr-id-b3558d21e059c4f4af36966164aaa9807a1519a9/>
<csr-id-b7bbe970e39ad095c5af16f4a32aceb173cdaa57/>

### Chore

 - <csr-id-f811a245a70c313fdd10d685f993c6d7d89a4272/> Update manifest fields

### Chore

 - <csr-id-b7bbe970e39ad095c5af16f4a32aceb173cdaa57/> Remove excess keywords
   Apparently crates.io "expects at most 5 keywords per crate"

### Chore

 - <csr-id-b3558d21e059c4f4af36966164aaa9807a1519a9/> Add changelog
   Add changelog auto-generated by `cargo-smart-release` via `git-cliff`.

### New Features

 - <csr-id-7e001f8fba68182d97ed5b33e86653a9e11a834c/> Relax dangerous character detection
   We were catching lots of safe characters. Though now we might not be strict enough. Some non-ASCII characters seem to have made it past detection.
 - <csr-id-19c9156183bdf1e8658b7cb56d337267474bc3be/> Always clean text if not analyzing
 - <csr-id-a68fff04274c85022647a83e4c87c9cd5a94a83f/> Show info for all input characters
 - <csr-id-cce08a294bbbcd5b8dc8ac9709d1e773735fa6f6/> Add extended character info display
 - <csr-id-d4cba2da179042b6c81776bea76801967ca06ad5/> Enable detection of dangerous characters
 - <csr-id-1f45e7cff18b9ece15e4796d7c10c886009a7856/> Enable slugifying text for URLs or filenames
 - <csr-id-b7dc63526c23d707744bbd0d1be67f3724fbc590/> Enable censoring profanity
 - <csr-id-48ff3fe9dde5d77fa0c543278c7de3deff64f8e2/> Enable defanging of URLs
 - <csr-id-f49187b0b504140a53f8be60d815503971614156/> Enable stripping HTML tags
 - <csr-id-44679d20941bd7017fdf61e34906bf053a917ecf/> Add proper newtype pattern (with err)
 - <csr-id-4fe8f214a5b9220f5be16129249434d3042de14f/> Avoid printing unsanitized data
 - <csr-id-c516e3a22ebd8bb8c9d489f3e6e681feb50cfe94/> Implement TypeState pattern for text processing with RawInput and CleanedText states
 - <csr-id-46adb89ac56acac6586a96202b8c1d6c4bbe6c85/> Add CLI application structure for Unicode character utility

### Bug Fixes

 - <csr-id-537d9405ba863f8586f3a2a5155185feedf078ce/> Remove a disallowed function call
 - <csr-id-a22a1bb439bb1eb69a3600d1a6b2ebec9f306d58/> Add missing functionality
   Not sure why the functionality showing in "architect" mode wasn't added to the main file, but it was still in the chat history so we're adding it manually.

### Refactor

 - <csr-id-0a3c60df9e0e49c7be124074bc1e485c1775bc16/> Adjust text cleaning flow
   Also hooked up deunicode crate

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release over the course of 1 calendar day.
 - 19 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release eunicode v0.1.0 ([`20f4346`](https://github.com/egrieco/eunicode/commit/20f4346da10ddec2d48432c42eacc5d9b5aabead))
    - Remove excess keywords ([`b7bbe97`](https://github.com/egrieco/eunicode/commit/b7bbe970e39ad095c5af16f4a32aceb173cdaa57))
    - Release eunicode v0.1.0 ([`7f52da4`](https://github.com/egrieco/eunicode/commit/7f52da45218edf3ce8713089e0f906afafbe52e3))
    - Add changelog ([`b3558d2`](https://github.com/egrieco/eunicode/commit/b3558d21e059c4f4af36966164aaa9807a1519a9))
    - Update manifest fields ([`f811a24`](https://github.com/egrieco/eunicode/commit/f811a245a70c313fdd10d685f993c6d7d89a4272))
    - Relax dangerous character detection ([`7e001f8`](https://github.com/egrieco/eunicode/commit/7e001f8fba68182d97ed5b33e86653a9e11a834c))
    - Always clean text if not analyzing ([`19c9156`](https://github.com/egrieco/eunicode/commit/19c9156183bdf1e8658b7cb56d337267474bc3be))
    - Show info for all input characters ([`a68fff0`](https://github.com/egrieco/eunicode/commit/a68fff04274c85022647a83e4c87c9cd5a94a83f))
    - Add extended character info display ([`cce08a2`](https://github.com/egrieco/eunicode/commit/cce08a294bbbcd5b8dc8ac9709d1e773735fa6f6))
    - Enable detection of dangerous characters ([`d4cba2d`](https://github.com/egrieco/eunicode/commit/d4cba2da179042b6c81776bea76801967ca06ad5))
    - Enable slugifying text for URLs or filenames ([`1f45e7c`](https://github.com/egrieco/eunicode/commit/1f45e7cff18b9ece15e4796d7c10c886009a7856))
    - Enable censoring profanity ([`b7dc635`](https://github.com/egrieco/eunicode/commit/b7dc63526c23d707744bbd0d1be67f3724fbc590))
    - Enable defanging of URLs ([`48ff3fe`](https://github.com/egrieco/eunicode/commit/48ff3fe9dde5d77fa0c543278c7de3deff64f8e2))
    - Enable stripping HTML tags ([`f49187b`](https://github.com/egrieco/eunicode/commit/f49187b0b504140a53f8be60d815503971614156))
    - Adjust text cleaning flow ([`0a3c60d`](https://github.com/egrieco/eunicode/commit/0a3c60df9e0e49c7be124074bc1e485c1775bc16))
    - Remove a disallowed function call ([`537d940`](https://github.com/egrieco/eunicode/commit/537d9405ba863f8586f3a2a5155185feedf078ce))
    - Add proper newtype pattern (with err) ([`44679d2`](https://github.com/egrieco/eunicode/commit/44679d20941bd7017fdf61e34906bf053a917ecf))
    - Avoid printing unsanitized data ([`4fe8f21`](https://github.com/egrieco/eunicode/commit/4fe8f214a5b9220f5be16129249434d3042de14f))
    - Implement TypeState pattern for text processing with RawInput and CleanedText states ([`c516e3a`](https://github.com/egrieco/eunicode/commit/c516e3a22ebd8bb8c9d489f3e6e681feb50cfe94))
    - Add missing functionality ([`a22a1bb`](https://github.com/egrieco/eunicode/commit/a22a1bb439bb1eb69a3600d1a6b2ebec9f306d58))
    - Add CLI application structure for Unicode character utility ([`46adb89`](https://github.com/egrieco/eunicode/commit/46adb89ac56acac6586a96202b8c1d6c4bbe6c85))
    - Add aider ignores ([`93a294c`](https://github.com/egrieco/eunicode/commit/93a294c9e888394b02856f3b398b54b8e25ecc40))
    - Add AI guidance ([`d34964c`](https://github.com/egrieco/eunicode/commit/d34964c26e709f3fd86e8025e2edb76db1a01139))
    - Add design docs ([`04aedb2`](https://github.com/egrieco/eunicode/commit/04aedb2e7c60f858b6d41e2b60d4afdcfdd16f53))
    - Add initial Cargo template ([`bd8d7dd`](https://github.com/egrieco/eunicode/commit/bd8d7dd89b0378ea23f2c7c95702a1b1f30ed07d))
    - Initial Commit ([`d1912d6`](https://github.com/egrieco/eunicode/commit/d1912d66493538fb626dee876b0e38754c9d7cb1))
</details>

