fn main()
{
    let num: i32 = 8;
    let character;
    character = 'm';         // compiler infers the type at first initialization
    // character = 's';      // immutable can't change

    println!("{}", num);
    println!("{}", character);
}