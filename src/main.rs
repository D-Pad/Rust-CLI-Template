use cli_template::{self, ServerError};
use std::{io::{self, BufRead}, process};


fn main() {

    println!("service_started");
    
    fn collect_args<'a>(input_string: &'a str) -> Vec<&'a str> {
        input_string.split_whitespace().collect()
    }

    fn main_error_handler(err: ServerError, error_inputs: &[&str]) {
        let response = cli_template::error_handler(err, Some(error_inputs));
        eprintln!("{}", response.1); 
    }

    // Initial processes
    let stdin = io::stdin().lock();

    // Main loop 
    for line in stdin.lines() {
     
        let input = match line {
            Ok(v) => v,
            Err(_) => break
        };
        
        if input == "stop" { 
            println!("service_stopping");
            process::exit(0); 

        };

        let input_args: Vec<&str> = collect_args(&input);

        match cli_template::run(&input_args) {
            Ok(response) => {
                let delimited: String = response.msg.replace('\n', "\0");
                println!("{}", delimited)
            },
            Err(err) => {
                main_error_handler(err, &input_args);
            } 
        };
    };
}


