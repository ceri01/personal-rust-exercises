// Exercise to recap all concepts in exercises 1-6


pub enum Operation {
    Add(i32, i32),
    Sub(i32, i32),
    Nothing
}
pub fn exec_operations(operatios: &Vec<Operation>) {
    for op in operatios {
        match_operation(op)
    }
}

pub fn exec_operations_reverse(operatios: &mut Vec<Operation>) {
    while let Some(op) = &mut operatios.pop() {
        match_operation(&op)
    }
}

fn match_operation(operatios: &Operation) {
    match operatios {
        Operation::Add(x, y) => println!("\tSum between {x} and {y} = {}", x+y),
        Operation::Sub(x, y) => println!("\tDiff between {x} and {y} = {}", x-y),
        Operation::Nothing => println!("\tNo operation"),
    }
}