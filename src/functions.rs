pub fn run() {
    greeting("Hello","Jean");
    println!("{}", add(2,3));

    // closure
    // shorthand functions
    let add_nums = |n1: i32, n2:i32| n1 + n2;
    println!("C sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2:i32) -> i32 {
    n1 + n2
}