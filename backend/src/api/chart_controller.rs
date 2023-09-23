use actix_web::{get, web, Responder};
use actix_files::NamedFile;
use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};

use crate::{database::models::Object, ops::object_ops::show_objects_by_statistic_id};

#[get("/visualize/{statistic_id}")]
pub async fn make_chart(info: web::Path<i32>) -> impl Responder {
    let stat_id = info.into_inner();
    let option_items = show_objects_by_statistic_id(stat_id);
    if option_items.is_none() {
        return NamedFile::open_async("").await;
    }
    let items: Vec<Object> = option_items.unwrap();
    let domain: Vec<String> = items.iter().map(|obj| obj.name.to_string()).collect();

    let max_count = items.iter().map(|obj| obj.counter).max().unwrap_or(0);
    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(domain)
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);
    // Create a linear scale that will interpolate values in [0.0, max_count as f32 + 3] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, max_count as f32 + 3.0])
        .set_range(vec![height - top - bottom, 0]);
    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    //let data1 = vec![("A", 90), ("B", 10), ("C", 30)];
    let data: Vec<(&str, i32)> = items
        .iter()
        .map(|obj| (obj.name.as_str(), obj.counter))
        .collect();
    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .load_data(&data)
        .unwrap();
    let mut chart_name: String = String::from("vertical-bar-chart");
    chart_name.push_str(&stat_id.to_string());
    chart_name.push_str(".svg");
    let mut save_path: String = String::from("./charts/");
    save_path.push_str(&chart_name); //where .svg will be stored
                                     // Generate and save the chart.

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Bar Chart"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Quantity")
        .add_bottom_axis_label("Items")
        .save(&save_path)
        .unwrap();

    return NamedFile::open_async(save_path).await;
}
