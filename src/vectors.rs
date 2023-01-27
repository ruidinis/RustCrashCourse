// Vectors - Resizable arrays

use std::num;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(12);

    // Pop off last value
    numbers.pop();
    
    println!("{:?}", numbers);

    // Get single val
    println!("{}", numbers[0]);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    //Vector are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers) );

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice );

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

}