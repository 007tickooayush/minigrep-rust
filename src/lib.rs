use std::{ env, error, fs };

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.filename)?;

    // println!("With text:\n{}", contents);
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone(); // clone() is used to make a copy of the string
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        return Ok(Config { query, filename, case_sensitive });
    }
}

pub fn search<'a>(query:&str, contents:&'a str) -> Vec<&'a str>{
    // vec![]

    //// old approach
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line)
    //     }
    // }
    // results

    //// new approach using iterator methods and closures
    contents.lines().filter( |x| x.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

// pub fn search_case_insensitive<'a>(query:&str, contents:&'a str) -> Vec<&'a str>{
//     let query = query.to_lowercase();
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line)
//         }
//     }
//     results
// }

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result_test(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}