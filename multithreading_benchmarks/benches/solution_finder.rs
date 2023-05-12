use std::char::from_digit;
use std::thread;
use std::thread::JoinHandle;
use itertools::Itertools;

const NUMBER_TO_REACH: u32 = 10;
static OPERATIONS: &'static[char] = &['+', '-', '/', '*'];

pub fn search_for_operations_no_threads(input_vec: &[u32]) -> Vec<Box<String>> {
    let mut result_vec: Vec<Box<String>> = Vec::new();
    let mut result_string: String = String::new();
    for number_permutation in input_vec.iter().permutations(input_vec.len()).unique(){
        for operation_permutation in OPERATIONS.iter().permutations(OPERATIONS.len()).unique(){
            result_string.clear();
            let mut has_broken = false;
            let mut number_permutation_index = 0;
            let mut permutation_result: u32 = *number_permutation[number_permutation_index];
            result_string.push(from_digit(*number_permutation[number_permutation_index], 10).ok_or("Error casting i32 to char").unwrap());
            number_permutation_index += 1;
            for operation_index in 0..operation_permutation.len(){
                match operation_permutation[operation_index]{
                    '+' => {
                        let mut aus_string: String= String::from('+');
                        permutation_result += *number_permutation[number_permutation_index];
                        aus_string.push(from_digit(*number_permutation[number_permutation_index], 10).ok_or("Error casting i32 to char").unwrap());
                        result_string.push_str(&aus_string);
                    },
                    '-' => {
                        let mut aus_string: String= String::from('-');
                        if permutation_result >= *number_permutation[number_permutation_index] {
                            permutation_result -= *number_permutation[number_permutation_index];
                            aus_string.push(from_digit(*number_permutation[number_permutation_index], 10).ok_or("Error casting i32 to char").unwrap());
                            result_string.push_str(&aus_string);
                        }else {
                            has_broken=true;
                            break;
                        }
                    },
                    '/' => {
                        let mut aus_string: String= String::from('/');
                        permutation_result = permutation_result / *number_permutation[number_permutation_index] ;
                        aus_string.push(from_digit(*number_permutation[number_permutation_index], 10).ok_or("Error casting i32 to char").unwrap());
                        result_string.push_str(&aus_string);
                    },
                    '*' => {
                        let mut aus_string: String= String::from('*');
                        permutation_result *= *number_permutation[number_permutation_index];
                        aus_string.push(from_digit(*number_permutation[number_permutation_index],10).ok_or("Error casting i32 to char").unwrap());
                        result_string.push_str(&aus_string);
                    },
                    _ => ()
                }
                number_permutation_index+=1;
            }
            if permutation_result == NUMBER_TO_REACH && !has_broken {
                let my_box: Box<String> = Box::new(String::from(result_string.clone()));
                result_vec.push(my_box);
            }
        }
    }
    result_vec
}

fn n_permutations_count(input_vec: Vec<u32>) -> usize{ input_vec.iter().permutations(input_vec.len()).unique().count() }

fn slice_of_permutation(input_vec: Vec<u32>, start_index: usize, end_index: usize) -> Vec<Box<Vec<u32>>>{
    let mut res_vec: Vec<Box<Vec<u32>>> = Vec::new();
    let mut index = 0;
    for number_permutation in input_vec.clone().into_iter().permutations(input_vec.len()).unique(){
        if index >= start_index && index <= end_index{
            let aus = Box::new(number_permutation);
            res_vec.push(aus)
        }
        index+=1;
    }
    res_vec
}

pub fn create_n_threads(n_threads: usize, input_vec_param: Vec<u32>){
    let mut threads: Vec<JoinHandle<Vec<Box<String>>>> = Vec::new();
    let mut vec_result : Vec<Box<String>> = Vec::new();

    let total_permutation: usize = n_permutations_count(input_vec_param.clone());
    //println!("\n\n\n\n total number of permutations: {}", total_permutation);

    let mut exact_multiple = true;
    let block_of_permutations_size: f64 = (total_permutation as f64 / n_threads as f64).floor();
    let mut last_block_size = 0;

    if total_permutation % n_threads != 0 {
        exact_multiple=false;
        last_block_size = total_permutation-(block_of_permutations_size as usize * (n_threads-1));
    }
    for thread_index in 0..n_threads{
        let start_index = thread_index * block_of_permutations_size as usize;
        let end_index;

        if thread_index == n_threads-1 && !exact_multiple{
            end_index = (thread_index * block_of_permutations_size as usize) + (last_block_size as usize - 1);
        }else {
            end_index = (thread_index * block_of_permutations_size as usize) + (block_of_permutations_size as usize - 1);
        }

        let permutation_vec = slice_of_permutation(input_vec_param.clone(), start_index, end_index);
        /*let mut counter = 0;
        for permutation in &permutation_vec{
            println!("Slice to work on: {:?}", *permutation);
            counter+=1;
        }
        println!("======Number of Slices to work on: {}======", counter);*/
        let permutation_cloned_vec = Box::new(permutation_vec.clone());
        threads.push(start_generic_thread(move || {
            let mut result_vec: Vec<Box<String>> = Vec::new();
            let mut result_string: String = String::new();

            for number_permutation in permutation_cloned_vec.iter(){
                for operation_permutation in OPERATIONS.iter().permutations(OPERATIONS.len()).unique(){
                    result_string.clear();
                    let mut has_broken = false;
                    let mut number_permutation_index = 0;
                    let mut permutation_result: u32 = number_permutation[number_permutation_index];
                    result_string.push(from_digit(number_permutation[number_permutation_index], 10).ok_or("Error casting i32 to char").unwrap());
                    number_permutation_index += 1;
                    for operation_index in 0..operation_permutation.len(){
                        match operation_permutation[operation_index]{
                            '+' => {
                                let mut aus_string: String= String::from('+');
                                permutation_result += number_permutation[number_permutation_index];
                                aus_string.push(from_digit(number_permutation[number_permutation_index], 10).ok_or("Error casting i32 to char").unwrap());
                                result_string.push_str(&aus_string);
                            },
                            '-' => {
                                let mut aus_string: String= String::from('-');
                                if permutation_result >= number_permutation[number_permutation_index] {
                                    permutation_result -= number_permutation[number_permutation_index];
                                    aus_string.push(from_digit(number_permutation[number_permutation_index], 10).ok_or("Error casting i32 to char").unwrap());
                                    result_string.push_str(&aus_string);
                                }else {
                                    has_broken=true;
                                    break;
                                }
                            },
                            '/' => {
                                let mut aus_string: String= String::from('/');
                                permutation_result = permutation_result / number_permutation[number_permutation_index] ;
                                aus_string.push(from_digit(number_permutation[number_permutation_index], 10).ok_or("Error casting i32 to char").unwrap());
                                result_string.push_str(&aus_string);
                            },
                            '*' => {
                                let mut aus_string: String= String::from('*');
                                permutation_result *= number_permutation[number_permutation_index];
                                aus_string.push(from_digit(number_permutation[number_permutation_index],10).ok_or("Error casting i32 to char").unwrap());
                                result_string.push_str(&aus_string);
                            },
                            _ => ()
                        }
                        number_permutation_index+=1;
                    }
                    if permutation_result == NUMBER_TO_REACH && !has_broken {
                        let my_box: Box<String> = Box::new(String::from(result_string.clone()));
                        result_vec.push(my_box);
                    }
                }
            }
            result_vec
        }));
    }

    for t in threads{
        vec_result.append(&mut t.join().unwrap());
    }

    /*for result in vec_result{
        println!("RESULT = {:?}", result);
    }*/
}

fn start_generic_thread<F: FnOnce() -> Vec<Box<String>> + Send + 'static>(f: F) -> JoinHandle<Vec<Box<String>>> {
    thread::spawn(f)
}