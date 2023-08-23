use super::SvgBuilder;

pub trait RectAttribute
{
    fn emit(&self, builder: &mut String);
}

struct Position
{
    x: u32,
    y: u32,
}

impl RectAttribute for Position {
    fn emit(&self, builder: &mut String) {
        builder.push_str(format!("x=\"{:}\" y=\"{:}\"", self.x, self.y).as_str())
    }
}

struct Radius
{
    rx: u32,
    ry: u32,
}

impl RectAttribute for Radius {
    fn emit(&self, builder: &mut String) {
        builder.push_str(format!("rx=\"{:}\" ry=\"{:}\"", self.rx, self.ry).as_str())
    }
}

pub struct RectBuilder<'a> {
    parent: &'a mut SvgBuilder,
    width: &'a str,
    height: &'a str,

    // TODO: HashMap のがよいだろう（もしくは dyn にしないで Box で持つ？）
    attributes: Vec<Box<dyn RectAttribute>>,
}

impl RectBuilder<'_> {
    pub fn new<'a>(parent: &'a mut SvgBuilder, width: &'a str, height: &'a str) -> RectBuilder<'a>
    {
        RectBuilder { parent, width, height, attributes: vec![] }
    }

    pub fn position(&mut self, x: u32, y: u32) -> &mut Self
    {
        self.add(Box::new(Position {x, y}));
        self
    }

    pub fn corner_radius(&mut self, rx: u32, ry: u32) -> &mut Self
    {
        self.add(Box::new(Radius{rx: rx, ry: ry}));
        self
    }

    fn add(&mut self, att: Box<dyn RectAttribute>)
    {
        self.attributes.push(att)
    }
}

impl Drop for RectBuilder<'_> {
    fn drop(&mut self) {
        // String の最大容量を決め打ちで確保しておくことで生成を高速化できそう
        let mut elem = format!(
            "<rect width=\"{:}\" height=\"{:}\" ",
            self.width, self.height
        );

        for att in self.attributes.iter() {
            att.emit(&mut elem);
            elem += " ";
        }

        elem += "/>\n";
        self.parent.content.push_str(&elem);
    }
}