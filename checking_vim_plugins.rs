use std::string::String;
pub struct Cat
{
    pub name:String,

}


impl Cat{
    fn meow()
    {
        println!("meow");
    }

}


fn main()
{
    let cat = Cat {
    name: "Test".to_string(),
};

Cat::meow();
println!("Hello world ! {}",cat.name);

}

