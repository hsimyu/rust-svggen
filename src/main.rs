extern crate svggen;

fn main() {
    let mut builder = svggen::SvgBuilder::new();

    builder.begin();

    builder
        .rect("50%", "50%")
        .position(10, 10)
        .corner_radius(20, 20);

    builder.rect("50%", "20%").stroke("red", 0.5);

    builder.circle(150, 100, 80);
    builder.text(150, 125, 60, "SVG");

    builder.end();

    builder.save_as("myimage.svg").unwrap();
}
