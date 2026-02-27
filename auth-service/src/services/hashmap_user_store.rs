// UserStoreError is an error enum we'll use to represent all the possible ways our data store may fail.
//  - It derives the Debug and PartailEq traits so that we can compare different instances, which is useful for testing.

// We are only implementing basic error handling in this sprint. More robust and idiomatic error handling will be the focus of a future sprint.

// HashmapUserStore is our user store implementation.
//  - It derives the Default trait so we can easily create new instances that contain an empty hashmap.

// We are not hashing passwords. This will be implemented in a later sprint!

// TODO:
// ✅ 1. Create a new struct called `HashmapUserStore` containing: users (stores a HashMap of email Strings mapped to User objects).
// ✅ 2. Derive the `Default` trait for `HashmapUserStore`.
// ✅ 3. Implement a public method called `get_user` (takes &self,  and &str [email] as arguments) -> returns Result<User, UserStoreError [UserStoreError::UserNotFound]>
// ✅ 4. Implement a public method called `validate_user` (takes &self, &str [email], &str [password]) -> returs Result<(), UserStoreError> [`UserStoreError::UserNotFound`, UserStoreError::InvalidCredentials]
// ✅ 5. Implement a pubic method called `add_user` (takes &mut self, User [a user]) -> returns Result<(), UserStoreError> [`UserStoreError::UserAlreadyExists`]
// ✅ 6. Add unit tests for your `HashmapUserStore` implementation
// ✅ 7. update the User struct to derive a few traits for the unit tests to pass.

use crate::domain::User;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

impl HashmapUserStore {
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(matched_user) => Ok(User {
                email: matched_user.email.clone(),
                password: matched_user.password.clone(),
                requires_2fa: matched_user.requires_2fa.clone(),
            }),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.get_user(&user.email).is_ok() {
            true => return Err(UserStoreError::UserAlreadyExists),
            false => {
                self.users.entry(user.email.clone()).or_insert(user);
                Ok(())
            }
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.get_user(email) {
            Ok(returned_user) => match returned_user.password == password {
                true => return Ok(()),
                false => return Err(UserStoreError::InvalidCredentials),
            },
            Err(returned_error) => return Err(returned_error),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //Note: check turbo fish operator
    #[tokio::test]
    async fn test_get_user() {
        let test_user: User = User::new(
            String::from("webmaster@example.greenpantsblackshirt.dev"),
            String::from("AcQY-iK9T-aNi6-OOlN-esU"),
            false,
        );

        let mut user_hashmap: HashMap<String, User> = HashMap::new();
        user_hashmap.insert(test_user.email.clone(), test_user.clone());
        let user_store: HashmapUserStore = HashmapUserStore {
            users: user_hashmap,
        };

        assert_eq!(test_user, user_store.get_user(&test_user.email).unwrap());
    }

    #[tokio::test]
    async fn test_add_user() {
        let user_hashmap: HashMap<String, User> = HashMap::new();
        let mut user_store: HashmapUserStore = HashmapUserStore {
            users: user_hashmap,
        };

        let test_user: User = User::new(
            String::from("webmaster@example.greenpantsblackshirt.dev"),
            String::from("AcQY-iK9T-aNi6-OOlN-esU"),
            false,
        );

        let _ = user_store.add_user(test_user.clone()); // We only care about adding the user
        assert_eq!(test_user, user_store.get_user(&test_user.email).unwrap());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let user_hashmap: HashMap<String, User> = HashMap::new();
        let mut user_store: HashmapUserStore = HashmapUserStore {
            users: user_hashmap,
        };

        let test_user: User = User::new(
            String::from("webmaster@example.greenpantsblackshirt.dev"),
            String::from("AcQY-iK9T-aNi6-OOlN-esU"),
            false,
        );

        let _ = user_store.add_user(test_user.clone()); // We only care about adding the user
        assert_eq!(
            (),
            user_store
                .validate_user(&test_user.email, &test_user.password)
                .unwrap()
        );
    }
}
