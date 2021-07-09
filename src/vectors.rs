//Vector are resizeable arrays
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);

    //Add on a vector
    numbers.push(23232);
    println!("{:?}", numbers);

    // get single value
    println!("{:?}", numbers[2]);

    // Update value
    numbers[2] = 42222;
    println!("{}", numbers[2] );

    //Length vector
    println!("{}", numbers.len());

    // get slice 
    let slice: &[i32] = &numbers[0..3];
    println!("{:?}", slice);

    //loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mut values
    for x in numbers.iter_mut(){
        *x *= 2;
        println!("double: {}", x);
    }
}