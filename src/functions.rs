pub fn main() {
    function_with_no_arguments();
    function_with_argument(10);
}

fn function_with_no_arguments() {
    print!("A Function with no arguments");
}

fn function_with_argument(number1: i32) {
    print!("The result value is:{number1}");
}

fn funtion_with_return_type() -> i32 {
    let x = 10;
    x + 2
    // A line with no semi column at last line of the function is a RETURN value, and its type should explicity annotated when declating the function (with -> sign)
}
