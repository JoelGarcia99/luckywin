use crate::config::env::SecretsVariables;

pub fn build_tracing(secrets: &SecretsVariables) {
    // defining logger
    let mut subscriber = tracing_subscriber::fmt();

    if secrets.environment == "dev" {
        subscriber = subscriber.with_max_level(tracing::Level::DEBUG);
    } else {
        subscriber = subscriber
            .with_max_level(tracing::Level::INFO)
            .with_file(true);
    }

    // initializing right subscriber
    tracing::subscriber::set_global_default(subscriber.finish()).unwrap();
}
