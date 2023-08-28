use std::vec;
use quick_sort::fill_array_with_usr_input;
use rand::Rng;
use std::time::Instant;

#[cfg(test)]
mod test;

fn main() {
    
    let mut usr_number_array: Vec<i32> = vec![];

    fill_array_with_usr_input(&mut usr_number_array);
    println!("Input array: {:?}", usr_number_array);

    //mesure time it took to sort our array
    let start_time = Instant::now();

    quick_sort(&mut usr_number_array);
    println!("Sorted array: {:?}", usr_number_array);

    let elapsed_time = start_time.elapsed();
    print!("It took {:?} to sort your array", elapsed_time);
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