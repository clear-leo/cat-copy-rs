use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        std::process::exit(0);
    }
    
    let message: String = match fs::read_to_string(&args[1]) {
        Ok(files) => files,
        Err(_error) => {
            println!("{}: No such file or directory", &args[1]); 
            std::process::exit(0);
        }
    };

    println!("{}", message);
}

