// Exercise to learn pattern matching

pub fn describe_point(p: (i32, i32)) -> String {
    match p {
        (0, 0) => {String::from("Origin")},
        (x, 0) => {format!("On X axe with x = {x}")},
        (0, y) => {format!("On Y axe with y = {y}")},
        (x, y) => {format!("Generic coordinate (x, y) = ({x}, {y})")}
    }
}