use rtor::LoggableError;
use rtor::config::load_config;

fn exit_with_error(error: LoggableError) {
    eprintln!("error[{}]: {}", error.label, error.message);
    std::process::exit(1);
}

async fn actual_main() -> Result<(), LoggableError> {
    let config = load_config()?;
    println!("incoming address {}:{}", config.incoming_address, config.incoming_port);
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = actual_main().await {
        exit_with_error(error);
    }
}
