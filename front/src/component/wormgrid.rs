use gloo::console::log;

mod worm;

use worm::SPEED;

// Debug
const DEBUG_DRAW_VISION_POINTS: bool = false;
const DEBUG_DRAW_HEAD_POINTS: bool = false;

pub struct WormGrid {
    size: maths::Vec2,
    worms: Vec<worm::Worm>,
    last_update: wasm_timer::Instant,
}

impl WormGrid {
    pub fn new(canvas_size: maths::Vec2) -> Self {
        let count = ((canvas_size.x + canvas_size.y /2.) * 0.01) as u16;

        log!(format!(
            "Initializing wormgrid with {count} worms on a canvas of size: {canvas_size}"
        ));

        let mut worms = Vec::new();

        // For canvas, topleft is [-canvas_size.x, canvas_size.y] and botright is [canvas_size.x, -canvas_size.y]
        for _ in 0..count {
            let rect = maths::Rect::new(
                maths::Point::new(
                    random::get(-canvas_size.x, canvas_size.x),
                    random::get(-canvas_size.y, canvas_size.y),
                ),
                maths::Point::new(40., 40.),
                0.,
            );

            let color = crate::render::Color::random_rgba();

            worms.push(worm::Worm::new(rect, color))
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
            //////
            //  Worm to Worm collision
            //////
            let iworm = self.worms.get(i).unwrap();
            if self.worms.iter().enumerate().any(|(j, jworm)| {
                if i == j {
                    return false;
                }

                jworm.tail().iter().any(|bit: &worm::WormTailBit| {
                    let prediction_time_s = 0.5;

                    // One eye at each side of it's head looking forward
                    [90.0f64, -90.0f64].iter().any(|angle| {
                        let angle = angle.to_radians();
                        maths::collision::rect_line(
                            maths::Rect::new_from_center(bit.position(), iworm.size(), 0.),
                            maths::Line::new_rotated(
                                maths::Point::new_rotated(
                                    iworm.position(),
                                    iworm.position() + maths::Point::new(iworm.size().x / 2., 0.),
                                    angle + iworm.direction().to_vec2().as_angle(),
                                ),
                                SPEED * prediction_time_s,
                                iworm.direction().to_vec2().as_angle(),
                            ),
                        )
                    })
                })
            }) {
                let worm = self.worms.get_mut(i).unwrap();

                worm.set_direction(random::pick(&worm.direction().sides()));
                worm.rotation_timer_mut().restart();
                // worm.step(dt)
            }

            /////
            // Grid bounds collision
            /////
            let worm = self.worms.get_mut(i).unwrap();

            if [
                // worm::Direction::Up,
                maths::Line::new(
                    maths::Point::new(-self.size.x, self.size.y),
                    maths::Point::new(self.size.x, self.size.y),
                ),
                // worm::Direction::Down,
                maths::Line::new(
                    maths::Point::new(self.size.x, self.size.y),
                    maths::Point::new(self.size.x, -self.size.y),
                ),
                // worm::Direction::Left,
                maths::Line::new(
                    maths::Point::new(self.size.x, -self.size.y),
                    maths::Point::new(-self.size.x, -self.size.y),
                ),
                // worm::Direction::Right,
                maths::Line::new(
                    maths::Point::new(-self.size.x, -self.size.y),
                    maths::Point::new(-self.size.x, self.size.y),
                ),
            ]
            .into_iter()
            .any(|border| {
                let prediction_time_s = 0.2;

                let los = maths::Line::new_rotated(
                    worm.position(),
                    SPEED * prediction_time_s,
                    worm.direction().to_vec2().as_angle(),
                );

                maths::collision::line_line(border, los)
            }) {
                worm.set_direction(random::pick(&worm.direction().sides()));
                worm.rotation_timer_mut().restart()
            }

            /////
            // OOB Worm
            /////
            if worm.position().x < -window_size.x
                || worm.position().x > window_size.x
                || worm.position().y < -window_size.y
                || worm.position().y > window_size.y
            {
                worm.set_position(maths::Point::ZERO);
                worm.tail_mut()
                    .iter_mut()
                    .for_each(|bit| bit.set_position(maths::Point::ZERO));
                log!(format!(
                    "Worm ({}) has exited the window and has been corrected",
                    worm.id()
                ));
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
            worm.tail().iter().for_each(|tail_bit| {
                crate::render::draw_rect(
                    glctx,
                    rect_shader_prog,
                    maths::Rect::new_from_center(
                        tail_bit.position(),
                        worm.size(),
                        tail_bit.lifetime().fraction(),
                    ),
                    crate::render::Color::from_rgba(
                        worm.color().r(),
                        worm.color().g(),
                        worm.color().b(),
                        (tail_bit.lifetime().fraction() * 255.) as u8,
                    ),
                )
            });

            // Draw corner point
            if DEBUG_DRAW_HEAD_POINTS {
                [
                    worm.rect().aa_topleft(),
                    worm.rect().aa_topright(),
                    worm.rect().aa_botright(),
                    worm.rect().aa_botleft(),
                ]
                .iter()
                .for_each(|corner| {
                    crate::render::draw_circle(
                        &glctx,
                        &circle_shader_prog,
                        maths::Circle::new(*corner, worm.size().x / 4.),
                        crate::render::Color::from_rgba(
                            worm.color().r(),
                            worm.color().g(),
                            worm.color().b(),
                            255,
                        ),
                        false,
                        false,
                    )
                });
            }

            // Draw head

            let head_distance_to_body = worm.size().x / 2.;
            let head_radius = 30.;
            let antena_distance = 30.;
            let antena_radius = 20.;

            let triangle_base = maths::Point::new_rotated(
                worm.position(),
                worm.position() + maths::Point::new(head_distance_to_body, 0.),
                worm.direction().to_vec2().as_angle(),
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
                    *angle + worm.direction().to_vec2().as_angle(),
                );

                let circle = maths::Circle::new(new_pt, antena_radius);
                crate::render::draw_circle(
                    glctx,
                    circle_shader_prog,
                    circle,
                    worm.head_color(),
                    true,
                    false,
                );
            });

            crate::render::draw_circle(
                glctx,
                circle_shader_prog,
                maths::Circle::new(triangle_base, head_radius),
                {
                    let mut c = worm.color();
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
                worm.color(),
                true,
                false,
            );

            // Debug vision points
            if DEBUG_DRAW_VISION_POINTS {
                [90.0f64, -90.0f64].iter().for_each(|angle| {
                    let angle = angle.to_radians();
                    let line = maths::Line::new_rotated(
                        maths::Point::new_rotated(
                            worm.position(),
                            worm.position() + maths::Point::new(worm.size().x / 2., 0.),
                            angle + worm.direction().to_vec2().as_angle(),
                        ),
                        SPEED * 0.5,
                        worm.direction().to_vec2().as_angle(),
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
