use crate::models::user::User;

pub fn get_all_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ]
}

pub fn create_user(user: User) -> User {
    User { id: 3, ..user }
}
