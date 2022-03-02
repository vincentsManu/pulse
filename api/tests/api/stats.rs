use crate::helpers::spawn_app;

#[tokio::test]
async fn stats_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_stats().await;

    // Assert
    assert!(response.status().is_success());
}
