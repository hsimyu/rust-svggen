use crate::SvgBuilder;

pub struct TextBuilder<'a> {
    pub parent: &'a mut SvgBuilder,
    pub x: u32,
    pub y: u32,
    pub font_size: u32,
    pub text: &'a str,
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
