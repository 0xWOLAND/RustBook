#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of athe rectangle is {} square pixels.",
        area(&mut rect1)
    );
    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &mut Rectangle) -> u32 {
    rectangle.height += 5;
    return rectangle.width * rectangle.height;
}
