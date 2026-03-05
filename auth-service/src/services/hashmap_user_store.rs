use crate::domain::{User, UserStore, UserStoreError};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(matched_user) => Ok(User {
                email: matched_user.email.clone(),
                password: matched_user.password.clone(),
                requires_2fa: matched_user.requires_2fa.clone(),
            }),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.get_user(&user.email).await.is_ok() {
            true => return Err(UserStoreError::UserAlreadyExists),
            false => {
                self.users.entry(user.email.clone()).or_insert(user);
                Ok(())
            }
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.get_user(email).await {
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

        assert_eq!(
            test_user,
            user_store.get_user(&test_user.email).await.unwrap()
        );
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
        assert_eq!(
            test_user,
            user_store.get_user(&test_user.email).await.unwrap()
        );
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
                .await
                .unwrap()
        );
    }
}
