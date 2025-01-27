// Exercise to learn vector used with while let

pub fn print_vec(elements: &mut Vec<i32>) {
    while let Some(x) = elements.pop() {
        print!("{} ", x);
    }
}