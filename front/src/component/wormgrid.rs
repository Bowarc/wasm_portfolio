use futures::io::Read;
use gloo::console::log;

const WORM_SPEED: f64 = 370.;

pub struct WormGrid {
    size: maths::Vec2,
    worms: Vec<Worm>,
    last_update: wasm_timer::Instant,
}

#[derive(Clone)]
pub struct Worm {
    rect: maths::Rect,
    color: crate::render::Color,
    head_color: crate::render::Color,
    direction: Direction,
    rotation_timer: time::DTDelay,
    tail: std::collections::VecDeque<WormTailBit>,
    tail_spawn_delay: time::DTDelay,
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

    pub fn update(&mut self, window_size: maths::Vec2) {
        self.size = window_size;
        let dt = self.last_update.elapsed().as_secs_f64();
        self.last_update = wasm_timer::Instant::now();
        // log!(dt);

        for i in 0..self.worms.len() {
            let mut collide = false;
            for j in 0..self.worms.len() {
                if i == j {
                    continue;
                }
                let iworm = self.worms.get(i).unwrap();
                let jworm = self.worms.get(j).unwrap();

                let c = jworm.tail.iter().any(|bit| {
                    maths::collision::rect_rect(
                        iworm.rect,
                        maths::Rect::new_from_center(bit.position, iworm.rect.size(), 0.),
                    )
                });
                if c {
                    collide = true;
                    break;
                }
            }

            let worm = self.worms.get_mut(i).unwrap();
            if collide {
                worm.direction = random::pick(&[
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]);
            }

            worm.step(dt);

            let out_bounds = worm.rect.center().x < -window_size.x
                || worm.rect.center().x > window_size.x
                || worm.rect.center().y < -window_size.y
                || worm.rect.center().y > window_size.y;
            if out_bounds {
                worm.direction = if worm.rect.center().x < -window_size.x {
                    Direction::Right
                } else if worm.rect.center().x > window_size.x {
                    Direction::Left
                } else if worm.rect.center().y < -window_size.y {
                    Direction::Down
                } else if worm.rect.center().y > window_size.y {
                    Direction::Up
                } else {
                    log!(format!("[ERROR] Could not infer new direction for out of bounds worm with dir: {:?}", worm.direction));
                    random::pick(&[
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ])
                };

                worm.rect.set_center(worm.rect.center() +worm.direction.to_vec2() * window_size.x * 0.01);


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
            worm.tail.iter().for_each(|tail_bit| {
                crate::render::draw_rect(
                    glctx,
                    rect_shader_prog,
                    maths::Rect::new_from_center(
                        tail_bit.position,
                        worm.rect.size(),
                        tail_bit.lifetime.fraction(),
                    ),
                    crate::render::Color::from_rgba(
                        worm.color.r(),
                        worm.color.g(),
                        worm.color.b(),
                        (tail_bit.lifetime.fraction() * 255.) as u8,
                    ),
                )
            });

            // // Draw corner point
            // [
            //     worm.rect.aa_topleft(),
            //     worm.rect.aa_topright(),
            //     worm.rect.aa_botright(),
            //     worm.rect.aa_botleft(),
            // ]
            // .iter()
            // .for_each(|corner| {
            //     crate::render::draw_circle(
            //         &glctx,
            //         &circle_shader_prog,
            //         maths::Circle::new(*corner, worm.rect.width() / 4.),
            //         crate::render::Color::from_rgba(
            //             worm.color.r(),
            //             worm.color.g(),
            //             worm.color.b(),
            //             255,
            //         ),
            //     )
            // });

            // Draw head

            let head_radius = 30.;
            let antena_distance = 30.;
            let antena_radius = 20.;

            let triangle_base = maths::Point::new_rotated(
                worm.rect.center(),
                worm.rect.center() + maths::Point::new(worm.rect.width(), 0.),
                worm.direction.to_vec2().as_angle(),
            );

            [
                -90.0f64.to_radians(),
                0.0f64.to_radians(),
                90.0f64.to_radians(),
            ]
            .iter()
            .for_each(|angle| {
                let new_pt = maths::Point::new_rotated(
                    triangle_base,
                    triangle_base + maths::Point::new(antena_distance, 0.),
                    *angle + worm.direction.to_vec2().as_angle(),
                );

                let circle = maths::Circle::new(new_pt, antena_radius);
                crate::render::draw_circle(
                    glctx,
                    circle_shader_prog,
                    circle,
                    worm.head_color,
                    true,
                    false,
                );
            });

            crate::render::draw_circle(
                glctx,
                circle_shader_prog,
                maths::Circle::new(triangle_base, head_radius),
                {
                    let mut c = worm.color;
                    c.set_alpha(25);
                    c
                },
                false,
                true,
            );
            crate::render::draw_circle(
                glctx,
                circle_shader_prog,
                maths::Circle::new(triangle_base, head_radius * 0.5),
                worm.color,
                true,
                false,
            );
        }

        // crate::render::draw_circle(
        //     &glctx,
        //     &circle_shader_prog,
        //     maths::Circle::new(maths::Vec2::new(0., 0.), 100.),
        //     crate::render::Color::random_rgba(),
        // );
    }
    pub fn size(&self) -> maths::Vec2 {
        self.size
    }
}

impl Worm {
    pub fn new(rect: maths::Rect, color: crate::render::Color) -> Self {
        Self {
            rect,
            color,
            head_color: crate::render::Color::random_rgb(),
            direction: random::pick(&[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]),
            rotation_timer: time::DTDelay::new(random::get_inc(0., 1.)),
            tail: std::collections::VecDeque::<WormTailBit>::new(),
            tail_spawn_delay: time::DTDelay::new(rect.size().x / WORM_SPEED),
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

        let positions = self
            .tail
            .iter()
            .map(|tailbit| tailbit.position)
            .collect::<Vec<maths::Point>>();
        self.tail.iter_mut().enumerate().for_each(|(i, bit)| {
            let previous_pos = if i == 0 {
                self.rect.center()
            } else {
                *positions.get(i - 1).unwrap()
            };

            if maths::get_distance(bit.position, previous_pos)
                < self.rect.size().x - WORM_SPEED * dt
            {
                return;
            }

            let direction = maths::point::two_points_angle(bit.position, previous_pos);

            bit.position += maths::Point::from_angle(direction)
                * maths::Vec2::new(WORM_SPEED * dt, WORM_SPEED * dt);

            // bit.lifetime.update(dt)
        });

        const TAIL_MAX_LEN: usize = 4;

        if self.tail.is_empty() || self.tail.len() < TAIL_MAX_LEN && self.tail_spawn_delay.ended() {
            self.tail_spawn_delay.restart();
            self.tail.push_front(WormTailBit::new(
                if self.tail.len() == 0 {
                    self.rect.center()
                } else {
                    *positions.get(self.tail.len() - 1).unwrap()
                } + maths::Vec2::new(0., self.rect.width()),
            ));
            return;
        }

        while self.tail.back().unwrap().lifetime.ended() {
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
    fn fliped(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
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
