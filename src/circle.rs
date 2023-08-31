use crate::SvgBuilder;

pub struct CircleBuilder<'a> {
    pub parent: &'a mut SvgBuilder,
    pub cx: u32,
    pub cy: u32,
    pub radius: u32,
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
