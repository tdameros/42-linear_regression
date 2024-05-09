use clap::{arg, Parser};
use linear_regression::LinearModel;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version = "1.0", about, long_about = None)]
struct Args {
    /// Predicted x value
    #[arg(index = 1)]
    x_value: f64,

    /// CSV model
    #[arg(index = 2)]
    model_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let model = LinearModel::load(&args.model_path)?;
    println!(
        "Estimate value for {} (x): {} (y)",
        args.x_value,
        model.estimate(args.x_value)
    );
    Ok(())
}
