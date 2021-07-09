pub fn run(){
    let hello = "Hello";
    let mut hello2 = String::from("Hello ");
    
    println!("{}", hello);
    println!("{}", hello2);

    //Get lenght
    println!("Lenght: {}", hello.len());
    println!("length2 {}", hello2.len());

    //Concat
    hello2.push_str("Carlos");
    println!("length2 {}", hello2);

    //CApacity in bytes
    println!("Capacity {}", hello2.capacity());

    //Is empty?
    println!("Is Empty?: {}", hello2.is_empty());

    //Contians a word
    println!("Contains 'llo'?: {}",hello2.contains("llo"));

    //REplace
    println!("replace  {}", hello2.replace("Carlos", "Cesare"));

    // loop with a split function 
    for word in hello2.split_whitespace(){
        println!("{}", word);
    }

    
}
