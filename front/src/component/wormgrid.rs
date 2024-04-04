use gloo::console::log;

const TAIL_MAX_LEN: usize = 5;
const WORM_SPEED: f64 = 370.;
const WORM_ROTATION_TIMER_LOW: f64 = 0.5;
const WORM_ROTATION_TIMER_HIGH: f64 = 3.;

// Debug
const DEBUG_DRAW_VISION_POINTS: bool = false;
const DEBUG_DRAW_HEAD_POINTS: bool = false;

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
            let iworm = self.worms.get(i).unwrap();
            let collide = self.worms.iter().enumerate().any(|(j, jworm)| {
                if i == j {
                    return false;
                }

                jworm.tail.iter().any(|bit| {
                    let prediction_time_s = 0.5;

                    // One eye at each side of it's head looking forward
                    [90.0f64, -90.0f64].iter().any(|angle| {
                        let angle = angle.to_radians();
                        maths::collision::rect_line(
                            maths::Rect::new_from_center(bit.position, iworm.rect.size(), 0.),
                            maths::Line::new_rotated(
                                maths::Point::new_rotated(
                                    iworm.rect.center(),
                                    iworm.rect.center()
                                        + maths::Point::new(iworm.rect.width() / 2., 0.),
                                    angle + iworm.direction.to_vec2().as_angle(),
                                ),
                                WORM_SPEED * prediction_time_s,
                                iworm.direction.to_vec2().as_angle(),
                            ),
                        )
                    })
                })
            });

            let worm = self.worms.get_mut(i).unwrap();

            if collide {
                worm.direction = random::pick(&worm.direction.sides());
                worm.rotation_timer.restart();
                worm.step(dt)
            }

            if worm.rect.center().x < -window_size.x
                || worm.rect.center().x > window_size.x
                || worm.rect.center().y < -window_size.y
                || worm.rect.center().y > window_size.y
            {
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
                    // Maybe should reset the worm pos at 0, 0
                    random::pick(&Direction::all())
                };
                // worm.rotation_timer.restart();
            }
            worm.step(dt);
            worm.update_tail(dt);
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
            if DEBUG_DRAW_HEAD_POINTS {
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
                        false,
                        false,
                    )
                });
            }

            // Draw head

            let head_distance_to_body = worm.rect.width() / 2.;
            let head_radius = 30.;
            let antena_distance = 30.;
            let antena_radius = 20.;

            let triangle_base = maths::Point::new_rotated(
                worm.rect.center(),
                worm.rect.center() + maths::Point::new(head_distance_to_body, 0.),
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

            // Debug vision points
            if DEBUG_DRAW_VISION_POINTS {
                [90.0f64, -90.0f64].iter().for_each(|angle| {
                    let angle = angle.to_radians();
                    let line = maths::Line::new_rotated(
                        maths::Point::new_rotated(
                            worm.rect.center(),
                            worm.rect.center() + maths::Point::new(worm.rect.width() / 2., 0.),
                            angle + worm.direction.to_vec2().as_angle(),
                        ),
                        WORM_SPEED * 0.5,
                        worm.direction.to_vec2().as_angle(),
                    );

                    let size = 10.;
                    crate::render::draw_circle(
                        glctx,
                        circle_shader_prog,
                        maths::Circle::new(line.0, size),
                        crate::render::Color::WHITE,
                        false,
                        false,
                    );
                    crate::render::draw_circle(
                        glctx,
                        circle_shader_prog,
                        maths::Circle::new(line.1, size),
                        crate::render::Color::WHITE,
                        false,
                        false,
                    );
                });
            }
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
            direction: random::pick(&Direction::all()),
            rotation_timer: time::DTDelay::new(random::get_inc(
                WORM_ROTATION_TIMER_LOW,
                WORM_ROTATION_TIMER_HIGH,
            )),
            tail: std::collections::VecDeque::<WormTailBit>::new(),
            tail_spawn_delay: time::DTDelay::new(rect.size().x / WORM_SPEED),
        }
    }
    pub fn step(&mut self, dt: f64) {
        self.rotation_timer.update(dt);
        if self.rotation_timer.ended() {
            self.direction = random::pick(&self.direction.sides());
            self.rotation_timer = time::DTDelay::new(random::get_inc(
                WORM_ROTATION_TIMER_LOW,
                WORM_ROTATION_TIMER_HIGH,
            ))
        }

        let mut pos = self.rect.center();

        pos += self.direction.to_vec2() * maths::Vec2::new(WORM_SPEED * dt, WORM_SPEED * dt);

        self.rect.set_center(pos);
    }

    pub fn update_tail(&mut self, dt: f64) {
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

        if self.tail.is_empty() || self.tail.len() < TAIL_MAX_LEN && self.tail_spawn_delay.ended() {
            self.tail_spawn_delay.restart();
            self.tail.push_front(WormTailBit::new(self.rect.center()));
            return;
        }

        while self.tail.back().unwrap().lifetime.ended() {
            self.tail.pop_back();
        }
    }
}

impl Direction {
    fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }
    fn to_vec2(&self) -> maths::Vec2 {
        use maths::Vec2;
        match self {
            Direction::Up => Vec2::new(0., -1.),
            Direction::Down => Vec2::new(0., 1.),
            Direction::Left => Vec2::new(-1., 0.),
            Direction::Right => Vec2::new(1., 0.),
        }
    }
    // fn fliped(&self) -> Self {
    //     match self {
    //         Direction::Up => Direction::Down,
    //         Direction::Down => Direction::Up,
    //         Direction::Left => Direction::Right,
    //         Direction::Right => Direction::Left,
    //     }
    // }
    // fn others(&self) -> [Self; 3] {
    //     match self {
    //         Direction::Up => [Self::Down, Self::Left, Self::Right],
    //         Direction::Down => [Self::Up, Self::Left, Self::Right],
    //         Direction::Left => [Self::Up, Self::Down, Self::Right],
    //         Direction::Right => [Self::Up, Self::Down, Self::Left],
    //     }
    // }

    fn sides(&self) -> [Self; 2] {
        match self {
            Direction::Up => [Self::Left, Self::Right],
            Direction::Down => [Self::Left, Self::Right],
            Direction::Left => [Self::Up, Self::Down],
            Direction::Right => [Self::Up, Self::Down],
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
