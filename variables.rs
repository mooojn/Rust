fn main(){
    let num: i32 = 8;
   
    let character;           
    character = 'M'; // compiler infers the type at first initialization
    // character = 's'; // immutable can't change
   
    let result: bool = { // we can assign a result of a block during assignment 
        if num > 8{
            true
        }
        else{
            false
        }
    // ommiting of semi-colon signifies the return
    };

    // ouput of vars
    println!("intger: {}\ncharacter: {}\nboolean: {}", num, character, result);

    // custom fn
    let temp1 = 32;
    let x = print(temp1);  
    println!("unit data type: {:?}", x); // displaying return type 

    // sum of two nums
    let n1: i32 = 6;
    let n2: i32 = 7;
    let summed = sum(6,7);
    println!("Sum of {} and {} is: {}", n1, n2, summed)
}

// args must be annoted with type // "->" after this we tell the return type
fn print(temp1: i32) -> (){
    println!("output from custom print: {}", temp1);
// no statement ending without semicolon so it'll return "unit data type()"
}

// fn to add to nums 
fn sum(num1: i32, num2: i32) -> i32{
    let res: i32 = num1 + num2; // can't return assignment
    res                        // so returning on next line
}