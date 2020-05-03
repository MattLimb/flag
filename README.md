# flag

Show LGBTQ flags in your terminal! 

**NOTE:** This project may have updates. It may not.

The only supported platform is Linux for now. This program may work on other Unix like operating systems, however, has not been tested. Windows is not showing colours properly. This may work on the Windows Subsystem for Linux, although this has not been tested. 

Usage:

```bash
flag <flag_name>
```

Supported replacements for `<flag_name>` is:

| Flag Value   | Description                       |
|:------------:|:--------------------------------- |
| list         | Show the list of flag values      |
| lgbtq        | Show the LGBTQ 6 colour flag      |
| agender      | Show the agender flag             |
| aromantic    | Show the aromantic flag           |
| asexual      | Show the asexual flag             |
| bisexual     | Show the bisexual flag            |
| demisexual   | Show the demisexual flag          |
| genderfluid  | Show the genderfluid flag         |
| genderqueer  | Show the genderqueer flag         |
| intersex     | Show the intersex flag            |
| lesbian      | Show the traditional lesbian flag |
| lesbian-comm | Show the lesbian community flag   |
| non-binary   | Show the non-binary flag          |
| pansexual    | Show the pansexual flag           |
| polysexual   | Show the polysexual flag          |
| transgender  | Show the transgender flag         |

### Building from source

This project is built in rust-lang. To build from source the Rust compiler will be needed. Please visit the official rust-lang webpage to install for your distribution here: [Install Rust](https://www.rust-lang.org/tools/install)

This guide assumes you are in your home directory.

1. Clone the repository
2. Change into the flag directory: `cd flag`
3. Build the program using the Rust tool Cargo: `cargo build --release`
4. Then to run: `./target/release/flag <flag_name>`

Optional instructions to install onto system:

1. Use cargo to install the program: `cargo install --path ~/flag`
   
   **Replace `~/flag` with the path to the flag directory, if it was not cloned directly into your home folder.**
