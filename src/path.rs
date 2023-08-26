use super::SvgBuilder;

pub struct PathBuilder<'a> {
    parent: &'a mut SvgBuilder,
    commands: Vec<Box<dyn PathCommand>>,
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

impl PathBuilder<'_> {
    pub fn new<'a>(parent: &'a mut SvgBuilder) -> PathBuilder<'a> {
        PathBuilder {
            parent,
            commands: vec![],
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
}

impl Drop for PathBuilder<'_> {
    fn drop(&mut self) {
        // String の最大容量を決め打ちで確保しておくことで生成を高速化できそう
        let mut elem = format!("<path ");

        // 直線コマンド
        elem += "d=\"";
        for command in self.commands.iter() {
            command.emit(&mut elem);
            elem += " ";
        }
        elem += "\"";

        elem += "/>\n";
        self.parent.content.push_str(&elem);
    }
}
