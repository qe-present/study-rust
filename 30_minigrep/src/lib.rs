#![allow(dead_code)]
use std::error::Error;
use std::fs;

pub fn run(config: Config) ->Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents){
        dbg!(line);
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub struct  Config {
    query: String,
    file_path: String,
}
impl  Config {
    pub fn build(args:&[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("缺少参数");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            query,
            file_path,
        })
    }
}
#[cfg(test)]
mod tests{
    use  super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
C++:";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn  case_sensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me:";
        assert_eq!(vec!["safe, fast, productive."], search_case_insentive(query, contents));
    }
}
