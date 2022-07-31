
fn handle_printing(number: i8) {

    // if number % 3 == 0 && number % 5 == 0 {
    //     println!("fizzbuzz");
    // }
    // else if number % 3 == 0 {
    //     println!("fizz");
    // } else if number % 5 == 0 {
    //     println!("buzz");
    // } else {
    //     println!("{}", number);
    // }


match (number % 3, number % 5) {
        (0, 0) => println!("fizzbuzz"),
        (0, _) => println!("fizz"),
        (_, 0) => println!("buzz"),
        _ => println!("{}", number)
    }
}
fn main() {

    for i in 1..101 {
        handle_printing(i);
    }
    println!("END!");
}
