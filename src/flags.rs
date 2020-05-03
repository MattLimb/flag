use colored_truecolor::*;

pub fn row(width: i32) -> String {
    let mut row: String = "".to_string();
    for _ in 0..width {
        row = format!("{}{}", row, " ");
    };
    return row
}

// Flags
pub fn lgbtq() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["FF0018", "FFA52C", "FFFF41", "008018", "0000F9", "86007D"];   
    for col in cols {
        for _ in 0..3 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }
    return flag
}

pub fn agender() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["000000", "BABABA", "FFFFFF", "BAF584", "FFFFFF", "BABABA", "000000"];   
    for col in cols {
        for _ in 0..3 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }
    return flag
}

pub fn aromantic() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["3AA63F", "A8D47A", "FFFFFF", "AAAAAA", "000000"];   
    for col in cols {
        for _ in 0..4 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }
    return flag
}

pub fn asexual() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["000000", "A4A4A4", "FFFFFF", "810081"];   
    for col in cols {
        for _ in 0..5 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }
    return flag
}

pub fn bi() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["D60270", "9B4F96", "0038A8"];   
    
    for _ in 0..7 {
        flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));
    }

    for _ in 0..4 {
        flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()));
    }

    for _ in 0..7 {
        flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap()));
    }

    return flag
}

pub fn demi() -> String {
    let mut flag: String = "\n".to_string();
    let mut count: i32 = 0;
    let cols: Vec<&str> = vec!["000000", "FFFFFF", "A4A4A4", "810081"];   

    for _ in 0..8 {
        match count {
            0 => flag = format!("{} {}{}\n", flag, row(4).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(71).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            1 => flag = format!("{} {}{}\n", flag, row(6).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(69).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            2 => flag = format!("{} {}{}\n", flag, row(8).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(67).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            3 => flag = format!("{} {}{}\n", flag, row(10).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(65).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            4 => flag = format!("{} {}{}\n", flag, row(12).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(63).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            5 => flag = format!("{} {}{}\n", flag, row(14).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(61).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            6 => flag = format!("{} {}{}\n", flag, row(16).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(59).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap())),
            7 => flag = format!("{} {}{}\n", flag, row(18).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(57).on_hex_color(u64::from_str_radix(cols[3], 16).unwrap())),
            _ => flag = format!("{} {}{}\n", flag, row(20).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(55).on_hex_color(u64::from_str_radix(cols[3], 16).unwrap())),
        }
        count = count+1;
    }

    for _ in 0..9 {
        match count {
            0 => flag = format!("{} {}{}\n", flag, row(4).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(71).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            1 => flag = format!("{} {}{}\n", flag, row(6).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(69).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            2 => flag = format!("{} {}{}\n", flag, row(8).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(67).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            3 => flag = format!("{} {}{}\n", flag, row(10).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(65).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            4 => flag = format!("{} {}{}\n", flag, row(12).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(63).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            5 => flag = format!("{} {}{}\n", flag, row(14).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(61).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            6 => flag = format!("{} {}{}\n", flag, row(16).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(59).on_hex_color(u64::from_str_radix(cols[2], 16).unwrap())),
            7 => flag = format!("{} {}{}\n", flag, row(18).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(57).on_hex_color(u64::from_str_radix(cols[3], 16).unwrap())),
            _ => flag = format!("{} {}{}\n", flag, row(20).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(55).on_hex_color(u64::from_str_radix(cols[3], 16).unwrap())),
        }
        count = count-1;
    }

    return flag
}

pub fn genderfluid() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["FF76A4", "FFFFFF", "C011D7", "000000", "2F3CBE"];   
    for col in cols {
        for _ in 0..4 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }
    return flag
}

pub fn genderqueer() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["B77FDD", "FFFFFF", "48821E"];   
    for col in cols {
        for _ in 0..6 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }
    return flag
}

pub fn intersex() -> String {
    let mut flag: String = "\n".to_string();
    let cols: Vec<&str> = vec!["FFDA00", "7A00AC"];   

    for _ in 0..4 {
        flag = format!("{} {}\n", flag, row(60).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));
    }

    flag = format!("{} {}{}{}\n", flag, row(25).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(10).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()), row(25).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));
    flag = format!("{} {}{}{}{}{}\n", flag, 
        row(23).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()),
        row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()),
        row(10).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()),
        row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()),
        row(23).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));

    flag = format!("{} {}{}{}{}{}\n", flag, 
        row(21).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), 
        row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()),
        row(14).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), 
        row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()), 
        row(21).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));

    for _ in 0..4 {
        flag = format!("{} {}{}{}{}{}\n", flag, 
            row(19).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), 
            row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()),
            row(18).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), 
            row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()), 
            row(19).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));
    }
    
    flag = format!("{} {}{}{}{}{}\n", flag, 
        row(21).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), 
        row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()),
        row(14).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), 
        row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()), 
        row(21).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));

    flag = format!("{} {}{}{}{}{}\n", flag, 
        row(23).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()),
        row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()),
        row(10).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()),
        row(2).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()),
        row(23).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));
    
    flag = format!("{} {}{}{}\n", flag, row(25).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()), row(10).on_hex_color(u64::from_str_radix(cols[1], 16).unwrap()), row(25).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));
    
    for _ in 0..4 {
        flag = format!("{} {}\n", flag, row(60).on_hex_color(u64::from_str_radix(cols[0], 16).unwrap()));
    }   


    return flag
}

pub fn lesbian() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["A60061", "B95594", "D162A8", "E5ADD1", "C64D53", "8C1801"];   
    
    for col in cols {
        for _ in 0..3 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }

    return flag
}

pub fn lesbian_comm() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["D62900", "FF9B55", "FFFFFF", "D461A6", "A50062"];   
    
    for col in cols {
        for _ in 0..4 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }

    return flag
}

pub fn non_binary() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["FFF430", "FFFFFF", "9C59D1", "000000"];   
    
    for col in cols {
        for _ in 0..4 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }

    return flag
}

pub fn pan() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["FF1B8D", "FFDA00", "1BB3FF"];   
    
    for col in cols {
        for _ in 0..6 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    } 

    return flag
}

pub fn polysexual() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["F714BA", "01D66A", "1594F6"];   
    
    for col in cols {
        for _ in 0..6 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    } 

    return flag
}

pub fn trans() -> String {
    let mut flag: String = "\n".to_string();
    
    let cols: Vec<&str> = vec!["55CDFC", "F7A8B8", "FFFFFF", "F7A8B8", "55CDFC"];   
    
    for col in cols {
        for _ in 0..3 {
            flag = format!("{} {}\n", flag, row(75).on_hex_color(u64::from_str_radix(col, 16).unwrap()));
        }
    }

    return flag
}

