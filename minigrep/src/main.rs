use std::env; 
use std::fs; // handle files 

// Basic function to grep for something in a file 
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let filename = &args[2];

//     println!("Searching for {}", query);
//     println!("In file {}", filename); 
// }

// Print the entire poem from a file 

fn main() {
    let args: Vec<String> = env::args().collect();

    let _query = &args[1]; // the "_" here is to signify that the variable is important but unused in the rest of the code

    let filename = &args[2];

    println!("In file {}", filename);

    // returns Result<string>
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong in reading the file");

    // Test statment to check if the file is read
    println!("With Text:\n{}", contents);

}