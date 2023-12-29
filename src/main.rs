use std::io;

fn git_name() -> Vec<String> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let array: Vec<String> = guess.trim().split('/').map(String::from).collect();
    array
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
        println!("{:?}", branch_name);
    } else {
        println!("please check your string");
    }

}
// https://linear.app/chalkboard/issue/CHA-3625/typo-fixes-verification-screen