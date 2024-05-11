use std::error::Error;

use plotters::prelude::*;

use crate::dataset::Dataset;
use crate::linear_model::LinearModel;

const PLOT_BACKGROUND: RGBColor = RGBColor(236, 236, 243);
const PLOT_TITLE: &str = "Linear Regression";
const PLOT_MARGIN: i32 = 20;
const PLOT_DATA_CIRCLE_COLOR: RGBColor = RGBColor(92, 157, 255);
const PLOT_DATA_CIRCLE_RADIUS: i32 = 5;
const PLOT_WIDTH: u32 = 1000;
const PLOT_HEIGHT: u32 = 800;

pub fn plot_linear_model(
    linear_model: &LinearModel,
    dataset: &Dataset,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new(path, (PLOT_WIDTH, PLOT_HEIGHT)).into_drawing_area();

    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(PLOT_TITLE, ("sans-serif", 30))
        .margin(PLOT_MARGIN)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(
            dataset.get_x_min()..dataset.get_x_max(),
            dataset.get_y_min()..dataset.get_y_max(),
        )?;

    chart.draw_series(std::iter::once(Rectangle::new(
        [
            (dataset.get_x_min(), dataset.get_y_min()),
            (dataset.get_x_max(), dataset.get_y_max()),
        ],
        PLOT_BACKGROUND.filled(),
    )))?;

    let bold_white_style = ShapeStyle::from(&WHITE).stroke_width(2);

    chart
        .configure_mesh()
        .light_line_style(PLOT_BACKGROUND)
        .bold_line_style(bold_white_style)
        .axis_style(PLOT_BACKGROUND)
        .x_labels(10)
        .y_labels(10)
        .draw()?;

    let circle_style = Into::<ShapeStyle>::into(PLOT_DATA_CIRCLE_COLOR).filled();

    chart
        .draw_series(
            dataset
                .into_iter()
                .map(|(x, y)| Circle::new((*x, *y), PLOT_DATA_CIRCLE_RADIUS, circle_style)),
        )?
        .label("Data")
        .legend(|(x, y)| {
            Circle::new(
                (x + PLOT_DATA_CIRCLE_RADIUS, y),
                PLOT_DATA_CIRCLE_RADIUS,
                Into::<ShapeStyle>::into(PLOT_DATA_CIRCLE_COLOR).filled(),
            )
        });

    chart
        .draw_series(LineSeries::new(
            [
                (
                    dataset.get_x_min(),
                    linear_model.estimate(dataset.get_x_min()),
                ),
                (
                    dataset.get_x_max(),
                    linear_model.estimate(dataset.get_x_max()),
                ),
            ]
            .iter()
            .cloned(),
            BLACK.stroke_width(2),
        ))?
        .label("Regression Line")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK.stroke_width(2)));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .draw()?;
    Ok(())
}
