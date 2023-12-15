pub fn main(){
    
    //Mutable variable
    let mut mutable_variable = 50;
    println!("{mutable_variable}");
        //value reassignment for mutable variable 
    mutable_variable = 60;
    println!("{mutable_variable}");


    //Immutable Variable
    let immutable_variable = 54;
        //we cannot reassign a new varaible to the immutable variable
    //immutable_variable = 60   ###This give compilation error


    //Varaible Shadowing
    //reassigning a the same variable again is called varable shadowing

    let x = 10;
    println!("{x}");
    let x = x + 20;
    println!("{x}");

    // The variable shadowing will stays upto specific code block.. Code Block means logic inside a "{}"

    //for example
    {
        let x = x + 10;
        println!("{x}"); //here the answer will be 40 in this code block
    }
    println!("{x}"); //and here it will again become 30
    //The another benefit of variable shadowing is we can change the data type of the variable, whereas, it isn't possible for mutable variables

    
    
    
    
    //Constant Variable
    //A constant variable is declared by using const keyword and we 'must' annotate the data type of it
    const A_CONSTANT_VARIABLE :i32 = 60*30; 
    //using upper snakecase format is a good practice for constnat variables, but also lower case can be used


}
