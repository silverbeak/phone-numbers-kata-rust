
pub mod tree_node {
    use std::collections::BTreeMap;
    use user;

    #[derive(Debug)]
    pub struct TreeNode {
        xs: BTreeMap<u8, TreeNode>,
        user: Option<user::user::User>
    }

    pub trait InsertUser {
        fn insert_user(&mut self, user: &user::user::User, index: usize) -> Result<user::user::User, String>;
    }

    impl TreeNode {
        pub fn new() -> TreeNode {
            TreeNode {
                xs: BTreeMap::new(),
                user: None
            }
        }

        fn do_insert(&mut self, user: &user::user::User) -> Result<user::user::User, String> {
            if self.xs.len() > 0 {
                Err(format!("Wanted to insert {:?}, but someone was already there", user))
            } else {
                let user2 = user.clone();
                self.user = Some(user.clone());
                Ok(user2)
            }
        }

        fn init_tree_in_position(&mut self, first_digit: u8) {
            let tree = TreeNode::new();
            self.xs.insert(first_digit, tree);
        }
    }

    impl InsertUser for TreeNode {

        fn insert_user(&mut self, user: &user::user::User, index: usize) -> Result<user::user::User, String> {
            match self.user {
                Some(ref u) => Err(format!("User already here: {:?} - {:?}", u, user)),
                None => {
                    if user.phone.len() <= index {
                        // No more digits in number. Display warning if there is a user here, otherwise fill slot and return
                        self.do_insert(user)
                    } else {
                        let first_digit_char = user.phone.chars().nth(index).unwrap();
                        let first_digit = first_digit_char.to_string().parse::<u8>().unwrap();
                        
                        if !self.xs.contains_key(&first_digit) {
                            self.init_tree_in_position(first_digit);
                        }

                        match self.xs.get_mut(&first_digit) {
                            Some(x) => x.insert_user(user, index + 1),
                            None => panic!()
                        }
                    }
                }
            }
        }
        
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_tree_node() {
            let tree_node = TreeNode::new();
            assert!(tree_node.user.is_none());
            assert!(tree_node.xs.is_empty());
        }

        #[test]
        fn test_insert_user_successful() {
            let user = user::user::User { name: String::from("Rune Riddare"), phone: String::from("987654321") };
            let mut tree = TreeNode::new();
            match tree.insert_user(&user, 0) {
                Ok(u) => {
                    assert!(u.name == "Rune Riddare");
                    assert!(u.phone == "987654321");
                },
                Err(e) => assert!(false, format!("Insert should have succeeded! {}", e))
            }
        }

        #[test]
        fn test_insert_user_fail_existing_user_longer_number() {
            let user = user::user::User { name: String::from("Rune Riddare"), phone: String::from("987654321") };
            let user2 = user::user::User { name: String::from("Dag Hammarsköld"), phone: String::from("987") };
            let mut tree = TreeNode::new();
            let first_insert = tree.insert_user(&user, 0).unwrap();
            assert!(first_insert.name == "Rune Riddare");
            let user2_clone = user2.clone();
            match tree.insert_user(&user2, 0) {
                Ok(u) => assert!(false, format!("Insert should have failed for {:?}", u)),
                Err(e) => assert!(e == format!("Wanted to insert {:?}, but someone was already there", user2_clone))
            }
        }

        #[test]
        fn test_insert_user_fail_existing_user_shorter_number() {
            let user = user::user::User { name: String::from("Rune Riddare"), phone: String::from("987") };
            let user_clone = user.clone();
            let user2 = user::user::User { name: String::from("Dag Hammarsköld"), phone: String::from("987654321") };
            let mut tree = TreeNode::new();
            let first_insert = tree.insert_user(&user, 0).unwrap();
            assert!(first_insert.name == "Rune Riddare");
            let user2_clone = user2.clone();
            match tree.insert_user(&user2, 0) {
                Ok(u) => assert!(false, format!("Insert should have failed for {:?}", u)),
                Err(e) => assert!(e == format!("User already here: {:?} - {:?}", user_clone, user2_clone))
            }
        }
    }

}