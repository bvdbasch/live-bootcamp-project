Basically the best Rust lesson ever!

```rust
// Alternative implementation that returns a reference to a user (so still tied to the hashmap)
// pub fn get_user_return_ref(&self, email: &str) -> Result<&User, UserStoreError> {
//     match &self.users.get(email) {
//         Some(some_user) => Ok(some_user),
//         None => Err(UserStoreError::UserNotFound),
//     }
// }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        // match self.get_user(email) {
        //     Ok(returned_user) => {
        //         if returned_user.password != password {
        //             return Err(UserStoreError::InvalidCredentials);
        //         }
        //         return Ok(());
        //     }
        //     Err(returned_error) => return Err(returned_error),
        // };

        let validated = match self.get_user(email) {
            Ok(returned_user) => returned_user.password == password,
            Err(returned_error) => return Err(returned_error),
        };

        match validated {
            true => return Ok(()),
            false => return Err(UserStoreError::InvalidCredentials),
        }

        // if !bestaat
        // -> return bestaat niet
        //

        // if !password is goed
        // -> return verkeerde password gast
        //

        // alles ok
        // -> return ok

        match self.get_user(email) {
            Ok(returned_user) => match returned_user.password == password {
                true => return Ok(()),
                false => return Err(UserStoreError::InvalidCredentials),
            },
            Err(returned_error) => return Err(returned_error),
        };

        // Ok(())
        // == loosely matched  Co == co prima === Nope
        // try to get user -> re <T, E>
        // match
        //  some
        //      validate -> string != string -> retrun err
        //      return ();
        //  none
        //      return notfounderror
        //

        // if let Err(returned_error) = self.get_user(email) {
        //     return Err(returned_error);
        // }

        // let scoped_user = self.get_user(email);

        // Ok(())
    }
```
