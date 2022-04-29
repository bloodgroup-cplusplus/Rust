// variables hold primitive data or refrences to data
// variables are immutable by default 
// rust is a block-scoped language
//
//
//
pub fn run()
{
    let name = "Brad";
    println!("My name is {}",name);
    let mut  age = 37;
    age = 38;
    println!("My age is {}", age);

    // this is not possible 
    // age = 38;

}



