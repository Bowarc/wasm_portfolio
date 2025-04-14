use gloo::console::log;

mod worm;

use worm::SPEED;

// Debug
pub static mut WORM_DEBUG_DRAW_VISION_POINTS: bool = false;
const DEBUG_DRAW_HEAD_POINTS: bool = false;

pub struct WormGrid {
    size: math::Vec2,
    worms: Vec<worm::Worm>,
    last_update: wasm_timer::Instant,
}

impl WormGrid {
    pub fn new(canvas_size: math::Vec2) -> Self {
        let count = ((canvas_size.x + canvas_size.y / 2.) * 0.01) as u16;

        log!(format!(
            "Initializing wormgrid with {count} worms on a canvas of size: {canvas_size}"
        ));

        let mut worms = Vec::new();

        // For canvas, topleft is [-canvas_size.x, canvas_size.y] and botright is [canvas_size.x, -canvas_size.y]
        for _ in 0..count {
            let rect = math::Rect::new(
                math::Point::new(
                    random::get(-canvas_size.x, canvas_size.x),
                    random::get(-canvas_size.y, canvas_size.y),
                ),
                math::Point::new(40., 40.),
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

    pub fn update(&mut self, window_size: math::Vec2) {
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
                        math::collision::rect_line(
                            &math::Rect::new_from_center(bit.position(), iworm.size(), 0.),
                            &math::Line::new_rotated(
                                math::Point::new_rotated(
                                    iworm.position(),
                                    iworm.position() + math::Point::new(iworm.size().x / 2., 0.),
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

                worm.set_direction(random::pick(&worm.direction().sides()).clone());
                worm.rotation_timer_mut().restart();
                // worm.step(dt)
            }

            /////
            // Grid bounds collision
            /////
            let worm = self.worms.get_mut(i).unwrap();

            if [
                // worm::Direction::Up,
                math::Line::new(
                    math::Point::new(-self.size.x, self.size.y),
                    math::Point::new(self.size.x, self.size.y),
                ),
                // worm::Direction::Down,
                math::Line::new(
                    math::Point::new(self.size.x, self.size.y),
                    math::Point::new(self.size.x, -self.size.y),
                ),
                // worm::Direction::Left,
                math::Line::new(
                    math::Point::new(self.size.x, -self.size.y),
                    math::Point::new(-self.size.x, -self.size.y),
                ),
                // worm::Direction::Right,
                math::Line::new(
                    math::Point::new(-self.size.x, -self.size.y),
                    math::Point::new(-self.size.x, self.size.y),
                ),
            ]
            .into_iter()
            .any(|border| {
                let prediction_time_s = 0.2;

                let los = math::Line::new_rotated(
                    worm.position(),
                    SPEED * prediction_time_s,
                    worm.direction().to_vec2().as_angle(),
                );

                math::collision::line_line(&border, &los)
            }) {
                worm.set_direction(random::pick(&worm.direction().sides()).clone());
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
                worm.set_position(math::Point::ZERO);
                worm.tail_mut()
                    .iter_mut()
                    .for_each(|bit| bit.set_position(math::Point::ZERO));
                // log!(format!(
                //     "Worm ({}) has exited the window and has been corrected",
                //     worm.id()
                // ));
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
                    math::Rect::new_from_center(
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
                        glctx,
                        circle_shader_prog,
                        math::Circle::new(*corner, worm.size().x / 4.),
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

            let triangle_base = math::Point::new_rotated(
                worm.position(),
                worm.position() + math::Point::new(head_distance_to_body, 0.),
                worm.direction().to_vec2().as_angle(),
            );

            [
                -90.0f64.to_radians(),
                0.0f64.to_radians(),
                90.0f64.to_radians(),
            ]
            .iter()
            .for_each(|angle| {
                let new_pt = math::Point::new_rotated(
                    triangle_base,
                    triangle_base + math::Point::new(antena_distance, 0.),
                    *angle + worm.direction().to_vec2().as_angle(),
                );

                let circle = math::Circle::new(new_pt, antena_radius);
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
                math::Circle::new(triangle_base, head_radius),
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
                math::Circle::new(triangle_base, head_radius * 0.5),
                worm.color(),
                true,
                false,
            );

            // Debug vision points
            if unsafe { WORM_DEBUG_DRAW_VISION_POINTS } {
                [90.0f64, -90.0f64].iter().for_each(|angle| {
                    let angle = angle.to_radians();
                    // let line = math::Line::new_rotated(
                    //     math::Point::new_rotated(
                    //         worm.position(),
                    //         worm.position() + math::Point::new(worm.size().x / 2., 0.),
                    //         angle + worm.direction().to_vec2().as_angle(),
                    //     ),
                    //     SPEED * 0.5,
                    //     worm.direction().to_vec2().as_angle(),
                    // );

                    let size = 50.;
                    // crate::render::draw_circle(
                    //     glctx,
                    //     circle_shader_prog,
                    //     math::Circle::new(line.0, size),
                    //     crate::render::Color::WHITE,
                    //     false,
                    //     false,
                    // );
                    // crate::render::draw_circle(
                    //     glctx,
                    //     circle_shader_prog,
                    //     math::Circle::new(line.1, size),
                    //     crate::render::Color::WHITE,
                    //     false,
                    //     false,
                    // );

                    let head_side = math::Point::new_rotated(
                        worm.position(),
                        worm.position() + math::Point::new(worm.size().x / 2., 0.),
                        angle + worm.direction().to_vec2().as_angle(),
                    );

                    crate::render::draw_circle(
                        glctx,
                        circle_shader_prog,
                        math::Circle::new(
                            math::Point::new_rotated(
                                head_side,
                                head_side + SPEED * 0.5,
                                worm.direction().to_vec2().as_angle() - 45f64.to_radians(),
                            ),
                            size,
                        ),
                        crate::render::Color::from(worm.color().rgb()),
                        true,
                        false,
                    );
                });

                // crate::render::draw_circle(
                //     glctx,
                //     circle_shader_prog,
                //     math::Circle::new(
                //         math::Point::new_rotated(
                //             worm.position(),
                //             worm.position() + SPEED * 0.5,
                //             worm.direction().to_vec2().as_angle() - 45f64.to_radians(),
                //         ),
                //         50.,
                //     ),
                //     crate::render::Color::random_rgba(),
                //     true,
                //     false,
                // );
            }
        }

        // crate::render::draw_circle(
        //     &glctx,
        //     &circle_shader_prog,
        //     math::Circle::new(math::Vec2::new(0., 0.), 100.),
        //     crate::render::Color::random_rgba(),
        // );
    }
    pub fn size(&self) -> math::Vec2 {
        self.size
    }
}

pub fn start_wormgrid(glctx: web_sys::WebGlRenderingContext) {
    // This should log only once -- not once per frame
    use {
        crate::{component::WormGrid, render},
        gloo::console::log,
        wasm_bindgen::JsCast as _,
        web_sys::{window, HtmlCanvasElement, WebGlRenderingContext},
    };

    crate::render::init(&glctx);

    let canvas_size = math::Point::new(
        glctx.drawing_buffer_width() as f64,
        glctx.drawing_buffer_height() as f64,
    );

    log!(format!("Canvas size: {canvas_size}"));

    // Gloo-render's request_animation_frame has this extra closure
    // wrapping logic running every frame, unnecessary cost.
    // Here constructing the wrapped closure just once.

    let update_fn = std::rc::Rc::new(std::cell::RefCell::new(None));

    *update_fn.borrow_mut() = Some(wasm_bindgen::closure::Closure::wrap(Box::new({
        let rect_shader_program = render::setup_shader(&glctx, "rect");
        let circle_shader_program = render::setup_shader(&glctx, "circle");
        let glctx = glctx.clone();
        let update_fn = update_fn.clone();
        let mut wormgrid = WormGrid::new(canvas_size);
        move || {
            glctx.clear(
                WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
            );
            let window_size = math::Point::new(
                window().unwrap().inner_width().unwrap().as_f64().unwrap(),
                window().unwrap().inner_height().unwrap().as_f64().unwrap(),
            );

            if window_size != wormgrid.size() {
                let canvas = glctx
                    .canvas()
                    .unwrap()
                    .dyn_into::<HtmlCanvasElement>()
                    .unwrap();
                canvas.set_width(window_size.x as u32);
                canvas.set_height(window_size.y as u32);
            }

            // render::draw(
            //     &glctx,
            //     &rect_shader_program,
            //     &render::rect_to_vert(math::Rect::new((0., 0.), canvas_size, 0.), canvas_size),
            //     color,
            // );
            wormgrid.update(window_size);
            wormgrid.draw(&glctx, &rect_shader_program, &circle_shader_program);

            crate::render::end_frame(update_fn.borrow().as_ref().unwrap())
        }
    })
        as Box<dyn FnMut()>));

    crate::render::end_frame(update_fn.borrow().as_ref().unwrap());
}
