use crate::drawing::svg::SVGElement;

pub struct Text {
    x: f32,
    y: f32,
    value: String,
    rotate: Option<f32>,
    font_size: Option<f32>,
}

impl Text {
    pub fn new(x: f32, y: f32, text: String) -> Text {
        Text {
            x,
            y,
            value: text,
            rotate: None,
            font_size: None,
        }
    }

    pub fn set_rotate(mut self, v: f32) -> Text {
        self.rotate = Some(v);
        self
    }

    pub fn set_font_size(mut self, v: f32) -> Text {
        self.font_size = Some(v);
        self
    }
}

impl SVGElement for Text {
    fn to_svg_string_node(&self) -> String {
        let mut res = format!("<text x=\"{}\" y=\"{}\"", self.x, self.y);
        if let Some(r) = self.rotate {
            res.push_str(format!(" transform=\"rotate({})\"", r).as_str());
        }
        if let Some(r) = self.font_size {
            res.push_str(format!(" font-size=\"{}\"", r).as_str());
        }
        res.push_str(
            format!(
                " text-anchor=\"middle\" dominant-baseline=\"middle\">{}</text>",
                self.value
            )
            .as_str(),
        );
        res
    }
}
