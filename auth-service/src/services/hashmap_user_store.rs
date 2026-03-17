use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default, Debug)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.get_user(user.email.clone()).await.is_ok() {
            true => return Err(UserStoreError::UserAlreadyExists),
            false => {
                self.users.entry(user.email.clone()).or_insert(user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        match self.users.get(&email) {
            Some(matched_user) => Ok(User {
                email: matched_user.email.clone(),
                password: matched_user.password.clone(),
                requires_2fa: matched_user.requires_2fa.clone(),
            }),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    // My version did not have references:
    // async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        // My version had:
        // match self.get_user(email).await {
        //     Ok(returned_user) => match returned_user.password == password {
        //         true => return Ok(()),
        //         false => return Err(UserStoreError::InvalidCredentials),
        //     },
        //     Err(returned_error) => return Err(returned_error),
        // };
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //Note: check turbo fish operator
    #[tokio::test]
    async fn test_get_user() {
        let test_user: User = User::new(
            Email::parse(String::from("webmaster@example.greenpantsblackshirt.dev")).unwrap(),
            Password::parse(String::from("AcQY-iK9T-aNi6-OOlN-esU")).unwrap(),
            false,
        );

        let mut user_hashmap: HashMap<Email, User> = HashMap::new();
        user_hashmap.insert(test_user.email.clone(), test_user.clone());
        let user_store: HashmapUserStore = HashmapUserStore {
            users: user_hashmap,
        };

        assert_eq!(
            test_user,
            user_store.get_user(test_user.email.clone()).await.unwrap()
        );
    }

    #[tokio::test]
    async fn test_add_user() {
        // My version had:
        // let user_hashmap: HashMap<Email, User> = HashMap::new();
        // let mut user_store: HashmapUserStore = HashmapUserStore {
        //     users: user_hashmap,
        // };
        let mut user_store = HashmapUserStore::default();

        // My version had:
        // let test_user: User = User::new(
        //     Email::parse(String::from("webmaster@example.greenpantsblackshirt.dev")),
        //     Password::parse(String::from("AcQY-iK9T-aNi6-OOlN-esU")).unwrap(),
        //     false,
        // );
        let user = User {
            email: Email::parse("test@example.com".to_owned()).unwrap(),
            password: Password::parse("password".to_owned()).unwrap(),
            requires_2fa: false,
        };

        // Test adding a new user
        // My version had
        // let _ = user_store.add_user(test_user.clone()); // We only care about adding the user
        // assert_eq!(
        //     test_user,
        //     user_store.get_user(test_user.email.clone()).await.unwrap()
        // );
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        // Test adding an existing user
        // I did not have this at all as it was not mentioned in the assignment
        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_validate_user_lgr() {
        // My version had:
        // let user_hashmap: HashMap<Email, User> = HashMap::new();
        // let mut user_store: HashmapUserStore = HashmapUserStore {
        //     users: user_hashmap,
        // };
        let mut user_store = HashmapUserStore::default();

        // My version had
        // let test_user: User = User::new(
        //     Email::parse(String::from("webmaster@example.greenpantsblackshirt.dev")).unwrap(),
        //     Password::parse(String::from("AcQY-iK9T-aNi6-OOlN-esU")).unwrap(),
        //     false,
        // );
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let password = Password::parse("password".to_owned()).unwrap();
        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };

        // After adding my version had
        // There is a big difference in the validation scenarios here
        // Also the validate method breaks with the LGR version in this file stating that the validate_user arguments are incorrect
        // let _ = user_store.add_user(test_user.clone()); // We only care about adding the user
        // assert_eq!(
        //     (),
        //     user_store
        //         .validate_user(test_user.email.clone(), test_user.password.clone())
        //         .await
        //         .unwrap()
        // );

        // Test validating a user that exists with correct password
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        // Test validating a user that exists with incorrect password
        let wrong_password = Password::parse("wrongpassword".to_owned()).unwrap();
        let result = user_store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validating a user that doesn't exist
        let result = user_store
            .validate_user(
                &Email::parse("nonexistent@example.com".to_string()).unwrap(),
                &password,
            )
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
