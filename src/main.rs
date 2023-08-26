extern crate svggen;

fn main() {
    let mut builder = svggen::SvgBuilder::new();

    builder.begin();

    builder
        .path()
        .move_to(10, 10)
        .horizontal_line_to(90)
        .vertical_line_to(90)
        .horizontal_line_to(10)
        .line_to(10, 10);

    builder.end();

    builder.save_as("myimage.svg").unwrap();
}
