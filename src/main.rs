#![allow(unused)]

mod pesel;

use std::fs::File;
use std::result::Result;
use std::env;
use pesel::PeselNumber;

const USAGE_STR: &'static str = "usage: pesel [-r | random <count>] [-h | --help]";

fn help() {
    println!("{USAGE_STR}")
}

fn unrecognized(arg: &mut String) {
    match arg.as_str() {
        "" => {
            println!("{USAGE_STR}");
        },
        _ => {
            let illegal_cmd_name = if arg.chars().next().unwrap() == '-' && arg.len() > 1 {
                arg.remove(0);
                arg
            } else {
                arg
            };
            println!("pesel: illegal option -- {}", illegal_cmd_name.trim());
            println!("{USAGE_STR}");
        }    
    }
}

fn output(args: &mut env::Args) {
    let mut last_arg = args
        .next()
        .unwrap_or(String::from("1"));

    let count = i32::from_str_radix(&last_arg, 10);

    match count {
        Ok(c) => {
            let output: Vec<String> = (0..c)
                .map(|elem| format!("{}", PeselNumber::rand()))
                .collect();
        
            println!("{}", output.join("\n"));
        },
        Err(_) => {
            unrecognized(&mut last_arg);
        }
    }

}

fn write_file(path: &str) -> Result<(), ()> {
    let mut file = File::create(path);
    Ok(())
}

fn main() {
    let mut args = env::args();
    args.next();
    
    // flags handling
    match &mut args.next() {
        Some(arg) => {
            match arg.as_str() {
                "-r" | "--random" => output(&mut args),
                "-h" | "--help"   => help(),
                _    => unrecognized(arg)
            }
        },
        None => {
            unrecognized(&mut String::from(""));
        }
    }  
} 
