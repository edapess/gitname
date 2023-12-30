extern crate colored;

use std::io;
use std::process::Command;
use colored::Colorize;

fn git_name() -> Vec<String> {
    let mut task_link = String::new();
io::stdin()
        .read_line(&mut task_link)
        .expect("Failed to read line");

let array: Vec<String> = task_link.trim().split('/').map(String::from).collect();
    array
}
fn create_branch_with_name (branch_name_from_link: String) {
 // create a new Git branch named "branch_name_from_link"
 let branch_output = Command::new("git")
 .arg("branch")
 .arg(branch_name_from_link.clone())
 .output()
 .expect("Failed to execute 'git branch' command");

// check if creating the branch was successful
if branch_output.status.success() {

    println!(
        "{} {} {}",
        "Git branch".cyan(),
        branch_name_from_link.green().italic(),
        "created successfully".cyan()
    );
 // switch to the new Git branch
 let checkout_output = Command::new("git")
     .arg("checkout")
     .arg(branch_name_from_link.clone())
     .output()
     .expect("Failed to execute 'git checkout' command");

 // Print the output of the checkout command (if any)
 if !checkout_output.stdout.is_empty() {
     println!("stdout: {}", String::from_utf8_lossy(&checkout_output.stdout));
 }

 // Print the error of the checkout command (if any)
 if !checkout_output.status.success() {
    eprintln!(
        "{} {} {}",
        "Error:".red(),
        "stderr:".red(),
        String::from_utf8_lossy(&checkout_output.stderr).red()
    );
 }

 // Check if the checkout command was successful
 if checkout_output.status.success() {

     println!(
        "{} {} {}",
        "Switched to Git branch".cyan(),
        branch_name_from_link.green().italic(),
        "successfully".cyan()
    );
 } else {
     eprintln!("{}","Failed to switch to Git branch".red());
 }
} else {
 eprintln!("{}","Failed to create Git branch".red());
}

}

fn main() {
    println!("Please enter your your task link from Linear: ");

    let result = git_name();
        if result.len() >= 2 {
        let last_two: Vec<&String> = result.iter().rev().take(2).rev().collect();
        let branch_name = last_two.iter().enumerate().map(|(index, s)|{
            if index == 1 {
                s.to_lowercase()
            }else{
                s.to_string()
            }
        }).collect::<Vec<String>>().join("-");
        create_branch_with_name(branch_name);
    } else {
        println!("please check your string");
    }
}