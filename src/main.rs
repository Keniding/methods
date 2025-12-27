mod rectangle;

use rectangle::Rectangle;

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    if rect.width() {
        println!("The rectangle has width {} square has a nonzero!", rect.width);
    }
}
