pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    //formating
    //let &mut number = 4;
    // println!("test of a file {}", number);
    println!("{} is from {}", "Cesare", "mexico");
    println!("{0} is from {1} and {2} is from {1} too", "Cesare", "Mexico", "Carl");
    println!("{name} likes to play {activity}", name="John", activity="footbal");


    //Placeholder traits
    println!("Binary : {:b} hex: {:x} octoal: {:o}", 10, 10, 10);

    //placeholder for debug traits like TUPLE and dump or print_all()
    println!("{:?}", (12, true, "hello"));

    
}