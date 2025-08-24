mod chapter_three {
    pub mod fahrenheight_to_celsius;
    pub mod fibonacci_number;
    pub mod christmas_carol;
}

use std::io;
use chapter_three::fahrenheight_to_celsius::fahrenheit_to_celsius;
use chapter_three::fibonacci_number::calc_n_fibonacci_number;
use chapter_three::christmas_carol::print_christmas_carol;

fn main() {
    println!("Chapter 3");
    println!("===================");
    println!("Fahrenheight to Celcius Temperature Conversion");
    let mut fahrenheight_temp = String::new();

    io::stdin()
            .read_line(&mut fahrenheight_temp)
            .expect("Failed to read line");
    
    let fahrenheight_temp: f64 = fahrenheight_temp.trim().parse().expect("Invalid number");
    let celcius_temp = fahrenheit_to_celsius(fahrenheight_temp);
    println!("Temperature in Fahrenheight {fahrenheight_temp} and Temperature in Celcius {celcius_temp}");
    println!("Calculate the Nth Fibonacci Number");
    let mut nth_number = String::new();
    io::stdin()
            .read_line(&mut nth_number)
            .expect("Failed to read line");
    let nth_number: i32 = nth_number.trim().parse().expect("Invalid number");
    let nth_fibonacci_num = calc_n_fibonacci_number(nth_number);
    println!("nth_fibonacci_num {nth_fibonacci_num}");
    println!("Print the lyircs for a given day in the 12 days of Christmas");
    let mut day_of_christmas = String::new();
    io::stdin()
            .read_line(&mut day_of_christmas)
            .expect("Failed to read line");
    let day_of_christmas: i32 = day_of_christmas.trim().parse().expect("Invalid number");
    print_christmas_carol(day_of_christmas);
}
