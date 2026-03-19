use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

use crate::helpers::{get_random_email, TestApp};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({ "password": "password123" }),
        serde_json::json!({ "email": "mail@example.letsgetrusty.com"}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await; // call `post_signup`
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    // Create an array of invalid inputs.
    let invalid_inputs = [
        // empty email
        serde_json::json!({ "email": "", "password": "password123"}),
        // email does not contain @
        serde_json::json!({ "email": "", "password": "password123"}),
        // password smaller than 8 chars
        serde_json::json!({ "email": get_random_email(), "password": "1234"}),
    ];

    // Then, iterate through the array and make HTTP calls to the signup route.
    //Assert a 400 HTTP status code is returned.
    for body in invalid_inputs.iter() {
        let response = app.post_login(body).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", body);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.

    // At this point there is no user yet, so ANY valid request will yield the incorrect credentials error!
    let app = TestApp::new().await;
    // let payload = serde_json::json!({ "email": get_random_email(), "password": "1234"});
    // let response = app.post_login(&payload).await;
    // assert_eq!(response.status().as_u16(), 401, "Failed for input: {:?}", payload);

    // Create an array of valid inputs.
    let valid_users = [serde_json::json!({ "email": get_random_email(), "password": "password1234"})];

    // Then, iterate through the array and make HTTP calls to the signup route.
    //Assert a 401 HTTP status code is returned.
    for body in valid_users.iter() {
        let response = app.post_login(body).await;
        assert_eq!(response.status().as_u16(), 401, "Failed for input: {:?}", body);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Incorrect credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());
}
