use crate::helpers::spawn_app;

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    // Act
    let response = app.post_subscriptions(body.into()).await;
    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            "name=&email=ursula_le_guin%40gmail.com",
            "empty name",
            400u16,
        ),
        ("name=Ursula&email=", "empty email", 400u16),
        (
            "name=Ursula&email=definitely-not-an-email",
            "invalid email",
            400u16,
        ),
    ];

    // Act
    for (body, description, status) in test_cases {
        let response = app.post_subscriptions(body.into()).await;
        // Assert
        assert_eq!(
            status,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        );
    }
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        // Act
        let response = app.post_subscriptions(body.into()).await;
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {},response body:{:?}.",
            description,
            response.text().await.expect("获取返回值异常")
        );
    }
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error_message) in test_cases {
        // Act
        let response = app.post_subscriptions(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        )
    }
}
