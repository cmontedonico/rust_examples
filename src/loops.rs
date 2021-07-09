// loops to iterate
pub fn run() {
    let mut count = 0;

    // inFinite Loop 
    loop {
        count += 1;
        println!("{}", count);

        if count == 100 {
            break;
        }
    }

    //While Loop (FizzBuzz)
    //If number is divisible by 3 print fizz
    // if number divisible by 5 print Buzz
    // if number divisible by 15 print fizzbuzz
    count = 1;
    while count <= 100{
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }

}