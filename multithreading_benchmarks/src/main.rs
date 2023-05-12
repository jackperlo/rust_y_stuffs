use std::io;

use crate::solution_finder::{create_n_threads, search_for_operations_no_threads};
mod solution_finder;

fn main() {
    let mut my_vec: Vec<u32> = Vec::new();
    let result_strings: Vec<Box<String>>;

    println!("Insert the numbers separated by comma: ");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Error while reading stdin");

    match parse_input(input_string){
        Ok(vec_parsed) => my_vec = vec_parsed,
        Err(err) => eprintln!("{:?}", err)
    }

    println!("=============No Threads Test:=============");
    result_strings = search_for_operations_no_threads(&my_vec);
    for result_string in result_strings{
        println!("RESULT = \"{}\"", result_string);
    }
    println!("\n\n=============Threads Test:=============");
    create_n_threads(5, my_vec);
}

fn parse_input(input_string: String) -> Result<Vec<u32>, &'static str> {
    let vec_parsed = input_string
        .trim()
        .split(',')
        .map(|x| x.parse().expect("parse error"))
        .collect::<Vec<u32>>();
    if vec_parsed.len() != 5 {
        return Err("ERROR: too many/few numbers inserted");
    }
    Ok(vec_parsed)
}