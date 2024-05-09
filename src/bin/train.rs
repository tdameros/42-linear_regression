use clap::{arg, Parser};
use linear_regression::{plotter, Dataset, LinearModel};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version = "1.0", about, long_about = None)]
struct Args {
    /// CSV dataset
    #[arg(index = 1)]
    dataset_path: String,

    /// Output model path
    #[arg(short, long, default_value = "linear_model.csv")]
    output_model_path: String,

    /// Plot path
    #[arg(long, default_value = "plot.png")]
    plot_path: String,

    /// Number of iterations
    #[arg(short, long, value_parser = check_iterations, default_value = "10000", )]
    iterations: usize,

    /// Learning rate
    #[arg(short, long, default_value = "0.01")]
    learning_rate: f64,

    /// Plot the dataset and the model
    #[arg(long)]
    plot: bool,

    /// Print the model precision (Mean Absolute Percentage Error)
    #[arg(long)]
    precision: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut model = LinearModel::new(args.learning_rate);
    let mut dataset = Dataset::load(&args.dataset_path)?;
    dataset.normalize();
    model.train(&dataset, args.iterations);
    model.save(&args.output_model_path)?;
    model.denormalize(&dataset);
    dataset.denormalize();

    println!("{:?}", model);
    if args.plot {
        plotter::plot_linear_model(&model, &dataset, &args.plot_path)?;
    }
    if args.precision {
        println!(
            "Mean Absolute Percentage Error: {:.2}%",
            model.mean_absolute_percentage_error(&dataset) * 100.
        );
    }
    Ok(())
}

fn check_iterations(value: &str) -> Result<usize, String> {
    match value.parse::<usize>() {
        Ok(iterations) if iterations > 0 => Ok(iterations),
        _ => Err("Number of iterations must be greater than 0".to_string()),
    }
}
