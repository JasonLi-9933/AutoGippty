use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
}; // used for coloring the command text
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statment: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the print color
        let statment_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statment in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        // Reset color
        stdout.execute(SetForegroundColor(statment_color)).unwrap();
        print!("{} \n", agent_statment);

        stdout.execute(ResetColor).unwrap();
    }
}

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespaces then return
    user_response.trim().to_string()
}

// Get user response that code is safe to execute
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        println!("WARNING: You are about to run code written entirely by AI");
        println!("Review your code and confirm you wish to continue.");

        stdout.execute(ResetColor).unwrap();
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good!");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Let's stop this project!");

        stdout.execute(ResetColor).unwrap();
        let mut user_response: String = String::new();
        stdin()
            .read_line(&mut user_response)
            .expect("Failed to read response");

        let user_response_trimmed = user_response.trim().to_lowercase();

        match user_response_trimmed.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select 1 or 2");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_print_agent_msg() {
        PrintCommand::AICall.print_agent_message("Managing Agent", "Testing")
    }
}
