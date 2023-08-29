use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
pub mod attribute;
pub mod path;
mod rect;

pub struct SvgBuilder {
    content: String,

    width: Option<String>,
    height: Option<String>,
}

impl SvgBuilder {
    pub fn new() -> SvgBuilder {
        SvgBuilder {
            content: String::new(),
            width: None,
            height: None,
        }
    }

    // TODO: Self じゃなくて HeaderBuilder を返すとかのがいいかも？
    pub fn begin(&mut self) -> &mut Self {
        self
    }

    pub fn width(&mut self, v: &str) -> &mut Self {
        let mut r = String::new();
        r.push_str(v);
        self.width = Some(r);
        self
    }

    pub fn height(&mut self, v: &str) -> &mut Self {
        let mut r = String::new();
        r.push_str(v);
        self.height = Some(r);
        self
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
        let mut header = String::new();
        header.push_str("<svg ");

        if let Some(v) = self.width.as_ref() {
            header.push_str(format!("width={:} ", v).as_str());
        }

        if let Some(v) = self.height.as_ref() {
            header.push_str(format!("height={:} ", v).as_str());
        }

        // これは SVG 要素がトップの時だけ必要
        header.push_str(format!("xmlns=\"http://www.w3.org/2000/svg\">\n").as_str());

        let content = self.content.clone();
        let footer = "</svg>";

        self.content = header + &content + footer;
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
