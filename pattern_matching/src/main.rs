use crate::exercise_6::{exec_operations, exec_operations_reverse, Operation};
use crate::exercise_2::{manage_message, Message};
use crate::exercise_3::{area_or_square, Rectangle};
use crate::exercise_4::describe_slice;
use crate::exercise_5::print_vec;

mod exercise_1;
mod exercise_2;
mod exercise_3;
mod exercise_4;
mod exercise_5;
mod exercise_6;

fn main() {
    println!("Exercise 1:");
    println!("\t{}", exercise_1::describe_point((0, 0)));
    println!("\t{}", exercise_1::describe_point((0, 2)));
    println!("\t{}", exercise_1::describe_point((4, 0)));
    println!("\t{}", exercise_1::describe_point((4, 2)));

    println!("Exercise 2:");
    print!("\t");
    manage_message(Message::Ping);
    print!("\t");
    manage_message(Message::Text(String::from("Hello world!")));
    print!("\t");
    manage_message(Message::Pong);
    print!("\t");
    manage_message(Message::Close);

    println!("Exercise 3:");
    println!("\t{}", area_or_square(Rectangle {height: 3, width: 2}));
    println!("\t{}", area_or_square(Rectangle {height: 3, width: 3}));

    println!("Exercise 4:");
    print!("\t");
    describe_slice(&[]);
    print!("\t");
    describe_slice(&[1]);
    print!("\t");
    describe_slice(&[1, 2, 4, 5, 233]);

    println!("Exercise 5:");
    print!("\t");
    let el = &mut vec![1, 2, 3, 4];
    print_vec(el);

    println!("\nExercise 6:");
    let el2 = &mut vec![Operation::Sub(1, 5), Operation::Add(9, 4), Operation::Sub(5, 7), Operation::Nothing, Operation::Add(2, 2)];
    exec_operations(el2);
    println!("\nReversed\n");
    exec_operations_reverse(el2)
}
