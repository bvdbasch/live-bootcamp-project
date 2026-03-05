use crate::helpers::{get_random_email, TestApp};
use auth_service::{routes::SignupResponse, ErrorResponse};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({ "password": "password123","requires2FA": true }),
        serde_json::json!({ "requires2FA": true }),
        serde_json::json!({ "password": "password123"}),
        serde_json::json!({ "email": "mail@example.letsgetrusty.com"}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let req_body =
        serde_json::json!({ "email": random_email, "password": "password123","requires2FA": true});

    let response = app.post_signup(&req_body).await;

    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input: {:?}",
        response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    // Create an array of invalid inputs.
    let invalid_inputs = [
        // empty email
        serde_json::json!({ "email": "", "password": "password123","requires2FA": true}),
        // email does not contain @
        serde_json::json!({ "email": "", "password": "password123","requires2FA": true}),
        // password smaller than 8 chars
        serde_json::json!({ "email": get_random_email(), "password": "1234","requires2FA": true}),
    ];

    // Then, iterate through the array and make HTTP calls to the signup route.
    //Assert a 400 HTTP status code is returned.
    for body in invalid_inputs.iter() {
        println!("{body}");
        let response = app.post_signup(body).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            body
        );
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
async fn should_return_409_if_email_already_exists() {
    // Call the signup route twice. The second request should fail with a 409 HTTP status code
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let req_body =
        serde_json::json!({ "email": random_email, "password": "password123","requires2FA": true});

    // Should create user
    let response = app.post_signup(&req_body).await;
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input: {:?}",
        response
    );

    let response = app.post_signup(&req_body).await;
    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
