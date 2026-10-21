// TODO: Work on getting drawings
// TODO: Write more tests

pub trait CanvasObject {
    fn draw(&self) {}
}

pub struct Coords {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

pub struct Context<'a> {
    pub origin: (f32, f32),
    pub coords: Coords,
    pub objects: Vec<&'a dyn CanvasObject>,
}

pub struct Canvas<'a> {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub origin: (f32, f32),
    pub coords: Coords,
    pub context: Context<'a>,
}

impl Canvas<'_> {
    pub fn new(w: f32, h: f32) -> Self {
        let coords = Coords {
            x1: 0.0,
            y1: 0.0,
            x2: w,
            y2: h,
        };

        let ctx_coords = Coords {
            x1: -(w / 2.0),
            y1: h / 2.0,
            x2: w / 2.0,
            y2: -(h / 2.0),
        };

        let context = Context {
            origin: (0.0, 0.0),
            coords: ctx_coords,
            objects: vec![],
        };

        Self {
            x: 0.0,
            y: 0.0,
            w,
            h,
            origin: generate_origin(0.0, 0.0, w, h),
            coords,
            context,
        }
    }

    fn get_screen_coords_from_context(&self, xy: (f32, f32)) -> (f32, f32) {
        let x = self.origin.0;
        let y = self.origin.1;

        let ctx_origin = self.context.origin;
        let ctx_x = ctx_origin.0;
        let ctx_y = ctx_origin.1;

        let screen_coords = (x - (ctx_x - xy.0), y - (ctx_y - xy.1)); // Translation: canvas_origin_x - (context_x - passed_in_x)
                                                                      // canvas_origin_y - (context_y - passed_in_y)
        screen_coords
    }

    pub fn update_coords(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.origin = generate_origin(x, y, self.w, self.h);

        let coords = Coords {
            x1: x,
            y1: x,
            x2: self.w,
            y2: self.h,
        };
        self.coords = coords;
    }

    pub fn update_dimensions(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
        self.origin = generate_origin(self.x, self.y, self.w, self.h);

        let coords = Coords {
            x1: self.x,
            y1: self.y,
            x2: w,
            y2: h,
        };
        let ctx_coords = Coords {
            x1: -(w / 2.0),
            y1: h / 2.0,
            x2: w / 2.0,
            y2: -(h / 2.0),
        };
        self.coords = coords;
        self.context.coords = ctx_coords;
    }
}

fn generate_origin(x: f32, y: f32, w: f32, h: f32) -> (f32, f32) {
    let x = x;
    let x = (w / 2.0) + x;

    let y = y;
    let y = (h / 2.0) + y;

    (x, y)
}

#[test]
fn test_get_screen_coords_from_context() {
    let canvas = Canvas::new(1420.0, 720.0);
    let (x, y) = canvas.get_screen_coords_from_context((-1.0, 0.0));

    assert_eq!((710.0, 360.0), (x, y));
}
