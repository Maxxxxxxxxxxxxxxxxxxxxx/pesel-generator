#![allow(unused)]

pub const YEAR_MIN: i32 = 1900;
pub const YEAR_MAX: i32 = 2100;

use rand::thread_rng;
use rand::Rng;
use regex::Regex;
use std::fmt::Display;

fn rng_digits(length: i32) -> String {
    let mut rng = thread_rng();
    let mut vec = Vec::<char>::new();

    for i in 0..length {
        let num = char::from_digit(rng.gen_range(0..10), 10).unwrap();
        vec.push(num)
    }

    vec.into_iter().collect()
}

fn get_control_number(incomplete_pesel: String) -> u32 {
    let multipliers = [1,3,7,9,1,3,7,9,1,3];

    let mut control_num_checksum: u32 = 0;
    let mut index = 0;

    for ch in incomplete_pesel.chars() {
        control_num_checksum += char::to_digit(ch, 10).unwrap() * multipliers[index];
        index += 1;
    }

    match control_num_checksum % 10 {
        0 => 0,
        _ => 10 - control_num_checksum % 10
    }
}

#[derive(Debug, PartialEq)]
pub struct Date {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

// impl PartialOrd for Date {
//     fn gt(&self, other: &Self) -> bool {
//         let days_count = (self.day + self.month * 30);
//     }
// }

impl Date {
    pub fn new(day: i32, month: i32, year: i32) -> Self {
        Date {
            day,
            month,
            year
        }
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();

        let month = rng.gen_range(1..12);
        let year = rng.gen_range(YEAR_MIN..YEAR_MAX);
        let day = match (month, year) {
            (2, year) => {
                if year % 4 == 0 {
                    rng.gen_range(1..29)
                } else {
                    rng.gen_range(1..28)
                }
            }
            (month, year) => {
                if month < 8 {
                    if month % 2 == 0 {
                        rng.gen_range(1..30)
                    } else {
                        rng.gen_range(1..31)
                    }
                } else {
                    if month % 2 == 0 {
                        rng.gen_range(1..31)
                    } else {
                        rng.gen_range(1..30)
                    }
                }
            }
            _ => unreachable!(),
        };

        Date::new(day, month, year)
    }
}

impl From<String> for Date
{
    fn from(string: String) -> Self {
        let mut collect = string.split("-");
        Date {
            day:   i32::from_str_radix(collect.next().unwrap(), 10).unwrap(),
            month: i32::from_str_radix(collect.next().unwrap(), 10).unwrap(),
            year:  i32::from_str_radix(collect.next().unwrap(), 10).unwrap()
        }
    }
}

impl From<&str> for Date
{
    fn from(string: &str) -> Self {
        let mut collect = string.split("-");
        Date {
            day:   i32::from_str_radix(collect.next().unwrap(), 10).unwrap(),
            month: i32::from_str_radix(collect.next().unwrap(), 10).unwrap(),
            year:  i32::from_str_radix(collect.next().unwrap(), 10).unwrap()
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day_fmt: String;
        let month_fmt: String;

        if self.day.to_string().len() == 1 {
            day_fmt = format!("0{}", &self.day)
        } else {
            day_fmt = self.day.to_string()
        }

        if self.month.to_string().len() == 1 {
            month_fmt = format!("0{}", &self.month)
        } else {
            month_fmt = self.month.to_string()
        }

        write!(f, "{}-{}-{}", day_fmt, month_fmt, self.year)
    }
}

#[derive(Debug)]
pub struct PeselNumber {
    pub value: String,
    birth_date: Date,
}

impl PeselNumber {
    pub fn rand() -> Self {
        let birthdate = Date::random();
        PeselNumber {
            value: PeselNumber::construct_pesel(Date::new(
                birthdate.day,
                birthdate.month,
                birthdate.year,
            )),
            birth_date: birthdate,
        }
    }
    pub fn from_date(date_str: &str) -> Self {
        let birthdate = Date::from(date_str);
        PeselNumber { value: Self::construct_pesel(birthdate), birth_date: Date::from(date_str) }
    }
    fn construct_pesel(birthdate: Date) -> String {
        let first_digit_pair: String = birthdate.year.to_string().chars().skip(2).collect();
        let second_digit_pair = if birthdate.year >= 2000 {
            format!("{}", 20 + birthdate.month)
        } else {
            match birthdate.month.to_string().len() {
                1 => format!("0{}", birthdate.month),
                2 => birthdate.month.to_string(),
                _ => unreachable!(),
            }
        };
        let third_digit_pair = match birthdate.day.to_string().len() {
            1 => format!("0{}", birthdate.day),
            2 => birthdate.day.to_string(),
            _ => unreachable!(),
        };
        let rng_digits = rng_digits(4);

        let control_number = get_control_number(format!(
            "{}{}{}{}",
            first_digit_pair, second_digit_pair, third_digit_pair, rng_digits
        ));

        format!(
            "{}{}{}{}{}",
            first_digit_pair, 
            second_digit_pair, 
            third_digit_pair, 
            rng_digits, 
            control_number
        )
    }
}

impl Display for PeselNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
