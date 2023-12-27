use std::io;

pub fn main() {
    //SCALAR TYPES
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    scalar_type();

    //COMPOUND TYPES
    //Rust has two primitive compound types: tuples and arrays.
    compound_type();
}

fn scalar_type() {
    integer_data_type();
    floating_data_type();
    boolean_data_type();
    character_data_type();
}

fn compound_type() {
    tuple_data_type();
    array_data_type();
}

fn integer_data_type() {
    //Integer Data type

    // the difference between u and i is, signed and unsigned. i.e., if we give a negative sign to signed integer variable
    // it will be considered as negative integer, and for unsigned integer it will ever be positive

    let signed_integer_of_8bit: i8 = -10; //here the output will be -10
    let unsigned_integer_of_8bit: u8 = 20; //if we try to sigining this variable it gives complie time error
    let signed_integer_of_16bit: i16 = 100;
    let unsigned_integer_of_16bit: u16 = 120;
    let signed_integer_of_32bit: i32 = 10;
    let unsigned_integer_of_32bit: u32 = 10;
    let signed_integer_of_64bit: i64 = 10;
    let unsigned_integer_of_64bit: u64 = 10;
    let signed_integer_of_128bit: i128 = 10;
    let unsigned_integer_of_32bit: u128 = 10;
    let signed_integer_of_size_computer_architecture: isize = 10;
    let unsigned_integer_of_size_computer_architecture: usize = 10;

    //Each signed variant can store numbers from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive, where n is the number of bits that variant uses.
    //So an i8 can store numbers from -(2^7) to (2^7) - 1
    //If you give a value exceeding those limit interger overflow will occur at compile time
    //for more info read https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow

    // the data type isize and usize is size (in bits) depends on the device(computer mostly) architecture
    // if the user using 32bit architectured
    //Default size for integer variable is 32bit(it'll applicable if you don't specify the data type)
}

fn floating_data_type() {
    //Floating-point types

    let float_variable_of_32bit: f32 = 20.1;
    let float_variable_of_64bit: f64 = 20.5;

    //Default size for float variable is 64bit(it'll applicable if you don't specify the data type)

    //Numerical Operators
    //like many of the programming language rust also have numerical operation for various arithematic operations.
    //those are...
    //1.Addition
    //2.subtraction
    //3.Multiplication
    //4.division
    //5.remainder

    //Addition
    let summing_variable = 5 + 10;

    //subtraction
    let subtraction_varable = 2 - 1;

    //multiplication
    let multiplication_variable = 3 * 2;

    //division
    let division_variable = 10 / 2;

    //remainder
    let remainder_variable = 10 % 5;
}

fn boolean_data_type() {
    //The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation....
}

fn character_data_type() {
    //The Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //Rust char type is four bytes in size and represented with unicode scalar value
    //for more info read https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type
}

fn tuple_data_type() {
    let tuple = (20.3, 16.5, "Hello", 'A');

    let tuple_with_type_annotation: (&str, char) = ("Ohayo", 'b');

    let twenty_point_three = tuple.0;

    let sixteen_point_five = tuple.1;

    let greeting = tuple_with_type_annotation.1;
}
fn array_data_type() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let first = a[0];
    let second = a[1];

    array_example();
}

fn array_example() {
    let a = [1, 2, 3, 4, 5];
    let mut input_index = String::new();

    print!("Enter a number");
    io::stdin()
        .read_line(&mut input_index)
        .expect("Unexpected interruption occurs");

    let input_index: usize = input_index
        .trim()
        .parse()
        .expect("User haven't entered a number");

    let result = a[input_index];

    print!("the result is {result}");
}
