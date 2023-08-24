pub struct Position
{
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn emit(&self, builder: &mut String) {
        builder.push_str(format!("x=\"{:}\" y=\"{:}\"", self.x, self.y).as_str())
    }
}

pub struct Radius
{
    pub rx: u32,
    pub ry: u32,
}

impl Radius {
    pub fn emit(&self, builder: &mut String) {
        builder.push_str(format!("rx=\"{:}\" ry=\"{:}\"", self.rx, self.ry).as_str())
    }
}

pub struct Fill
{
    pub color: String,
    pub opacity: f32,
}

impl Fill {
    pub fn emit(&self, builder: &mut String) {
        // TODO: opacity を Option に
        builder.push_str(format!("fill=\"{:}\" fill-opacity=\"{:}\"", self.color, self.opacity).as_str())
    }
}