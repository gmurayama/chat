use api::server;
use prometheus_client::registry::Registry;

#[tokio::test]
async fn healthcheck_works() {
    let registry = Registry::default();

    let app = server::Server::setup(server::Settings {
        app: server::AppSettings {
            host: "127.0.0.1".to_string(),
            port: 0,
            request_timeout_sec: 10,
        },
        metrics: server::MetricSettings {
            host: "127.0.0.1".to_string(),
            port: 0,
            registry,
        },
    })
    .expect("failed to setup the server");
    let port = app.port();

    tokio::spawn(app.run());
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(format!("http://localhost:{}/healthcheck", port))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
