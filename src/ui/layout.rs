use macroquad::{color::*, shapes::draw_rectangle};

pub trait Element {
    fn size(&self) -> Size;
    fn draw(&self, x: f32, y: f32);
}

pub enum Alignment {
    Left,   /* Also means the same as alignment up */
    Right,  /* Also means the same alignment down */
    Center, /* Means exactly what the word says */
}

pub struct Size {
    w: f32,
    h: f32,
}

pub struct Rectangle {
    w: f32,
    h: f32,
    color: Color,
}

pub struct Row<'a> {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    gap: f32,
    alignment: Alignment,
    children: Vec<&'a dyn Element>,
}

pub struct Column<'a> {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    gap: f32,
    alignment: Alignment,
    children: Vec<&'a dyn Element>,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}

impl Element for Rectangle {
    fn size(&self) -> Size {
        Size::new(self.w, self.h)
    }
    fn draw(&self, x: f32, y: f32) {
        draw_rectangle(x, y, self.w, self.h, self.color);
    }
}

impl Element for Row<'_> {
    fn size(&self) -> Size {
        Size::new(self.w, self.h)
    }

    fn draw(&self, x: f32, y: f32) {
        let mut x = x; // We only move the x axis forward for now
        let y = y;

        for child in &self.children {
            let y = y + self.get_align_offset(*child);
            child.draw(x, y);
            x += child.size().w + self.gap;
        }
    }
}

impl Element for Column<'_> {
    fn size(&self) -> Size {
        Size::new(self.w, self.h)
    }

    fn draw(&self, x: f32, y: f32) {
        let x = x; // We only move the x axis forward for now
        let mut y = y;

        for child in &self.children {
            child.draw(x, y);
            y += child.size().h + self.gap;
        }
    }
}

impl Rectangle {
    pub fn new(w: f32, h: f32, color: Option<Color>) -> Self {
        if let Some(color) = color {
            Self { w, h, color }
        } else {
            Self { w, h, color: BLACK }
        }
    }
}

impl<'a> Row<'a> {
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

    pub fn child(&mut self, element: &'a dyn Element) -> &mut Self {
        self.children.push(element); /* add child then calc new dimensions */

        let (width, height) = self.get_final_row_dimensions();
        self.w = width;
        self.h = height;

        self
    }

    fn get_final_row_dimensions(&self) -> (f32, f32) {
        let len = self.children.len();
        let mut width = (len - 1) as f32 * self.gap;
        let mut height = 0.0;
        let mut index = 0;

        while index < len {
            let child = &self.children[index];
            let child_size = child.size();

            width += child_size.w + self.gap;
            if child_size.h > height {
                height = child_size.h;
            }

            index += 1;
        }

        (width, height)
    }

    pub fn align(&mut self, alignment: Alignment) -> &mut Self {
        self.alignment = alignment;
        self
    }

    fn get_align_offset(&self, element: &dyn Element) -> f32 {
        // Gives it an y offset because that's all we need
        let offset: f32;
        match self.alignment {
            Alignment::Left => offset = self.x,
            Alignment::Center => {
                let el_size = element.size();
                offset = self.h / 2.0 - el_size.h / 2.0;
            }
            Alignment::Right => offset = self.h - element.size().h,
        }
        offset
    }

    pub fn gap(&mut self, gap: f32) -> &mut Self {
        self.gap = gap;
        self
    }
}

impl<'a> Column<'a> {
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

    pub fn align(&mut self, alignment: Alignment) -> &mut Self {
        self.alignment = alignment;
        self
    }

    pub fn gap(&mut self, gap: f32) -> &mut Self {
        self.gap = gap;
        self
    }

    fn get_final_column_dimensions(&self) -> (f32, f32) {
        let len = self.children.len();
        let mut width = 0.0;
        let mut height = (len - 1) as f32 * self.gap;
        let mut index = 0;

        while index < len {
            let child = &self.children[index];
            let child_size = child.size();

            height += child_size.h;
            if child_size.w > width {
                width = child_size.w;
            }

            index += 1;
        }

        (width, height)
    }

    pub fn child(&mut self, element: &'a dyn Element) -> &mut Self {
        self.children.push(element); /* add child then calc new dimensions */

        let (width, height) = self.get_final_column_dimensions();
        self.w = width;
        self.h = height;

        self
    }

    fn get_align_offset(&self, element: &'a dyn Element) -> f32 {
        // Gives it an y offset because that's all we need
        let offset: f32;

        match self.alignment {
            Alignment::Left => offset = self.x,
            Alignment::Center => {
                let size = element.size();
                offset = self.w / 2.0 - size.w / 2.0;
            }
            Alignment::Right => offset = self.w - element.size().w,
        }

        offset
    }
}
