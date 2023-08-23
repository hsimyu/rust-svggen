use super::SvgBuilder;
use super::attribute;

pub struct RectBuilder<'a> {
    parent: &'a mut SvgBuilder,
    width: &'a str,
    height: &'a str,

    position: Option<attribute::Position>,
    radius: Option<attribute::Radius>
}

impl RectBuilder<'_> {
    pub fn new<'a>(parent: &'a mut SvgBuilder, width: &'a str, height: &'a str) -> RectBuilder<'a>
    {
        RectBuilder { parent, width, height, position: None, radius: None }
    }

    pub fn position(&mut self, x: u32, y: u32) -> &mut Self
    {
        self.position = Some( attribute::Position { x, y });
        self
    }

    pub fn corner_radius(&mut self, rx: u32, ry: u32) -> &mut Self
    {
        self.radius = Some( attribute::Radius { rx, ry });
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
