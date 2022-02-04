fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100);

    let mut vv = vec![1, 2, 3, 4, 5];
    let first = &vv[0];

    // vv.push(6);

    println!("The first element is: {}", first);

    // loop
    let vvv = vec![100, 200, 300];
    for i in vvv {
        println!("vvv item: {}", i);
    }

    let mut vvvv = vec![100, 32, 57];
    for i in &mut vvvv {
        println!("vvvv mut before item: {}", i);
    }
    for i in &mut vvvv {
        *i += 50;
    }

    for i in vvvv {
        println!("vvvv item: {}", i);
    }
}
