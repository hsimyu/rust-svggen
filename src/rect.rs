use super::attribute;
use super::SvgBuilder;

pub struct RectBuilder<'a> {
    parent: &'a mut SvgBuilder,
    width: &'a str,
    height: &'a str,

    position: Option<attribute::Position>,
    radius: Option<attribute::Radius>,
    fill: Option<attribute::Fill>,
    stroke: Option<attribute::Stroke>,
}

impl RectBuilder<'_> {
    pub fn new<'a>(parent: &'a mut SvgBuilder, width: &'a str, height: &'a str) -> RectBuilder<'a> {
        RectBuilder {
            parent,
            width,
            height,
            position: None,
            radius: None,
            fill: None,
            stroke: None,
        }
    }

    pub fn position(&mut self, x: u32, y: u32) -> &mut Self {
        self.position = Some(attribute::Position { x, y });
        self
    }

    pub fn corner_radius(&mut self, rx: u32, ry: u32) -> &mut Self {
        self.radius = Some(attribute::Radius { rx, ry });
        self
    }

    // background-color
    pub fn fill(&mut self, color: &str, opacity: f32) -> &mut Self {
        let mut color_s = String::new();
        color_s.push_str(color);
        self.fill = Some(attribute::Fill {
            color: color_s,
            opacity: Some(opacity),
        });
        self
    }

    // border
    // TODO: linecap を指定したいけど opacity を指定したくないときは…？
    pub fn stroke(&mut self, color: &str) -> &mut Self {
        let stroke = attribute::Stroke::new(color);
        self.stroke = Some(stroke);
        self
    }

    pub fn stroke_with_opacity(&mut self, color: &str, opacity: f32) -> &mut Self {
        let mut stroke = attribute::Stroke::new(color);
        stroke.opacity = Some(opacity);
        self.stroke = Some(stroke);
        self
    }

    pub fn stroke_with_linecap(
        &mut self,
        color: &str,
        opacity: f32,
        linecap: attribute::StrokeLinecap,
    ) -> &mut Self {
        let mut stroke = attribute::Stroke::new(color);
        stroke.opacity = Some(opacity);
        stroke.linecap = Some(linecap);
        self.stroke = Some(stroke);
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

        if let Some(f) = self.fill.as_ref() {
            f.emit(&mut elem);
            elem += " ";
        }

        if let Some(s) = self.stroke.as_ref() {
            s.emit(&mut elem);
            elem += " ";
        }

        elem += "/>\n";
        self.parent.content.push_str(&elem);
    }
}
