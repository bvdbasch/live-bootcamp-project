use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use auth_service::app_state::{AppState, UserStoreType};
use auth_service::domain::User;
use auth_service::services::HashmapUserStore;
use auth_service::Application;

#[tokio::main]
async fn main() {
    // Make empty user store hashmap first
    let user_hashmap: HashMap<String, User> = HashMap::new();
    let hashmap_user_store: HashmapUserStore = HashmapUserStore {
        users: user_hashmap,
    };

    // Now feed it to the user_store
    let user_store: UserStoreType = Arc::new(RwLock::new(hashmap_user_store));

    // Lastly, create a new app state
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
