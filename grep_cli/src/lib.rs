use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }

        if args.len() > 3 {
            return Err("Too Many Arguments");
        }


        // Noted: Clone operation do take some performace but in this case args is very small, So its didnt matter much.
        let query = args[1].clone();
        let filename = args[2].clone();
        //let mut case_sentitive: bool;
        //if args[3] == "1"{
        //    let case_sensitive = true;
        //    return Ok(Config { query, filename, case_sensitive });
        //}
        //else {
        //    let case_sensitive = false;
        //    return Ok(Config { query, filename, case_sensitive });
        //}

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config { query, filename, case_sensitive });

        
    }
}

// TODO: why we have to use Box<> why not T 
//pub fn run<T>(config: Config) -> Result<(),T> {


// Box<dyn Error> is data type that can contain any Err type
// TODO: Why have to be dyn ??
//       Can we use other thing to contain any type of Err, i.e. Generic Type 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    //println!("With text:\n{}", contents);

    //for line in search(&config.query, &contents) {
    //    println!("{}", line);
    //}

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}",line);
    }

    return Ok(());
}

/*  Noted : only contests and return output have generic lifetime
            Because we did not use anything relate to query in output
            But return output need to reference contents
*/
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

fn search_case_insensitive<'a>(query: &str,contents: &'a str,) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// module test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        // assert if result from search() is not the same as vec!["safe, fast, productive."] (expected output)
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn test_case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            // assert if result from search() is not the same as vec!["Rust:", "Trust me."] (expected output)
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}