use macroquad::math::Rect;

enum Element {
    Rect(Rectangle),
    Row(Row),
}

enum Alignment {
    Left,   /* Also means the same as alignment up */
    Right,  /* Also means the same alignment down */
    Center, /* Means exactly what the word says */
}

#[derive(Debug)]
struct Rectangle {
    w: f32,
    h: f32,
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

    fn get_final_row_dimensions(&mut self) -> (f32, f32) {
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

        self.w = width;
        self.h = height;
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

    pub fn draw(&self) -> (f32, f32) {
        let mut x = self.x;
        let mut y = self.y;

        for child in &self.children {
            match child {
                Element::Rect(rect) => {
                    /* Call rect drawing function, and also properly calculate the x */
                    println!("{rect:?}");
                }

                Element::Row(row) => {
                    /* Call row drawing function, and also properly calculate the x */
                    println!("{row:?}");
                }
            }
        }

        (self.w, self.h)
    }
}
