# Simple Morphological Analyzer for Polish
Based on ispell dictionaries from [sjp.pl](https://sjp.pl/sl/ort/)

Inspired by [Morfeusz](https://morfeusz.sgjp.pl/)

> [!CAUTION]
> This program is a work in progress. It currently does not validate regex rules before applying affixes, which makes the results unreliable.

## Current Program Flow
1. Load the dictionary (words and their associated rules) from the `pl_PL.dic` file into an internal HashMap.
2. Load the affix rules from the `pl_PL.aff` file.
3. Find the rules connected to the word provided by the user.
4. Apply the rules to the word using data from the `.aff` file (NOT FINISHED!).

## Usage
```bash
git clone https://github.com/karoljarzabek0/morfeusz-rust
cd morfeusz-rust
cargo build --release
```

Run the program with:
```bash
./target/release/morfeusz-rust test # Example query for a word "test"
```
Output:
```bash
testom
tesom
teciom
teniom
testm
tesiom
tesksom
teziom
...
```