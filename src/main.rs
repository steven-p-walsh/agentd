use agentd::cli::run_cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_cli()?;
    Ok(())
}