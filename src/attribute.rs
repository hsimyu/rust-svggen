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

pub enum StrokeLinecap {
    Butt,
    Square,
    Round,
}

pub struct Stroke
{
    pub color: String,
    pub opacity: Option<f32>,
    pub linecap: Option<StrokeLinecap>,
}

impl Stroke {
    pub fn emit(&self, builder: &mut String) {
        builder.push_str(format!("stroke=\"{:}\"", self.color).as_str());

        if let Some(op) = self.opacity.as_ref() {
            builder.push_str(format!("stroke-opacity=\"{:}\"", op).as_str());
        }

        if let Some(linecap) = self.linecap.as_ref() {
            match linecap {
                StrokeLinecap::Butt => builder.push_str("stroke-linecap=\"butt\""),
                StrokeLinecap::Square => builder.push_str("stroke-linecap=\"square\""),
                StrokeLinecap::Round => builder.push_str("stroke-linecap=\"round\""),
            }
        }
    }
}