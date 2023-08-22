extern crate svggen;

fn main() {
    let mut builder = svggen::SvgBuilder::new();

    builder.begin();

    builder.rect("100%", "100%").position(20, 80);
    builder.circle(150, 100, 80);
    builder.text(150, 125, 60, "SVG");

    builder.end();

    builder.save_as("myimage.svg").unwrap();
}
