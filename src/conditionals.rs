pub fn run(){
    let age: u8 = 21;
    let check_id: bool = true;

    //  IF / ELSE
    if age >= 21 && check_id {
        println!("{} is greater than 21", age);
    } else if age <21 && check_id {
        println!("{} is lower than 21", age);
    } else {
        println!("I need to see your ID");
    }

    // shorthand if 
    let is_of_age = if age >= 21 { true } else { false };
    println!("is {}", is_of_age);

}