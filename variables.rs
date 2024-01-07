fn main()
{
    let num: i32 = 8;
    let character;           
    character = 'm'; // compiler infers the type at first initialization
    // character = 's'; // immutable can't change
    let result = { // we can assign a result of a block during assignment 
        if num > 8{
            true
        }
        else{
            false
        }
    };
        
    println!("{}", num);
    println!("{}", character);
    println!("{}", result);
}