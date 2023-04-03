use std::{env, process};

use mlrs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let data_set = vec![
        vec![1.0, 1.1],
        vec![1.0, 1.0],
        vec![0.0, 0.0],
        vec![0.0, 0.1],
    ];
    let labels = vec![1, 1, 2, 2];
    let in_x = vec![0.0, 0.2];
    let k = 3;
    let result = mlrs::classify0(&in_x, &data_set, &labels, k);
    println!(
        "The input point {:?} is classified as label {}",
        in_x, result
    );
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config{query, file_path})
    }
}
