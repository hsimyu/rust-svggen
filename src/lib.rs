use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
pub mod attribute;
mod path;
mod rect;

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

    pub fn rect<'a>(&'a mut self, width: &'a str, height: &'a str) -> rect::RectBuilder<'a> {
        rect::RectBuilder::new(self, width, height)
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

    pub fn path<'a>(&'a mut self) -> path::PathBuilder<'a> {
        path::PathBuilder::new(self)
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
