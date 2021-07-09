pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    //Single value
    println!("{}", numbers[0]);

    //Change value
    numbers[2] = 44;
    println!("{}", numbers[2]);

    //lenght
    println!("lenght {}", numbers.len());

    //Memory that takes
    println!("Array is allocated at {} bytes", std::mem::size_of_val(&numbers));


    //Diferent types for same array with memory
    let a : [u8;5] = [1,2,3,4,5];
    println!("Array is allocated at {} bytes", std::mem::size_of_val(&a));

}