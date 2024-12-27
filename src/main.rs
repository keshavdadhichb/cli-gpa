use std::{env, io::{self, Write}};
use colored::*;

fn convert_grade_to_points(grade: &str) -> Option<f32> {
    match grade.to_uppercase().as_str() {
        "S" => Some(10.0), "A" => Some(9.0), "B" => Some(8.0),
        "C" => Some(7.0), "D" => Some(6.0), "E" => Some(5.0),
        "F" | "N" => Some(0.0), _ => None,
    }
}

fn calculate_gpa() {
    println!("Enter credits followed by grade (e.g., 3A). Press q to finish.");
    let mut total_points = 0.0;
    let mut total_credits = 0.0;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.to_lowercase() == "q" { break; }

        let (credits_str, grade_str) = input.split_at(input.len() - 1);
        
        let credits: f32 = match credits_str.parse() {
            Ok(num) if [1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 20.0].contains(&num) => num,
            _ => { println!("Invalid credits!"); continue; }
        };

        let grade_points = match convert_grade_to_points(grade_str) {
            Some(points) => points,
            None => { println!("Invalid grade!"); continue; }
        };

        total_points += grade_points * credits;
        total_credits += credits;
    }

    if total_credits > 0.0 {
        print!("Your GPA is ");
        println!("{}", format!("{:.2}", total_points / total_credits).green().bold());
        println!("Credits: {:.1}", total_credits);
    }
}

fn main() {
    match env::args().nth(1).as_deref() {
        Some("gpacalc") => calculate_gpa(),
        _ => println!("Usage: {} gpacalc", env::args().next().unwrap()),
    }
} 