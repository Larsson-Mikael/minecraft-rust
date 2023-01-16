pub mod chunk {
    pub const LENGTH: i64 = 64;
    pub const WIDTH: i64 = 64;
    pub const HEIGHT: i64 = 128;
} 

pub mod noise {
    pub const GAP: f64 = 22.;
    pub const AMP: f64 = 16.;
    pub const SEED: u32 = 123;
}

pub mod world {
    pub const GRAVITY: f32 = 1.0;
    pub const LENGTH: i32 = 4;
    pub const WIDTH: i32 = 4;
}
