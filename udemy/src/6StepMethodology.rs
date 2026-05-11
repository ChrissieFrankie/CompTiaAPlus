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
    ("Research knowledge base / internet",                      &["research", "knowledge", "base"]),
    ("Establish a theory of probable cause",                    &["establish", "theory", "probable", "cause"]),
    ("Test the theory to determine the cause",                  &["test", "theory", "determine", "cause"]),
    ("Establish a plan of action and implement the solution",   &["establish", "plan", "implement", "solution"]),
    ("Verify full system functionality and preventive measures",&["verify", "preventive", "functionality", "measures"]),
];

fn main() {
    println!("\x1B[2J\x1B[1;1H"); // to clear the terminal
    println!("Enter some words per step to show you know you're familiar with the steps!\n");

    for (i, (title, keywords)) in STEPS.iter().enumerate() { // to iterate over the steps and their keywords
        loop {
            let input = prompt(&format!("Step {}: ", i + 1));
            if keywords.iter().all(|kw| input.contains(kw)) { // to check if the input contains any of the keywords
                println!("  Correct -- {}\n", title);
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