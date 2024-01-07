fn main(){
    let num: i32 = 8;
   
    let character;           
    character = 'm'; // compiler infers the type at first initialization
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
    println!("{}", num);
    println!("{}", character);
    println!("{}", result);

    // custom fn
    let temp1 = 32;
    let x = print(temp1);  
    println!("{:?}", x); // displaying return type 
}

// args must be annoted with type // "->" after this we tell the return type
fn print(temp1: i32) -> (){
    println!("{}", temp1);
}
// no statement ending without semicolon so it'll return "unit data type()"