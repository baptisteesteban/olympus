use crate::{drawing::svg::SVGElement, Rgb8};

pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    rx: f32,
    ry: f32,
    fill: Option<Rgb8>,
    stroke: Option<Rgb8>,
    stroke_width: Option<f32>,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            x,
            y,
            w,
            h,
            rx: 0f32,
            ry: 0f32,
            fill: None,
            stroke: None,
            stroke_width: None,
        }
    }

    pub fn new_rounded(x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32) -> Rect {
        Rect {
            x,
            y,
            w,
            h,
            rx,
            ry,
            fill: None,
            stroke: None,
            stroke_width: None,
        }
    }

    pub fn set_fill(mut self, c: Rgb8) -> Self {
        self.fill = Some(c);
        self
    }

    pub fn set_stroke(mut self, c: Rgb8) -> Self {
        self.stroke = Some(c);
        self
    }

    pub fn set_stroke_width(mut self, v: f32) -> Self {
        self.stroke_width = Some(v);
        self
    }
}

impl SVGElement for Rect {
    fn to_svg_string_node(&self) -> String {
        let mut res = format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" ry=\"{}\"",
            self.x, self.y, self.w, self.h, self.rx, self.ry
        );
        if let Some(c) = &self.fill {
            res.push_str(format!(" fill=\"#{:02x}{:02x}{:02x}\"", c.r, c.g, c.b).as_str());
        }
        if let Some(c) = &self.stroke {
            res.push_str(format!(" stroke=\"#{:02x}{:02x}{:02x}\"", c.r, c.g, c.b).as_str());
        }
        if let Some(v) = &self.stroke_width {
            res.push_str(format!(" stroke-width=\"{}\"", v).as_str());
        }

        res.push_str("/>");
        res
    }
}
