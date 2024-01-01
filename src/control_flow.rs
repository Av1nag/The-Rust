pub fn main() {
    let number = 10;

    if number == 10 {
        println!("This is number 10");
    } else {
        println!("this is not a 10 number");
    }
    if_cond_in_let_statement();
    loop_function();
    while_loop_func();
}

//Using if in a let Statement

//Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable

fn if_cond_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loop_function() {
    let mut number = 10;

    loop {
        println!("Hello World");
        number += 1;
        if number == 20 {
            break;
        }
    }
}

fn while_loop_func() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
