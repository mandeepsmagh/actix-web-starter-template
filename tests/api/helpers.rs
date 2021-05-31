use template::configuration::get_configuration;
use template::startup::Application;

pub struct TestApp {
    pub address: String,
}

// Test app instance that will be used by all intergration tests
pub async fn spawn_app() -> TestApp {
    // Get configuration
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        // Use a random port
        c.application.port = 0;
        c
    };
    // build application
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");
    let application_port = application.port();
    // Using tokio to run app as background task for testing
    let _ = tokio::spawn(application.run_until_stopped());
    // Now return the application address to caller
    TestApp {
        address: format!("http://localhost:{}", application_port),
    }
}
