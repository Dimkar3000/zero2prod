use zero2prod::health_check;

#[actix::test]
async fn health_check_succeeds() {
    let response = health_check().await;

    assert!(response.status().is_success())
}
