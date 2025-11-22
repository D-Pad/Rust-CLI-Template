use std::fs;

// ========================================================================= //
// ---------------------------- ERROR HANDLING ----------------------------- //
// ========================================================================= //
pub enum ServerError {
    BadArguments,
    RunTimeError
}

pub fn error_handler(
    server_error: ServerError, 
    input_arguments: Option<&[&str]>
) -> (i32, String) {

    let mut error_message: String = String::from("\x1b[1;31mServerError: ");
    let mut error_code: i32 = 0;
   
    if let ServerError::BadArguments = server_error {
        error_message.push_str("BadArguments: ");
        
        if let Some(args) = input_arguments {
            let arg_string = match args.len() {
                0 => "None provided".to_string(),
                _ => format!("`{}`", args.join(" "))
            }; 
            error_message.push_str(&arg_string)
        }; 
        
        error_code = 1;
    
    } else if let ServerError::RunTimeError = server_error {
        error_message.push_str("RunTimeError: ");
        error_code = 2;

    };
   
    error_message.push_str("\x1b[0m");

    (error_code, error_message)
}


// ========================================================================= //
// -------------------------- COMMAND DISPATCHING -------------------------- //
// ========================================================================= //
enum Command {
    Scan,
    Help,
    Unknown(String)
}

pub struct CommandResponse {
    pub msg: String
}

fn parse_command(cmd: &str) -> Command {
    match cmd {
        "scan" => Command::Scan,
        "help" => Command::Help,
        other => Command::Unknown(other.to_string())
    }
}

fn dispatch(command: Command, args: &ArgParser) -> CommandResponse {
    let msg: String = match command {
        Command::Scan => {
            match args.input_args.len() {
                0 => "No args passed".to_string(),
                _ => "Not yet implemented".to_string()
            }
        },
        Command::Help => {
            match fs::read_to_string("src/docs/help.txt") {
                Ok(t) => format!("\x1b[35m{}", t),
                Err(e) => format!("{}", e)
            }
        },
        Command::Unknown(cmd) => format!("Unknown command: {}", cmd), 
    };
    CommandResponse { msg }
}


// ========================================================================= //
// --------------------------- ARGUMENT PARSING ---------------------------- //
// ========================================================================= //
struct ArgParser<'a>{
    input_args: &'a Vec<&'a str>
}


impl<'a> ArgParser<'a> {

    fn new(input_args: &'a Vec<&str>) -> Self {
        ArgParser { input_args } 
    }

    fn parse(&self) -> Result<Command, ServerError> {

        let no_args_passed: bool = self.input_args.len() == 0;
        let mut cmd: Command = Command::Unknown("".to_string());
        let mut args_are_good = true;

        if no_args_passed {
            args_are_good = false;
        } else {
            cmd = parse_command(self.input_args[0]);
            if let Command::Unknown(_) = cmd {
                args_are_good = false;
            };
        };

        // Return value check here
        match args_are_good {
            true => Ok(cmd),
            false => Err(ServerError::BadArguments)
        }
    } 
}


// ========================================================================= //
// ---------------- FUNCTIONS NEEDED FOR RUNNING THE SERVER ---------------- //
// ========================================================================= //
pub fn run(input_args: &Vec<&str>) -> Result<CommandResponse, ServerError> {
    
    let arg_parser: ArgParser = ArgParser::new(input_args);

    match arg_parser.parse() {
        Ok(cmd) => {
            Ok(dispatch(cmd, &arg_parser))
        },
        Err(err) => Err(err)
    }

}

