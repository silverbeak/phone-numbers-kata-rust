use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
mod user;
mod tree_node;
use tree_node::tree_node::InsertUser;
use std::env;

fn read_file(file_name: &str) -> BufReader<File> {
    let file = match File::open(Path::new(file_name)) {
        Err(why) => panic!("Could not read file. {}", why),
        Ok(file) => file
    };
    BufReader::new(file)
}

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let file = read_file(&file_name);
    let mut tree = tree_node::tree_node::TreeNode::new();
    
    let users: Vec<user::user::User> = file
        .lines()
        .map(|x| user::user::User::from(&x))
        .collect();
    
    let results: Vec<Result<user::user::User, String>> = users
        .iter()
        .skip_while(|x| x.name == "Name")
        .map(|x| tree.insert_user(x, 0))
        .collect();

    // Just printing the errors below. Main operation is already done by now.
    let failures: Vec<String> = results
        .iter()
        .filter_map(|x| x.clone().err())
        .collect();

    for f in &failures {
        println!("{}", f);
    }
    println!("Found {} conflicts", &failures.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_file() {
        let file = read_file("./src/assets/phone_data.txt");
        let count = file.lines().count();
        assert!(count == 1001);
    }
}
