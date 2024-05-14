use dotenv::dotenv;
use serde::Deserialize;
use tokio::sync::OnceCell;
use crate::configuration::adyen::AdyenConfiguration;
use crate::configuration::application::ApplicationConfiguration;
use crate::configuration::auth0::Auth0Configuration;
use crate::configuration::database::DatabaseConfiguration;
use crate::configuration::environment::Environment;
use crate::configuration::footprint::FootprintConfiguration;
use crate::configuration::lithic::LithicConfiguration;
use crate::configuration::otel::OtelConfiguration;
use crate::configuration::redis::RedisConfiguration;

static CONFIGURATION: OnceCell<Configuration> = OnceCell::const_new();

#[derive(Deserialize)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
    pub redis: RedisConfiguration,
    pub application: ApplicationConfiguration,
    pub footprint: FootprintConfiguration,
    pub adyen: AdyenConfiguration,
    pub auth0: Auth0Configuration,
    pub otel: OtelConfiguration,
    pub lithic: LithicConfiguration
}


pub fn get_configuration_sync() -> Result<Configuration, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    dotenv::from_filename(format!("{}.env", environment.as_string())).expect("Unable to load expected env file");
    let environment_filename = format!("application-{}.yaml", environment.as_string());
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join(&environment_filename)))
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;

    settings.try_deserialize::<Configuration>()
}

async fn get_configuration_inner() -> Configuration {
    get_configuration_sync().expect("Configuration must load for application to run")
}

pub async fn get_global_configuration<'a>() -> &'a Configuration {
    CONFIGURATION.get_or_init(get_configuration_inner).await
}