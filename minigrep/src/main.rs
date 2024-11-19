use std::env;
use std::fs;


// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);

//     let query = &args[1];
//     let file_path = &args[2];

//     println!("Searching for {}", query);
//     println!("In file {}", file_path);

//     let contents = fs::read_to_string(file_path).expect("Error reading file");
//     println!("With text:\n{}", contents);

// }

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Error reading file");
    println!("With text:\n{}", contents);

}


struct Config{
    query: String,
    file_path: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{query, file_path})
    }
}

fn parse_config(args: &[String]) -> Config(str, str) {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config{query, file_path}
}