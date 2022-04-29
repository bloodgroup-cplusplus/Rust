//create a function in this print file and run in the rs 
//

pub fn run()
{
    // Print to console 

    println!("Hello From the print.rs File");
    println!("Number :{}",1);
    // add multiple place holders 
    println!("{} is from {}", "Brad", "Mass");

    //Positional Arguemnts

    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named Arguments 
    
    println!("{name} likes to play {activity}", name = "John", activity = "Football");


    //Placeholder traits 

    println!("Binary: {:b} Hex:{:x} Octal :{:o}", 10,10,10);



    // Placeholder for debug traits 

    println!("{:?}", (12,true,"hello"));

    // Basic Math 
    
    println!("10 + 10 = {}", 10+10);

    }



