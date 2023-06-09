use std::fs;
use std::error::Error;
use std::env;
pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool,
}
impl Config{
    pub fn build(mut args:impl Iterator<Item = String>)->Result<Config,&'static str>{
        args.next();
        let query=match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't recieved any query string")
        };
        let file_path=match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't recieved any file_path"),
        };
        let ignore_case=env::var("IGNORE_CASE").is_ok();
        Ok(Config {query,file_path,ignore_case})
    }
}
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let content=fs::read_to_string(config.file_path)?;
    let results=if config.ignore_case{
        search_case_insensitive(&config.query,&content)
    }else{
        search(&config.query, &content)
    };
    for line in results{
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn case_sensitive(){
        let query="duct";
        let content="\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],search(query, content))
    }
    #[test]
    fn case_insensitive(){
        let query="rUSt";
        let content="\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query,content)
        )
    }
}
pub fn search<'a>(query:&str,content:&'a str)->Vec<&'a str>{
    content
        .lines()
        .filter(|x| x.contains(query))
        .collect()
}
fn search_case_insensitive<'a>(query:&'a str,content:&'a str)->Vec<&'a str>{
    let query=query.to_lowercase();
    let mut results=Vec::new();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line)
        }
    }
    results
}