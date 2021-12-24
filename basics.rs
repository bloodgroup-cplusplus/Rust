 1 pub fn run()                                                                    
  2 {                                                                               
  3     println!("Hello from the print.rs file");                                   
  4     println!("Number: {}",1);                                                   
  5                                                                                 
  6     // for multiple place holders                                               
  7     //                                                                          
  8     println!("{} is from {}", "Brad", "Mass");                                  
  9                                                                                 
 10     //Positional Arguments                                                      
 11     //                                                                          
 12     println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "ckode");  
 13                                                                                 
 14                                                                                 
 15     //Named Arguments                                                           
 16     println!("{name} likes to play {activity}",                                 
 17             name = "John",                                                      
 18             activity = "Baseball",                                              
 19         );                                                                      
 20                                                                                 
 21     // Placeholder traits                                                       
 22                                                                                 
 23     println!("Binary : {:b} Hex:{:x} Octal :{:o}", 10,10,10);                   
 24                                                                                 
 25                                                                                 
 26                                                                                 
 27     println!("Binary : {:b} Hex : {:x} Octal : {:o}", 10,10,10);                
 28                                                                                 
 29                                                                                 
 30     // Placeholder for debug trait                                              
 31                                                                                 
 32     println!("{:?}",(12,true,"hello"));                                         
 33                                                                                 
 34     //                                                                          
 35     // Basic Math                                                               
 36     //                                                                          
 37     println!("10 + 10 = {}", 10+10);                                            
 38                                                                                 
 39                                                                                 
 40                                                                                 
 41 }  
