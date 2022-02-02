#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_tuple(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect2 is {:#?}", rect2);

    println!("The area of the rectangle is {} square pixels.", area_struct(&rect2));

    println!("{}-{}", rect2.width, rect2.height);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area_struct(&rect3));
    dbg!(rect3);
    println!("{}-{}", rect3.width, rect3.height);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}