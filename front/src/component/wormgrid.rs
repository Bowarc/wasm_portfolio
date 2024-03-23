use std::path::StripPrefixError;

use gloo::console::log;
use js_sys::Date;
use wasm_bindgen::{closure::Closure, JsCast};

const WORM_SPEED: f64 = 370.;

pub struct  WormGrid {
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
    tail: std::collections::VecDeque<WormTailBit>,
    tail_spawn_delay: time::DTDelay
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct WormTailBit /* You're a wizzard Pettigrew */ {
    position: maths::Point,
    lifetime: time::DTDelay,
}

impl WormGrid {
    pub fn new(canvas_size: maths::Vec2, worm_count: u16) -> Self {
        log!(format!(
            "Initializing wormgrid with {worm_count} worms on a canvas of size: {canvas_size}"
        ));

        let mut worms = Vec::new();

        // For canvas, topleft is [-canvas_size.x, canvas_size.y] and botright is [canvas_size.x, -canvas_size.y]
        for _ in 0..worm_count {
            let rect = maths::Rect::new(
                maths::Point::new(
                    random::get(-canvas_size.x, canvas_size.x),
                    random::get(-canvas_size.y, canvas_size.y),
                ),
                maths::Point::new(40., 40.),
                0.,
            );

            let color = crate::render::Color::random_rgba();

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
            worm.update_queue(dt);
        }
    }
    pub fn draw(
        &mut self,
        glctx: &web_sys::WebGlRenderingContext,
        rect_shader_prog: &web_sys::WebGlProgram,
        circle_shader_prog: &web_sys::WebGlProgram,
    ) {
        for worm in self.worms.iter() {
            // Draw tail
            worm.tail.iter().enumerate().for_each(|(i, tail_bit)| {
                crate::render::draw_rect(
                    glctx,
                    rect_shader_prog,
                    maths::Rect::new_from_center(tail_bit.position, worm.rect.size(), 0.),
                    crate::render::Color::from_rgba(
                        worm.color.r(),
                        worm.color.g(),
                        worm.color.b(),
                        (tail_bit.lifetime.fraction()*255.) as u8,
                    ),
                )
            });

            // Draw corner point
            [
                worm.rect.aa_topleft(),
                worm.rect.aa_topright(),
                worm.rect.aa_botright(),
                worm.rect.aa_botleft(),
            ]
            .iter()
            .for_each(|corner| {
                crate::render::draw_circle(
                    &glctx,
                    &circle_shader_prog,
                    maths::Circle::new(*corner, worm.rect.width() / 4.),
                    crate::render::Color::from_rgba(
                        worm.color.r(),
                        worm.color.g(),
                        worm.color.b(),
                        255,
                    ),
                )
            });

            // Draw head
        }

        crate::render::draw_circle(
            &glctx,
            &circle_shader_prog,
            maths::Circle::new(maths::Vec2::new(0., 0.), 100.),
            crate::render::Color::random_rgba(),
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
            tail: std::collections::VecDeque::<WormTailBit>::new(),
            tail_spawn_delay: time::DTDelay::new(rect.size().x / WORM_SPEED)
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

    pub fn update_queue(&mut self, dt: f64) {
        self.tail_spawn_delay.update(dt);

        self.tail.iter_mut().for_each(|bit| bit.lifetime.update(dt));

        // const TAIL_MAX_LEN: usize = 4;

        if self.tail.is_empty() {
            self.tail.push_front(WormTailBit::new(self.rect.center()));
            return;
        }

        if self.tail_spawn_delay.ended(){
            self.tail_spawn_delay.restart();
            self.tail.push_front(WormTailBit::new(self.rect.center()));
        }

        while self.tail.get(self.tail.len() -1).unwrap().lifetime.ended(){
            self.tail.pop_back();
        }

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

impl WormTailBit {
    pub fn new(position: maths::Point) -> Self {
        Self {
            position,
            lifetime: time::DTDelay::new(0.5),
        }
    }
}
