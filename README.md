# Website Sequence Finder

This is a simple Rust command-line program that reads a list of websites from a CSV file, fetches the HTML content of each website, and checks for the presence of specific sequences of text provided in a JSON configuration file.

## Directory Structure

This program requires a directory that contains the following two files:

1. `websites.csv`: A CSV file with one column that contains the URLs of the websites to be checked.

2. `config.json`: A JSON configuration file that specifies the sequences of text to be searched for on the websites. The sequences should be provided as an array, for example:

```json
{
    "sequences": [
        "sequence1",
        "sequence2"
    ]
}
```

## Setting up Rust

1. Download and install Rust using `rustup`, which is a toolchain installer for the Rust programming language. You can install `rustup` by running the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will also download and install the latest stable version of Rust, along with its package manager `cargo`.

2. Add the `cargo` bin directory to your `PATH`. This is usually done automatically by the `rustup` installer.

3. Verify your installation by running the following command:

```bash
rustc --version
```

This should print the installed version of Rust.

## Building the Program

Navigate to the program directory and run the following command to build the program:

```bash
cargo build --release
```

This will create an optimized executable in the `target/release` directory.

## Running the Program

After building the program, you can run it by providing the path to the directory that contains the `websites.csv` and `config.json` files as an argument:

```bash
./target/release/site_parser /path/to/directory
```

The program will write its output to an `output.csv` file in the same directory.
```
Note: It's recommended to create a backup of your data before running the program, especially if the data is not easily recoverable.