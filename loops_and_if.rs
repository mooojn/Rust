fn main(){
    // for loop
    println!("loop with step");
    // just like range fn of python
    for i in (0..10).step_by(2){    
        println!("Hi count {}", i);
    }
    println!("loop without step");
    for i in 0..5{    
        println!("Hi count {}", i);
    }
    
    println!("If conditions");
    let marks = 51;
    let age = 18;
    
    // if condition's
    if age < 18 || age > 18{
        println!("You did'nt meet the age requirements");
    }
    else if marks < 50{
        println!("Your score was below the expectations");
    }
    else if marks >= 50 && age == 18{
        println!("Eligible");
    }
    else{
        println!("unexpected input");
    }
}