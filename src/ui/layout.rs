use macroquad::{color::*, shapes::draw_rectangle};

#[derive(Debug, Clone)]
pub enum Element {
    Rect(Rectangle),
    Row(Row),
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
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

#[derive(Debug, Clone)]
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

    fn get_final_row_dimensions(&self) -> (f32, f32) {
        let mut width = 0.0;
        let mut height = 0.0;
        let mut index = 0;
        let len = self.children.len();

        while index < len {
            let child = &self.children[index];
            let child_dimensions = calculate_dimensions(child);

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

    fn get_align_offset(&self, element: &Element) -> f32 {
        // Gives it an y offset because that's all we need
        match element {
            Element::Rect(rect) => {
                let offset: f32;
                match self.alignment {
                    Alignment::Left => offset = self.y,
                    Alignment::Center => {
                        let (_, row_height) = self.get_final_row_dimensions();
                        let half_of_el = rect.h / 2.0;
                        let half_of_row_height = row_height / 2.0;

                        offset = half_of_row_height - half_of_el;
                    }
                    Alignment::Right => {
                        let (_, row_height) = self.get_final_row_dimensions();
                        offset = row_height - rect.h;
                    }
                }
                return offset;
            }

            Element::Row(row) => {
                let offset: f32;

                match self.alignment {
                    Alignment::Left => offset = self.y,
                    Alignment::Center => {
                        let (_, self_height) = self.get_final_row_dimensions();
                        let half_of_el = row.get_final_row_dimensions().1 / 2.0;
                        let half_of_row_height = self_height / 2.0;

                        offset = half_of_row_height + half_of_el;
                    }
                    Alignment::Right => {
                        let (_, row_height) = self.get_final_row_dimensions();
                        offset = row_height - row.get_final_row_dimensions().1;
                    }
                }
                return offset;
            }
        }
    }

    pub fn gap(&mut self, gap: f32) -> &mut Self {
        self.gap = gap;
        self
    }

    pub fn draw(&self, x: f32, y: f32) -> (f32, f32) {
        let mut x = x; // We only move the x axis forward for now
        let y = y;

        for child in &self.children {
            match child {
                Element::Rect(rect) => {
                    let y = y + self.get_align_offset(&Element::Rect(rect.clone()));
                    rect.draw(x, y);

                    let w = rect.w;
                    x += w + self.gap;
                }
                Element::Row(row) => {
                    let y = y + self.get_align_offset(&Element::Row(row.clone()));
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

fn calculate_dimensions(element: &Element) -> (f32, f32) {
    match element {
        Element::Rect(r) => (r.w, r.h),
        Element::Row(row) => (row.w, row.h),
    }
}
