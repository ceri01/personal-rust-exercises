// Exercise to learn slices used with pattern matching

pub fn describe_slice(numbers: &[i32]) {
    match &numbers[..] {
        [] => println!("Empty slice!"),
        [only] => println!("Only one element: {only}"),
        [first, second, others @ ..] => {
            println!("Many element: {}, {} and other {}", first, second, others.len())
        }
    }
}