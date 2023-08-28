use std::vec;
use rand::Rng;
use sorting_algorithms::fill_array_with_usr_input;
use std::time::Instant;

//for file operations
use std::fs::OpenOptions; //for managing file
use std::io::Write;//for writing to file with writeln! macro

#[cfg(test)]
mod test;

fn main() {
    //vector for user nums
    let mut usr_number_array: Vec<i32> = vec![];

    fill_array_with_usr_input(&mut usr_number_array);
    println!("Input array: {:?}", usr_number_array);

    //if its too small dont sort it
    if usr_number_array.len() <= 2 {
        print!("Your array is too small to be sorted!\n");
        return;
    }

    //mesure time it took to sort our array
    let start_time = Instant::now();

    quick_sort(&mut usr_number_array);

    //stop mesuring and print information
    let elapsed_time = start_time.elapsed();
    println!("Sorted array: {:?}", usr_number_array);
    print!("It took {:?} to sort your array", elapsed_time);

    //save time to file
    let file_name = "time_stamp.txt";
    //check if file exists and save time to it
    let mut file = OpenOptions::new().create(true).append(true).open(file_name).unwrap();
    writeln!(file, "{:?}", elapsed_time).expect("Error in writing to file");

}

fn quick_sort(array: &mut Vec<i32>){

    if array.len() <= 1 {
        return;
    }

    //create pivot so we can compare nums from array
    let pivot_index = rand::thread_rng().gen_range(0..array.len());
    let pivot = array[pivot_index];

    let mut smaller_nums: Vec<i32> = vec![];
    let mut bigger_nums: Vec<i32> = vec![];


    //now logic for sorting
    //we need to divide the array to two - one has smaller elements than piovot, other bigger
    for &num in array.iter() {
        if num > pivot {
            bigger_nums.push(num);
        } else if num < pivot {
            smaller_nums.push(num);
        }
    }

    //well we need to glue this together
    // 1st clear array
    array.clear();

    //append smaller elements and pivot
    quick_sort(&mut smaller_nums);
    array.append(&mut smaller_nums);
    array.push(pivot);

    //append bigger elements 
    quick_sort(&mut bigger_nums);
    array.append(&mut bigger_nums);

}