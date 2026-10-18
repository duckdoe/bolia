use macroquad::{color::*, shapes::draw_rectangle};

#[derive(Debug)]
pub enum Element {
    Rect(Rectangle),
    Row(Row),
}

#[derive(Debug)]
enum Alignment {
    Left,   /* Also means the same as alignment up */
    Right,  /* Also means the same alignment down */
    Center, /* Means exactly what the word says */
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    w: f32,
    h: f32,
    color: Color,
}

#[derive(Debug)]
pub struct Row {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    gap: f32,
    alignment: Alignment,
    children: Vec<Element>,
}

impl Rectangle {
    pub fn new(w: f32, h: f32, color: Option<Color>) -> Self {
        if let Some(color) = color {
            Self { w, h, color }
        } else {
            Self { w, h, color: BLACK }
        }
    }

    pub fn draw(&self, x: f32, y: f32) {
        draw_rectangle(x, y, self.w, self.h, self.color);
    }
}

impl Row {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            gap: 0.0,
            alignment: Alignment::Left,
            children: vec![],
        }
    }

    pub fn child(&mut self, element: Element) -> &mut Self {
        self.children.push(element); /* add child then calc new dimensions */

        let (width, height) = self.get_final_row_dimensions();
        self.w = width;
        self.h = height;

        self
    }

    fn calculate_dimensions(&self, element: &Element) -> (f32, f32) {
        match element {
            Element::Rect(r) => (r.w, r.h),
            Element::Row(_row) => (self.w, self.h),
        }
    }

    fn get_final_row_dimensions(&self) -> (f32, f32) {
        let mut width = 0.0;
        let mut height = 0.0;
        let mut index = 0;
        let len = self.children.len();

        while index < len {
            let child = &self.children[index];
            let child_dimensions = self.calculate_dimensions(child);

            width += child_dimensions.0 + self.gap;
            if child_dimensions.1 > height {
                height = child_dimensions.1;
            }

            index += 1;
        }

        (width, height)
    }

    pub fn align(&mut self, alignment: Alignment) -> &mut Self {
        self.alignment = alignment;
        self
    }

    pub fn gap(&mut self, gap: f32) -> &mut Self {
        self.gap = gap;
        self
    }

    pub fn draw(&self, x: f32, y: f32) -> (f32, f32) {
        let mut x = x; // We only move the x axis forward for now
        let _y = y;

        for child in &self.children {
            match child {
                Element::Rect(rect) => {
                    let w = rect.w;
                    rect.draw(x, y);

                    x += w + self.gap;
                }
                Element::Row(row) => {
                    row.draw(x, y);
                    let (w, _) = row.get_final_row_dimensions();
                    x += w + self.gap;
                }
            }

            println!("next element x coords = {x}");
        }

        (self.w, self.h)
    }
}
