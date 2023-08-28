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
        .close()
        .fill("blue", 0.4)
        .stroke("red");

    builder
        .path()
        .move_to(10, 10)
        .bezier3(20, 20, 40, 20, 50, 10)
        .stroke("black")
        .fill("transparent", 0.0);

    builder
        .path()
        .move_to(70, 10)
        .bezier3(70, 20, 110, 20, 110, 10)
        .stroke("black")
        .fill("transparent", 0.0);

    builder
        .path()
        .move_to(110, 10)
        .bezier3_repeat(190, 20, 230, 10)
        .stroke("black")
        .fill("transparent", 0.0);

    builder
        .path()
        .move_to(230, 10)
        .bezier2(230, 80, 20, 80)
        .bezier2_repeat(240, 150)
        .stroke("black")
        .fill("transparent", 0.0);

    builder
        .path()
        .move_to(230, 20)
        .arc(svggen::path::ArcCommand {
            radius_x: 45,
            radius_y: 45,
            rotation_in_degree: 0,
            large_arc_flag: 0,
            sweep_flag: 1,
            x: 275,
            y: 275,
        })
        .line_to(275, 230)
        .close()
        .fill("purple", 1.0);

    builder.end();

    builder.save_as("myimage.svg").unwrap();
}
