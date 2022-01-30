fn main() {
    println!("Hello, world!");

    another_function();

    param_function(5);

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 6;
        x
    };
    println!("Now y is: {}", y);

    let z = five();
    println!("The value of z is: {}", z);

    let p = plus_one(5);
    println!("The value of p is: {}", p);
}

fn another_function() {
    println!("Another function.");
}

fn param_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}