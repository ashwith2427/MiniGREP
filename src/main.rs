use std::env::args;
use std::process;
use mini_grep::Config;
use mini_grep::run;
fn main(){
    let config=Config::build(args()).unwrap_or_else(|error|{
        eprintln!("Problem parsing arguments: {}",error);
        process::exit(1);
    });
    if let Err(e)=run(config){
        eprintln!("Application Error: {e}");
        process::exit(1)
    }
}
