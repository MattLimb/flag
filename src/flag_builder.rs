extern crate yaml_rust;

use colored_truecolor::*;
use yaml_rust::{YamlLoader, Yaml};
use std::fs::File;
use std::io::prelude::*;

pub fn find_config(command: &str) -> Vec<Yaml> {
    let mut empty_vector: Vec<Yaml> = Vec::new();
    let mut flag_file = File::open(format!("flags.d/{}.yaml", &command));
    
    match flag_file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents);
            
            return YamlLoader::load_from_str(&contents).unwrap_or(empty_vector);
        }
        Err(_) => return empty_vector,
    }
}

pub fn build_flag(flag_docs: Vec<Yaml>) {
    let flag_config = &flag_docs[0];

    match flag_config["style"].as_str().unwrap() {
        "rows" => { 
            println!("\n{}", rows(flag_config["colours"].as_vec().unwrap(),
                flag_config["width"].as_i64().unwrap_or(70),
                flag_config["height"].as_i64().unwrap_or(18)));
                
        },
        _ => println!("error: unsupported flag type"),
    }

} 

pub fn get_row(width: i64) -> String {
    let mut row = String::new();
    for _ in 0..width {
        row.push_str(" ");
    }
    return row
}

fn rows(colours: &Vec<Yaml>, width: i64, height: i64) -> String {
    let num_rows = colours.len() as i64;

    let rows_per_colour = height / num_rows;
    let mut flag: String = "\n".to_string();

    for colour in colours.iter() {
        let the_colour = match colour.as_str() {
            Some(s) => u64::from_str_radix(s, 16).unwrap(),
            None => u64::from_str_radix(&colour.as_i64().unwrap().to_string(), 16).unwrap(),
        };

        for _ in 0..rows_per_colour {
            flag.push_str(&format!(" {}\n", get_row(width).on_hex_color(the_colour)));
        }
    }
    return flag;
}