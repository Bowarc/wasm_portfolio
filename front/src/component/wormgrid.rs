use gloo::console::log;
use js_sys::Date;
use wasm_bindgen::{closure::Closure, JsCast};

const WORM_SPEED: f64 = 170.;

pub struct WormGrid {
    size: maths::Vec2,
    worms: Vec<Worm>,
    last_update: wasm_timer::Instant,
}

#[derive(Clone)]
pub struct Worm {
    rect: maths::Rect,
    color: crate::render::Color,
    direction: Direction,
    rotation_timer: time::DTDelay,
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl WormGrid {
    pub fn new(canvas_size: maths::Vec2, worm_count: u16) -> Self {
        log!(format!(
            "Initializing wormgrid with {worm_count} worms on a canvas of size: {canvas_size}"
        ));

        let mut worms = Vec::new();

        for _ in 0..worm_count {
            let rect = maths::Rect::new(
                maths::Point::new(-canvas_size.x, canvas_size.y),
                maths::Point::new(40., 40.),
                0.,
            );

            let color = crate::render::Color::random_rgb();

            worms.push(Worm::new(rect, color))
        }

        Self {
            size: canvas_size,
            worms,
            last_update: wasm_timer::Instant::now(),
        }
    }

    pub fn update(&mut self, grid_size: maths::Vec2) {
        let dt = self.last_update.elapsed().as_secs_f64();
        self.last_update = wasm_timer::Instant::now();
        // log!(dt);

        for worm in self.worms.iter_mut() {
            worm.step(dt);
            if worm.rect.center().x < -grid_size.x {
                worm.rect
                    .set_center(maths::Vec2::new(grid_size.x, worm.rect.center().y));
            }
            if worm.rect.center().x > grid_size.x {
                worm.rect
                    .set_center(maths::Vec2::new(-grid_size.x, worm.rect.center().y));
            }
            if worm.rect.center().y < -grid_size.y {
                worm.rect
                    .set_center(maths::Vec2::new(worm.rect.center().x, grid_size.y));
            }
            if worm.rect.center().y > grid_size.y {
                worm.rect
                    .set_center(maths::Vec2::new(worm.rect.center().x, -grid_size.y));
            }
        }
    }
    pub fn draw(
        &mut self,
        glctx: &web_sys::WebGlRenderingContext,
        rect_shader_prog: &web_sys::WebGlProgram,
        circle_shader_prog: &web_sys::WebGlProgram,
    ) {
        for worm in self.worms.iter() {
            // log!(worm.rect.center().x);
            let verticies = crate::render::circle_to_vert(
                maths::Circle::new(worm.rect.center(), worm.rect.width()),
                self.size,
            );
            crate::render::draw_circle(
                &glctx,
                &circle_shader_prog,
                &verticies,
                worm.rect.center(),
                worm.color,
            )
        }

        let center = maths::Vec2::new(0., 0.);
        let verticies = crate::render::circle_to_vert(
            maths::Circle::new(center, 100.),
            self.size,
        );
        crate::render::draw_circle(
            &glctx,
            &circle_shader_prog,
            &verticies,
            center,
            crate::render::Color::random_rgb(),
        );
    }
}

impl Worm {
    pub fn new(rect: maths::Rect, color: crate::render::Color) -> Self {
        Self {
            rect,
            color,
            direction: random::pick(&[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]),
            rotation_timer: time::DTDelay::new(random::get_inc(0., 1.)),
        }
    }
    pub fn step(&mut self, dt: f64) {
        self.rotation_timer.update(dt);
        if self.rotation_timer.ended() {
            self.direction = random::pick(&[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]);
            self.rotation_timer = time::DTDelay::new(random::get_inc(0.1, 1.))
        }

        let mut pos = self.rect.center();

        pos += self.direction.to_vec2() * maths::Vec2::new(WORM_SPEED * dt, WORM_SPEED * dt);

        self.rect.set_center(pos);
    }
}

impl Direction {
    fn to_vec2(&self) -> maths::Vec2 {
        use maths::Vec2;
        match self {
            Direction::Up => Vec2::new(0., -1.),
            Direction::Down => Vec2::new(0., 1.),
            Direction::Left => Vec2::new(-1., 0.),
            Direction::Right => Vec2::new(1., 0.),
        }
    }
}
