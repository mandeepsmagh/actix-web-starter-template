use template::configuration::get_configuration;
use template::startup::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read the configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
