use crate::attribute;
use crate::SvgBuilder;

pub struct PathBuilder<'a> {
    parent: &'a mut SvgBuilder,
    commands: Vec<Box<dyn PathCommand>>,

    // Options
    fill: Option<attribute::Fill>,
    stroke: Option<attribute::Stroke>,
}

trait PathCommand {
    fn emit(&self, builder: &mut String);
}

struct MoveToCommand {
    x: u32,
    y: u32,
}

impl PathCommand for MoveToCommand {
    fn emit(&self, builder: &mut String) {
        // TODO: Support relative
        builder.push_str(format!("M {:} {:}", self.x, self.y).as_str())
    }
}

struct LineToCommand {
    x: u32,
    y: u32,
}

impl PathCommand for LineToCommand {
    fn emit(&self, builder: &mut String) {
        // TODO: Support relative
        builder.push_str(format!("L {:} {:}", self.x, self.y).as_str())
    }
}

struct HorizontalLineToCommand {
    x: u32,
}

impl PathCommand for HorizontalLineToCommand {
    fn emit(&self, builder: &mut String) {
        // TODO: Support relative
        builder.push_str(format!("H {:}", self.x).as_str())
    }
}

struct VerticalLineToCommand {
    y: u32,
}

impl PathCommand for VerticalLineToCommand {
    fn emit(&self, builder: &mut String) {
        // TODO: Support relative
        builder.push_str(format!("V {:}", self.y).as_str())
    }
}

struct CloseCommand {}

impl PathCommand for CloseCommand {
    fn emit(&self, builder: &mut String) {
        // TODO: Support relative
        builder.push_str("Z");
    }
}

struct BezierCommand {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x: i32,
    y: i32,
}

impl PathCommand for BezierCommand {
    fn emit(&self, builder: &mut String) {
        // TODO: Support relative
        builder.push_str(
            format!(
                "C {:} {:}, {:} {:}, {:} {:}",
                self.x1, self.y1, self.x2, self.y2, self.x, self.y
            )
            .as_str(),
        )
    }
}

impl PathBuilder<'_> {
    pub fn new<'a>(parent: &'a mut SvgBuilder) -> PathBuilder<'a> {
        PathBuilder {
            parent,
            commands: vec![],
            fill: None,
            stroke: None,
        }
    }

    pub fn move_to(&mut self, x: u32, y: u32) -> &mut Self {
        self.commands.push(Box::new(MoveToCommand { x, y }));
        self
    }

    pub fn line_to(&mut self, x: u32, y: u32) -> &mut Self {
        self.commands.push(Box::new(LineToCommand { x, y }));
        self
    }

    pub fn horizontal_line_to(&mut self, x: u32) -> &mut Self {
        self.commands.push(Box::new(HorizontalLineToCommand { x }));
        self
    }

    pub fn vertical_line_to(&mut self, y: u32) -> &mut Self {
        self.commands.push(Box::new(VerticalLineToCommand { y }));
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.commands.push(Box::new(CloseCommand {}));
        self
    }

    // background-color
    // TODO: opacity 指定を分離する
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

    pub fn bezier(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x: i32, y: i32) -> &mut Self {
        self.commands.push(Box::new(BezierCommand {
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        }));
        self
    }
}

impl Drop for PathBuilder<'_> {
    fn drop(&mut self) {
        // String の最大容量を決め打ちで確保しておくことで生成を高速化できそう
        let mut elem = format!("<path ");

        elem += "d=\"";
        for command in self.commands.iter() {
            command.emit(&mut elem);
            elem += " ";
        }
        elem += "\" ";

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
