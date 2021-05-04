use std::env;
//use std::fs; // --> mv to lib.rs
use std::process;
//use std::error::Error; // --> mv to lib.rs
use grep_cli::Config;
//use cli::run;

/*
fn main() {
Collect Args to Vector of String args
   let args: Vec<String> = env::args().collect();
   //println!("{:?}", args);

   let query = &args[1];
   let filename = &args[2];

   println!("Searching for {}", query);
   println!("In file {}", filename);

   expect is some kind of error handling
   in below case is if can not convert content in file name (i.e. file not found or else) it will panic and print string "Something went wrong reading the file"
   let contents = fs::read_to_string(filename)
       .expect("Something went wrong reading the file");

   println!("With text:\n{}", contents);

}
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap is warpping the Result Type, if Err happen its will panic
    // example : if "let config = Config::new(&args).unwrap();" get Err
    //           thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "Not Enough Arguments"', src/main.rs:34:37
    // let config = Config::new(&args).unwrap();

    // unwrap_or_else is basically the same as unwrap but we can custom what we will to do with Err instead of panic
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //let config = parse_config(&args); //--> change to use Config::new() instead

    //println!("Searching for {}", config.query);// --> Print to display value 
    //println!("In file {}", config.filename);   // --> Print to display value   

    //run(config); // --> move fn run to lib.rs

    // if got Err from grep_cli::run(), print error and exit 1
    // eprintln! is print output to stderr instead of stdout
    if let Err(e) = grep_cli::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
    
    
    //let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file"); // change to use fn run() instead

    //println!("With text:\n{}", contents); // change to use fn run() instead
}

/*
struct Config { // --> move to lib.rs
   query: String,
   filename: String,
}
*/

/*
//fn parse_config(args: &[String]) -> Config { //--> Change parse_config functions to Constructor New of Struct Config 
//   let query = args[1].clone();
//   let filename = args[2].clone();
//
//   Config { query, filename }
//}

impl Config { // --> move to lib.rs
   fn new(args: &[String]) -> Result<Config, &str> {
       if args.len() < 3 {
           return Err("Not Enough Arguments");
       }

       if args.len() > 3 {
           return Err("Too Many Arguments");
       }

       let query = args[1].clone();
       let filename = args[2].clone();

       return Ok(Config { query, filename });
   }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> { // --> move to lib.rs
   let contents = fs::read_to_string(config.filename)?;

   println!("With text:\n{}", contents);

   return Ok(());
}

*/