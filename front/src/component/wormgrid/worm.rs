pub const TAIL_MAX_LEN: usize = 5;

pub const SPEED: f64 = 370.;
pub const ROTATION_TIMER_LOW: f64 = 0.5;
pub const ROTATION_TIMER_HIGH: f64 = 3.;

static mut CurrentId: u16 = 0;

#[derive(Clone)]
pub struct Worm {
    id: u16,
    rect: maths::Rect,
    color: crate::render::Color,
    head_color: crate::render::Color,
    direction: Direction,
    rotation_timer: time::DTDelay,
    tail: std::collections::VecDeque<WormTailBit>,
    tail_spawn_delay: time::DTDelay,
}

#[derive(Clone)]
pub struct WormTailBit /* You're a wizzard Pettigrew */ {
    position: maths::Point,
    lifetime: time::DTDelay,
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Worm {
    pub fn new(rect: maths::Rect, color: crate::render::Color) -> Self {
        Self {
            id: unsafe {
                CurrentId += 1;
                CurrentId - 1
            },
            rect,
            color,
            head_color: crate::render::Color::random_rgb(),
            direction: random::pick(&Direction::all()),
            rotation_timer: time::DTDelay::new(random::get_inc(
                ROTATION_TIMER_LOW,
                ROTATION_TIMER_HIGH,
            )),
            tail: std::collections::VecDeque::<WormTailBit>::new(),
            tail_spawn_delay: time::DTDelay::new(rect.size().x / SPEED),
        }
    }
    
    pub fn id(&self) -> u16{
        self.id
    }

    pub fn step(&mut self, dt: f64) {
        self.rotation_timer.update(dt);
        if self.rotation_timer.ended() {
            self.direction = random::pick(&self.direction.sides());
            self.rotation_timer =
                time::DTDelay::new(random::get_inc(ROTATION_TIMER_LOW, ROTATION_TIMER_HIGH))
        }

        let mut pos = self.rect.center();

        pos += self.direction.to_vec2() * maths::Vec2::new(SPEED * dt, SPEED * dt);

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

            if maths::get_distance(bit.position, previous_pos) < self.rect.size().x - SPEED * dt {
                return;
            }

            let direction = maths::point::two_points_angle(bit.position, previous_pos);

            bit.position +=
                maths::Point::from_angle(direction) * maths::Vec2::new(SPEED * dt, SPEED * dt);

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

    #[inline]
    pub fn rect(&self) -> &maths::Rect {
        &self.rect
    }

    #[inline]
    pub fn size(&self) -> maths::Vec2 {
        self.rect.size()
    }

    #[inline]
    pub fn color(&self) -> crate::render::Color {
        self.color
    }

    #[inline]
    pub fn head_color(&self) -> crate::render::Color {
        self.head_color
    }

    #[inline]
    pub fn position(&self) -> maths::Point {
        self.rect.center()
    }

    #[inline]
    pub fn direction(&self) -> Direction {
        self.direction.clone()
    }

    #[inline]
    pub fn rotation_timer(&self) -> &time::DTDelay {
        &self.rotation_timer
    }

    #[inline]
    pub fn rotation_timer_mut(&mut self) -> &mut time::DTDelay {
        &mut self.rotation_timer
    }

    #[inline]
    pub fn tail(&self) -> &std::collections::VecDeque<WormTailBit> {
        &self.tail
    }

    #[inline]
    pub fn tail_mut(&mut self) -> &mut std::collections::VecDeque<WormTailBit> {
        &mut self.tail
    }

    #[inline]
    pub fn set_position(&mut self, new_center: impl Into<maths::Point>) {
        self.rect.set_center(new_center.into());
    }

    #[inline]
    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction
    }
}

impl WormTailBit {
    pub fn new(position: maths::Point) -> Self {
        Self {
            position,
            lifetime: time::DTDelay::new(0.5),
        }
    }

    pub fn position(&self) -> maths::Point {
        self.position
    }

    pub fn lifetime(&self) -> &time::DTDelay {
        &self.lifetime
    }

    pub fn set_position(&mut self, new_position: impl Into<maths::Point>) {
        self.position = new_position.into()
    }
}

impl Direction {
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn to_vec2(&self) -> maths::Vec2 {
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

    pub fn sides(&self) -> [Self; 2] {
        match self {
            Direction::Up => [Self::Left, Self::Right],
            Direction::Down => [Self::Left, Self::Right],
            Direction::Left => [Self::Up, Self::Down],
            Direction::Right => [Self::Up, Self::Down],
        }
    }
}
