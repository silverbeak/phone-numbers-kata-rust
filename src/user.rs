pub mod user {
    use std::io::Error;

    #[derive(Debug, Clone)]
    pub struct User {
        pub name: String,
        pub phone: String
    }

    impl User {
        fn clean_phone_number(phone: &str) -> String {
            phone.replace(" ", "").replace("-", "")
        }
    }

    impl<'a> From<&'a Result<String, Error>> for User {
        fn from(line: &'a Result<String, Error>) -> Self {
            match line {
                &Ok(ref line) => {
                    let name_and_phone: Vec<&str> = line.split(",").collect();
                    let phone = User::clean_phone_number(name_and_phone[1]);
                    User { name: String::from(name_and_phone[0]), phone: phone }
                },
                &Err(ref e) => panic!(format!("Could not extract user. {}", e))
            }
        }
    }


    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_user_from_line() {
            let line = "Fader Abraham,123-456 789";
            let user = User::from(Ok(String::from(line)));
            assert!(user.name == "Fader Abraham");
            assert!(user.phone == "123456789");
        }

        #[test]
        fn test_clean_phone_number() {
            let result = User::clean_phone_number("12-3 4");
            assert!(result == "1234");
        }
    }
}

