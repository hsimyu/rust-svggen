extern crate svggen;

fn main() {
    let mut builder = svggen::SvgBuilder::new();

    builder.begin();

    builder
        .rect("50%", "50%")
        .position(10, 10)
        .corner_radius(20, 20);

    builder.rect("50%", "20%").stroke_with_opacity("red", 0.5);
    builder
        .rect("20%", "20%")
        .position(20, 40)
        .stroke_with_linecap("red", 0.5, svggen::attribute::StrokeLinecap::Round);

    builder.circle(150, 100, 80);
    builder.text(150, 125, 60, "SVG");

    builder.path();

    builder.end();

    builder.save_as("myimage.svg").unwrap();
}
