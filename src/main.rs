use colored_truecolor::*;
use clap::{App, Arg};

fn basic_row(row_count:i32) -> String {
    let mut basic_row: String = "".to_string();
    for _ in 0..row_count {
        basic_row = format!("{}{}", basic_row, " ");
    };
    return basic_row
}

fn lgbtq() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["FF0018", "FFA52C", "FFFF41", "008018", "0000F9", "86007D"];   
    for col in cols {
        for _ in 0..3 {
            flag = format!("{} {}\n", flag, basic_row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    } 

    return flag
}

fn bi() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["D60270", "9B4F96", "0038A8"];   
    
    for _ in 0..7 {
        flag = format!("{} {}\n", flag, basic_row(75).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));
    }

    for _ in 0..4 {
        flag = format!("{} {}\n", flag, basic_row(75).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()));
    }

    for _ in 0..7 {
        flag = format!("{} {}\n", flag, basic_row(75).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap()));
    }

    return flag
}

fn pan() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["FF1B8D", "FFDA00", "1BB3FF"];   
    
    for col in cols {
        for _ in 0..6 {
            flag = format!("{} {}\n", flag, basic_row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    } 

    return flag
}

fn trans() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["55CDFC", "F7A8B8", "FFFFFF", "F7A8B8", "55CDFC"];   
    
    for col in cols {
        for _ in 0..3 {
            flag = format!("{} {}\n", flag, basic_row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }

    return flag
}

fn lesbian() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["A60061", "B95594", "D162A8", "E5ADD1", "C64D53", "8C1801"];   
    
    for col in cols {
        for _ in 0..3 {
            flag = format!("{} {}\n", flag, basic_row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }

    return flag
}

fn demi() -> String {
    let mut flag: String = "\n".to_string();
    let mut count: i32 = 0;
    let cols: Vec<&str> = vec!["000000", "FFFFFF", "A4A4A4", "810081"];   

    for _ in 0..8 {
        match count {
            0 => flag = format!("{} {}{}\n", flag, basic_row(4).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(71).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            1 => flag = format!("{} {}{}\n", flag, basic_row(6).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(69).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            2 => flag = format!("{} {}{}\n", flag, basic_row(8).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(67).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            3 => flag = format!("{} {}{}\n", flag, basic_row(10).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(65).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            4 => flag = format!("{} {}{}\n", flag, basic_row(12).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(63).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            5 => flag = format!("{} {}{}\n", flag, basic_row(14).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(61).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            6 => flag = format!("{} {}{}\n", flag, basic_row(16).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(59).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            7 => flag = format!("{} {}{}\n", flag, basic_row(18).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(57).on_hex_color(u64::from_str_radix(cols[3], 16).unwrap())),
            _ => flag = format!("{} {}{}\n", flag, basic_row(20).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(55).on_hex_color(u64::from_str_radix(cols[3], 16).unwrap())),
        }
        count = count+1;
    }

    for _ in 0..9 {
        match count {
            0 => flag = format!("{} {}{}\n", flag, basic_row(4).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(71).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            1 => flag = format!("{} {}{}\n", flag, basic_row(6).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(69).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            2 => flag = format!("{} {}{}\n", flag, basic_row(8).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(67).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            3 => flag = format!("{} {}{}\n", flag, basic_row(10).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(65).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            4 => flag = format!("{} {}{}\n", flag, basic_row(12).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(63).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            5 => flag = format!("{} {}{}\n", flag, basic_row(14).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(61).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            6 => flag = format!("{} {}{}\n", flag, basic_row(16).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(59).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            7 => flag = format!("{} {}{}\n", flag, basic_row(18).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(57).on_hex_color(u64::from_str_radix(cols[3], 16).unwrap())),
            _ => flag = format!("{} {}{}\n", flag, basic_row(20).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), basic_row(55).on_hex_color(u64::from_str_radix(cols[3], 16).unwrap())),
        }
        count = count-1;
    }

    return flag
}
fn main() {
    let cmd_args = App::new("Terminal Flags")
                .version("0.5")
                .author("Matthew Limb <matt.limb17@gmail.com>")
                .about("Print coloured flags in the terminal.")
                .arg(Arg::with_name("FLAG")
                    .value_name("FLAG")
                    .help("Name of the flag to show. \nValid names:\n> lgbtq\n> bi\n> pan\n> lesbian\n> trans\n> demi")
                    .takes_value(true)
                )
            .get_matches();

    match cmd_args.value_of("FLAG").unwrap_or("lgbtq") {
        "lgbtq" => println!("{}", lgbtq()),
        "bi" => println!("{}", bi()),
        "trans" => println!("{}", trans()),
        "pan" => println!("{}", pan()),
        "lesbian" => println!("{}", lesbian()),
        "demi" => println!("{}", demi()),
        other => println!("No flag by the name: {}. Please choose a valid flag.", other),
    }
}