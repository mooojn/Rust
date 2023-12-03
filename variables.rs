fn main()
{
    let x = 5;
    assert_eq!(x, 5);         // checks if both are equal if not program can't run
    println!("Success");
    {                       // this is a separate scope the var here can't be accessed from outside of this body   
        let x = 10;       
        println!("The value of x is: {}", x);
    }
    let x = 20;            // var can be redeclared
    // x = 3;             // var declared with let only there value can't be changed 
    println!("The value of x is: {}", x);
    let mut y = 30;      // var declared with mutable keyword can be changed
    println!("The value of y is: {}", y);
    y = 3;
    println!("The value of y is: {}", y);
}