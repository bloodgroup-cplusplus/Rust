extern crate structopt;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Options
{
    #[structopt(default_value = "Meow!")]
    message:String,
    //[1]
    #[structopt( short = "d", long = "dead")]
    dead: bool,
    // add a field of type bool named dead. You can assign the long and 
    // short version of the flag by annotating the field with #[structopt(short = "d", long
    // ="dead")]. The help message will now look like this. 
    // FLAGS:
    //  -d -dead Makes the cat appear dead 
    //  -h --help Prints help informaton 
    //  -V --version Prints version informaton 

}

fn main() {
/*  println!("Hello, world!"); */
//    let message = std::env::args().nth(1)
 //       .expect("Missing the message.Usage:catsay< message>");
  //  println!("{}",message);
   // println!(" \\");
   // println!(" \\");
   // println!("     /\\_/\\");
   // println!("     ( o  o ) ");
    //println!("     =( I ) = ");

    /* the std::env::args() function returns an interator to the arguments. 
     * The 0th argument is the name of the binary itself, catsay, and the string
     * you are looking for is the next arguemnt, so you can call the nth(1)
     * function on the iterator to get the first argument. The nth() function 
     * might fail(e.g., if n is larger thatn the size of the iterator) and return an 
     * Option::None, so you can use unwrap or expect to get the contained value. 
     * Then you assign this value to a variable namedd message and print it out 
     * using println!();
     */
    let options = Options::from_args(); // [2] 
    let message = options.message;
    println!("{}",message);
    //...print the cat 
    // In [1] , you define a struct named Options that has one String field
    // called message. Then you annotate the struct with the custom derive
    // attribute #[derive(structopt)]. This way structopt will take the struct as 
    // the argument definition and generate the arguemetn parsers accordingly.
    // to actually parse the arguments in main(), you call options::from 
    // args() , which parses the arguments and fills theim into the Options struct
    // and returns it. 
    // You can then access the individual fields like a normal Rust struct 
    //
    //
    let eye = if options.dead { "x" } else { "o" } ; //[1] 
    println!("{}", message);
    println!(" \\");
    println!(" \\");
    println!("    /\\_/\\");
    println!("   ({eye} {eye})", eye = eye);//[2]
    println!("  = ( I ) = ");
    
    // when a flag has the bool tpe, its values are determiend by the
    // presence of it. If the flag is not present, it willbe set to false and vice versa. 
    //
    if message.to_lowercase() == "woof"{
        eprintln!(" A cat shouldn't bark like a dog. " )
    }
    //
}

