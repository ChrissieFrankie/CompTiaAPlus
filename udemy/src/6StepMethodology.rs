use std::io::{self, Write}; // to allow reading from stdin and writing to stdout

fn prompt(msg: &str) -> String { // to prompt the user for input
    print!("{}", msg);
    io::stdout().flush().unwrap(); // to flush the output buffer
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap(); // to read the input from stdin
    buf.trim().to_lowercase() // to trim the input and convert it to lowercase
}

const STEPS: &[(&str, &[&str])] = &[ // to store the steps and their keywords
    ("Identify the problem",                                    &["identify", "problem"]),
    ("Establish a theory of probable cause",                    &["establish", "theory", "probable", "cause"]),
    ("Test the theory to determine the cause",                  &["test", "theory", "determine", "cause"]),
    ("Establish a plan of action and implement the solution",   &["establish", "plan", "implement", "solution"]),
    ("Verify full system functionality and preventive measures",&["verify", "preventive", "functionality", "measures"]),
    ("Document findings, actions, and results",                   &["document", "findings", "actions", "results"]),
];

const ASCII_ART: &[(i32, &str)] = &[
  (1, r#"
       _________________
      /                /|
     /                / |
    /________________/  |
   |   __________   |   |       ____
   |  |   X  X   |  |   |    _.'    '._
   |  |    ~~    |  |   |  .'  _______  '.
   |  |__________|  |   |  |  |       |  |
   |________________|  /   |  | [i]   |  |
   |     ______     | /    |  |_______|  |
   |    |______|    |/     '.           .'
   |________________|        '._     _.'
  /                /            '---' \
 /________________/                    \
                                        \
  "#),
  (2, r#"
       ___________
      /          /|
     /__________/ |
    |  ______  |  |
    | |      | |  |
    | |  [T] | |  | 
    | |______| |  |
    |  ______  |  |
    | |      | |  |
    | |  ==  | |  |
    | |______| |  |
    |__________| /
  "#),
  (3, r#"
      ___________________
     |   _____________   | 
     |  /      |      \  | 
     | |       |       | | 
     | |       V       | |
     |  \_____________/  |          /|
     |      [ 12v ]      |         / |
     |   _     _     _   |        /  |
     |  (_)   (_)   (_)  |-------{:::|===>
     |___________________|        \  |
                                   \ |
                                    \|
  "#),
  (4, r#"
      ___________
      \         /
       )_______(
       |"""""""|_.-._,.---------.,_.-._
       |       | | |               | | ''-.
       |       |_| |_             _| |_..-'
       |_______| '-' `'---------'` '-'
       )"""""""(
      /_________\
      `'-------'`
    .-------------.
   /_______________\
  "#),
  (5, r#"
      _____________
     |             |
     |      _      |
     |    _| |_    |
     |   |_   _|   |
     |     |_|     |
     |_____________|
    /               \
   /_________________\
   |                 |
   |      _____      |
   |     |  |  |     |
   |     |__|__|     |
   |_________________|
  "#),
  (6, r#"
      ___________
     |  _______  |
     | |       | |
     | |  ---  | |
     | |  ---  | |
     | |  ---  | |
     | |_______| |
     |___________|
    /           /
   /           /
  /___________/
 |___________|
  "#),  
];

fn main() {
    println!("\x1B[2J\x1B[1;1H"); // to clear the terminal
    println!("Enter some words per step to show you know you're familiar with the steps!\n");
    for (i, (title, keywords)) in STEPS.iter().enumerate() { // to iterate over the steps and their keywords
        loop {
            let input = prompt(&format!("Step {}: ", i + 1));
            if keywords.iter().all(|kw| input.contains(kw)) { // to check if the input contains any of the keywords
                println!("  Correct -- {}\n", title);
                print!("{}", ASCII_ART[i].1);
                break;
            } else {
                println!("  Wrong. You need to study the steps more!\n"); // to print the alert message
                for (j, (t, _)) in STEPS.iter().enumerate() { // to iterate over the steps and their keywords
                    println!("  Step {}. {}", j + 1, t); // to print the step number and title
                }
                println!(); // to print a new line
                return; // to exit the program
            }
        }
    } // end for loop

    println!("You're familiar with the 6-step methodology."); // to print the success message
}