use super::SvgBuilder;

struct Position
{
    x: u32,
    y: u32,
}

impl Position {
    fn emit(&self, builder: &mut String) {
        builder.push_str(format!("x=\"{:}\" y=\"{:}\"", self.x, self.y).as_str())
    }
}

struct Radius
{
    rx: u32,
    ry: u32,
}

impl Radius {
    fn emit(&self, builder: &mut String) {
        builder.push_str(format!("rx=\"{:}\" ry=\"{:}\"", self.rx, self.ry).as_str())
    }
}

pub struct RectBuilder<'a> {
    parent: &'a mut SvgBuilder,
    width: &'a str,
    height: &'a str,

    position: Option<Position>,
    radius: Option<Radius>
}

impl RectBuilder<'_> {
    pub fn new<'a>(parent: &'a mut SvgBuilder, width: &'a str, height: &'a str) -> RectBuilder<'a>
    {
        RectBuilder { parent, width, height, position: None, radius: None }
    }

    pub fn position(&mut self, x: u32, y: u32) -> &mut Self
    {
        self.position = Some( Position { x, y });
        self
    }

    pub fn corner_radius(&mut self, rx: u32, ry: u32) -> &mut Self
    {
        self.radius = Some( Radius { rx, ry });
        self
    }
}

impl Drop for RectBuilder<'_> {
    fn drop(&mut self) {
        // String の最大容量を決め打ちで確保しておくことで生成を高速化できそう
        let mut elem = format!(
            "<rect width=\"{:}\" height=\"{:}\" ",
            self.width, self.height
        );

        if let Some(pos) = self.position.as_ref() {
            pos.emit(&mut elem);
            elem += " ";
        }

        if let Some(r) = self.radius.as_ref() {
            r.emit(&mut elem);
            elem += " ";
        }

        elem += "/>\n";
        self.parent.content.push_str(&elem);
    }
}
