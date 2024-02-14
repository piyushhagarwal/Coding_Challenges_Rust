# Build Your Own wc Tool: ccwc - Character, Word, and Line Counter

[Link to the challenge](https://codingchallenges.fyi/challenges/challenge-wc/)

`ccwc` is a simple command-line tool written in Rust that counts the number of characters, words, and lines in a text file.

### Create new project

```bash
$ cargo new ccwc
$ cd ccwc
```

Use the test file `test.txt` to test the program.

### To run the program

## 1. Count the number of bytes in a file

```bash
$ cargo run -- -c test.txt
```

The output should be `   2439 test.txt`

## 2. Count the number of words in a file

```bash
$ cargo run -- -w test.txt
```

The output should be `   339 test.txt`

## 3. Count the number of lines in a file

```bash
$ cargo run -- -l test.txt
```

The output should be `    23 test.txt`

## 4. Count all the above in a file

```bash
$ cargo run -- test.txt
```

The output should be `    23 339 2439 test.txt`

### To run the tests

```bash
$ cargo test
```
