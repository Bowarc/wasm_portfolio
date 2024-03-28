#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Color(u8, u8, u8, u8); // 0 - 255

impl Color {
    pub const WHITE: Self = Self(255, 255, 255, 255);
    pub const BLACK: Self = Self(0, 0, 0, 255);
    pub const TRANSPARENT: Self = Self(0, 0, 0, 0);
    pub fn from_rgba(
        r: impl Into<u8>,
        g: impl Into<u8>,
        b: impl Into<u8>,
        a: impl Into<u8>,
    ) -> Self {
        Self(r.into(), g.into(), b.into(), a.into())
    }

    pub fn from_rgb(r: impl Into<u8>, g: impl Into<u8>, b: impl Into<u8>) -> Self {
        Self(r.into(), g.into(), b.into(), 255)
    }
    #[allow(clippy::get_first)]
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.replace('#', "");
        let bits = (0..hex.len())
            .step_by(2)
            .flat_map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
            .collect::<Vec<u8>>();

        if bits.len() == 3 {
            Self::from_rgb(
                *bits.get(0).unwrap(),
                *bits.get(1).unwrap(),
                *bits.get(2).unwrap(),
            )
        } else if bits.len() == 4 {
            Self::from_rgba(
                *bits.get(0).unwrap(),
                *bits.get(1).unwrap(),
                *bits.get(2).unwrap(),
                *bits.get(3).unwrap(),
            )
        } else {
            // warn!("Could not convert hex: #{hex} to rgb, falling back to default color");
            Self::default()
        }
    }

    pub fn random_rgb() -> Self {
        Self(
            random::get_inc(0u8, 255u8),
            random::get_inc(0u8, 255u8),
            random::get_inc(0u8, 255u8),
            255,
        )
    }
    pub fn random_rgba() -> Self {
        Self(
            random::get_inc(0u8, 255u8),
            random::get_inc(0u8, 255u8),
            random::get_inc(0u8, 255u8),
            random::get_inc(0u8, 255u8),
        )
    }

    pub fn rgba(&self) -> [u8; 4] {
        [self.red(), self.green(), self.blue(), self.alpha()]
    }
    pub fn red(&self) -> u8 {
        self.0
    }
    pub fn set_red(&mut self, new_red: u8) {
        self.0 = new_red;
    }
    pub fn green(&self) -> u8 {
        self.1
    }
    pub fn set_green(&mut self, new_green: u8) {
        self.1 = new_green;
    }
    pub fn blue(&self) -> u8 {
        self.2
    }
    pub fn set_blue(&mut self, new_blue: u8) {
        self.2 = new_blue;
    }
    pub fn alpha(&self) -> u8 {
        self.3
    }
    pub fn set_alpha(&mut self, new_alpha: u8) {
        self.3 = new_alpha;
    }
    pub fn r(&self) -> u8 {
        self.red()
    }
    pub fn set_r(&mut self, new_r: u8) {
        self.set_red(new_r)
    }
    pub fn g(&self) -> u8 {
        self.green()
    }
    pub fn set_g(&mut self, new_g: u8) {
        self.set_green(new_g)
    }
    pub fn b(&self) -> u8 {
        self.blue()
    }
    pub fn set_b(&mut self, new_b: u8) {
        self.set_blue(new_b)
    }
    pub fn a(&self) -> u8 {
        self.alpha()
    }
    pub fn set_a(&mut self, new_a: u8) {
        self.set_alpha(new_a)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl From<[u8; 4]> for Color {
    fn from(u8array: [u8; 4]) -> Color {
        Color::from_rgba(u8array[0], u8array[1], u8array[2], u8array[3])
    }
}
