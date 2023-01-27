// Arrays - Fixed list where elements are the same data types

use std::num;

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    //Re-assign value
    numbers[2] = 20;
    
    println!("{:?}", numbers);

    // Get single val
    println!("{}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers) );

    // Get Slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice );

}