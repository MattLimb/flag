use clap::{App, Arg};
use std::fs::{self};
use std::path::Path;


mod flags;
mod flag_builder;

fn list() {
    let mut flags: Vec<String> = Vec::new();
    let mut max_len = 10;

    for entry in fs::read_dir(Path::new("flags.d/")).unwrap() {
        match entry {
            Ok(e) => {
                let mut filename = e.file_name().to_str().unwrap().to_string();

                filename = filename[0..filename.find('.').unwrap()].to_string();

                if max_len < filename.len() {
                    max_len = filename.len();
                }

                flags.push(filename);
            },
            Err(_) => println!("Error with file")
        }
    }

    println!("\n Flag Value{}Flag Description", flag_builder::get_row((max_len-8) as i64));
    println!(" --------------------------------------------------");
    for flag in flags {
        println!(" {}{}Show the {} flag.", &flag, flag_builder::get_row(((max_len-&flag.len())+2) as i64), &flag);
    }

    println!(" demisexual{}[Built-In] Show the demisexual flag.", flag_builder::get_row(((max_len-10)+2) as i64));
    println!(" intersex{}[Built-In] Show the intersex flag.", flag_builder::get_row(((max_len-8)+2) as i64));
}
fn main() {
    let cmd_args = App::new("Terminal Flags")
                .version("2.0.0")
                .author("Matt Limb <matt.limb17@gmail.com>")
                .about("Print coloured flags in the terminal.")
                .arg(Arg::with_name("FLAG")
                    .value_name("FLAG")
                    .help("Name of the flag to show.")
                    .takes_value(true)
                )
            .get_matches();
            
    let flag_config = flag_builder::find_config(&cmd_args.value_of("FLAG").unwrap_or("lgbtq"));
                
    match flag_config.len() {
        0 => { 
            match cmd_args.value_of("FLAG").unwrap_or("lgbtq") {
                "demisexual" => println!("{}", flags::demi()),
                "intersex" => println!("{}", flags::intersex()),
                "list" => list(),
                other => println!("No flag by the name: {}. Please choose a valid flag.", other),
            };
        },
        _ => flag_builder::build_flag(flag_config),
    }

    
}