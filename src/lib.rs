use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct SvgBuilder {
    content: String,
}

impl SvgBuilder {
    pub fn new() -> SvgBuilder {
        SvgBuilder {
            content: String::new(),
        }
    }

    // TODO: version, baseProfile 対応
    // TODO: width, height 対応
    pub fn begin(&mut self) {
        self.content.push_str(
        r#"<svg version="1.1" baseProfile="full" width="300" height="200" xmlns="http://www.w3.org/2000/svg">
"#);
    }

    pub fn rect<'a>(&'a mut self, width: &'a str, height: &'a str) -> RectBuilder<'a> {
        RectBuilder {
            parent: self,
            width,
            height,
            attributes: vec![],
        }
    }

    pub fn circle<'a>(&'a mut self, cx: u32, cy: u32, radius: u32) -> CircleBuilder<'a> {
        CircleBuilder {
            parent: self,
            cx,
            cy,
            radius,
        }
    }

    pub fn text<'a>(
        &'a mut self,
        x: u32,
        y: u32,
        font_size: u32,
        text: &'a str,
    ) -> TextBuilder<'a> {
        TextBuilder {
            parent: self,
            x,
            y,
            font_size,
            text,
        }
    }

    pub fn end(&mut self) {
        self.content.push_str(r#"</svg>"#);
    }

    pub fn save_as(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        let mut f = File::create(file_name)?;
        f.write_all(self.content.as_bytes())?;
        Ok(())
    }
}

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

pub struct RectBuilder<'a> {
    parent: &'a mut SvgBuilder,
    width: &'a str,
    height: &'a str,
    attributes: Vec<Box<dyn RectAttribute>>,
}

impl RectBuilder<'_> {
    fn add(&mut self, att: Box<dyn RectAttribute>)
    {
        self.attributes.push(att)
    }

    pub fn position(&mut self, x: u32, y: u32)
    {
        self.add(Box::new(Position {x, y}))
    }
}

impl Drop for RectBuilder<'_> {
    fn drop(&mut self) {
        let mut elem = format!(
            "<rect width=\"{:}\" height=\"{:}\" ",
            self.width, self.height
        );

        for att in self.attributes.iter() {
            att.emit(&mut elem);
        }

        elem += " />\n";
        self.parent.content.push_str(&elem);
    }
}

pub struct CircleBuilder<'a> {
    parent: &'a mut SvgBuilder,
    cx: u32,
    cy: u32,
    radius: u32,
}

impl Drop for CircleBuilder<'_> {
    fn drop(&mut self) {
        let elem = format!(
            "<circle cx=\"{:}\" cy=\"{:}\" radius=\"{:}\" />\n",
            self.cx, self.cy, self.radius
        );
        self.parent.content.push_str(&elem);
    }
}

pub struct TextBuilder<'a> {
    parent: &'a mut SvgBuilder,
    x: u32,
    y: u32,
    font_size: u32,
    text: &'a str,
}

impl Drop for TextBuilder<'_> {
    fn drop(&mut self) {
        let mut elem = format!(
            "<text x=\"{:}\" y=\"{:}\" font-size=\"{:}\" text-anchor=\"middle\" fill=\"white\">",
            self.x, self.y, self.font_size
        );
        elem += self.text;
        elem += "</text>\n";

        self.parent.content.push_str(&elem);
    }
}
