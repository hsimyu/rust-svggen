use super::attribute;
use super::SvgBuilder;

pub struct PathBuilder<'a> {
    parent: &'a mut SvgBuilder,
}

impl PathBuilder<'_> {
    pub fn new<'a>(parent: &'a mut SvgBuilder) -> PathBuilder<'a> {
        PathBuilder { parent }
    }
}

impl Drop for PathBuilder<'_> {
    fn drop(&mut self) {
        // String の最大容量を決め打ちで確保しておくことで生成を高速化できそう
        let mut elem = format!("<path ");

        elem += "/>\n";
        self.parent.content.push_str(&elem);
    }
}
