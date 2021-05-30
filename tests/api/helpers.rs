use std::net::TcpListener;

// Test app instance that will be used by all intergration tests
pub fn spawn_app() -> String {
    // Using port 0 to trigger an OS scan for available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the random port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = template::startup::run(listener).expect("Failed to bind address");
    // Using tokio::spawn to launch server as background task
    // Using the non-binding let _ as spawned future is not required
    let _ = tokio::spawn(server);
    // Return application address to the caller
    format!("http://127.0.0.1:{}", port)
}
