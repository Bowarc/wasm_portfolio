use wasm_bindgen::{closure::Closure, JsCast};

const WORM_SPEED: f64 = 1.;

#[derive(Clone)]
pub struct Worm {
    rect: crate::maths::Rect,
    color: crate::render::Color,
    direction: crate::maths::Vec2,
}

impl Worm {
    pub fn new(rect: crate::maths::Rect, color: crate::render::Color) -> Self {
        Self {
            rect,
            color,
            direction: {
                let mut dir =
                    crate::maths::Vec2::new(random::get_inc(-1., 1.), random::get_inc(-1., 1.));
                dir.normalize();
                dir
            },
        }
    }
    pub fn step(&mut self) {
        let mut pos = self.rect.center();

        pos += self.direction * crate::maths::Vec2::new(WORM_SPEED, WORM_SPEED);

        self.rect.set_center(pos);
    }
}

pub struct WormGrid {
    size: crate::maths::Vec2,
    worms: Vec<Worm>,
}

impl WormGrid {
    pub fn new(canvas_size: crate::maths::Vec2, worm_count: u8) -> Self {
        let mut worms = Vec::new();

        for _ in 0..worm_count {
            let rect = crate::maths::Rect::new(
                crate::maths::Point::new(
                    random::get(-canvas_size.x, canvas_size.x),
                    random::get(-canvas_size.y, canvas_size.y),
                ),
                crate::maths::Point::new(10., 10.),
                0.,
            );

            let color = crate::render::Color::random_rgb();

            worms.push(Worm::new(rect, color))
        }

        Self {
            size: canvas_size,
            worms,
        }
    }

    pub fn update(&mut self, grid_size: crate::maths::Vec2) {
        for worm in self.worms.iter_mut() {
            worm.step();
            if worm.rect.center().x < -grid_size.x {
                worm.rect
                    .set_center(crate::maths::Vec2::new(grid_size.x, worm.rect.center().y));
            }
            if worm.rect.center().x > grid_size.x {
                worm.rect
                    .set_center(crate::maths::Vec2::new(-grid_size.x, worm.rect.center().y));
            }
            if worm.rect.center().y < -grid_size.y {
                worm.rect
                    .set_center(crate::maths::Vec2::new(worm.rect.center().x, grid_size.y));
            }
            if worm.rect.center().y > grid_size.y {
                worm.rect
                    .set_center(crate::maths::Vec2::new(worm.rect.center().x, -grid_size.y));
            }
        }
    }
    pub fn draw(
        &mut self,
        glctx: &web_sys::WebGlRenderingContext,
        rect_shader_prog: &web_sys::WebGlProgram,
    ) {
        for worm in self.worms.iter() {
            let rect_verticies = crate::render::rect_to_vert(worm.rect, self.size);
            crate::render::draw(&glctx, &rect_shader_prog, &rect_verticies, worm.color)
        }
    }
}
