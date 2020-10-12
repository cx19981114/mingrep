use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
    pub query:String,
    pub file_name:String,
    pub case_sensitive:bool,
}
impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config,&'static str>{
        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query String"),
        };
        let file_name = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query,file_name,case_sensitive})
    }
}
pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}",line);
    }
    Ok(())
}
/// 包含contents内容的一整行数据
/// # Example
/// let a = "a
/// b
/// c
/// d";
/// let b = "b";
/// assert_eq!("b",search(a,b));
pub fn search<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    contents.lines().filter(|a|a.contains(query)).collect()
}
/// 包含contents内容（忽略大小写）的一整行数据
/// # Example
/// let a = "a
/// B
/// c
/// d";
/// let b = "b";
/// assert_eq!("B",search(a,b));
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str) 
-> Vec<&'a str>{
    contents.lines().filter(|a|a.to_lowercase().
    contains(&query.to_lowercase())).collect()
}

// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn case_sensitive(){
//         let query = "duct";
//         let contents="\
// Rust:
// safe,fast,productive.
// Pick three.
// Duct tape";
//         assert_eq!(vec!["safe,fast,productive."],search(query,contents));
//     }
//     #[test]
//     fn case_insensitive(){
//         let query = "rUsT";
//         let contents="\
// Rust:
// safe,fast,productive.
// Pick three.
// Trust me.";
//         assert_eq!(vec!["Rust:", "Trust me."],
//         search_case_insensitive(query,contents));
//     }
// }