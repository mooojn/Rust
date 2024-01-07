fn main(){
    // just like range fn of python
    println!("loop with step");
    for i in (0..10).step_by(2){    
        println!("Hi count {}", i);
    }
    println!("loop without step");
    for i in 0..5{    
        println!("Hi count {}", i);
    }
    println!("If condition");
    let marks = 50;
    let age = 18;
    if marks >= 50 && age == 18{
        println!("Eligible");
    }
}