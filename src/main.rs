use std::io;
use std::process::Command;

fn git_name() -> Vec<String> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let array: Vec<String> = guess.trim().split('/').map(String::from).collect();
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
 println!("Git branch {branch_name_from_link} created successfully");

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
 if !checkout_output.stderr.is_empty() {
     eprintln!("stderr: {}", String::from_utf8_lossy(&checkout_output.stderr));
 }

 // Check if the checkout command was successful
 if checkout_output.status.success() {
     println!("Switched to Git branch '{branch_name_from_link} successfully");
 } else {
     eprintln!("Failed to switch to Git branch");
 }
} else {
 eprintln!("Failed to create Git branch");
}

}

fn main() {
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