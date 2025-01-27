// Exercise to learn struct

pub struct Rectangle {
    pub height: u32,
    pub width: u32
}

pub fn area_or_square(r: Rectangle) -> String {
    if r.height == r.width {
        "Square rectangle".to_string()
    }
    else {
        format!("Area = {}", r.height * r.width)
    }
}