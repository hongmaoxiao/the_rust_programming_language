fn main() {
    println!("Hello, world!");

    another_function();

    param_function(5);
}

fn another_function() {
    println!("Another function.");
}

fn param_function(x: i32) {
    println!("The value of x is: {}", x);
}
