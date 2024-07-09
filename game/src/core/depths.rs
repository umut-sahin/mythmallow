#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Depth {
    Bottom = 0,

    Map,
    ExperiencePoint,
    Enemy,
    Player,
    Item,
    Projectile,

    Top,
}

impl Depth {
    pub fn z(self) -> f32 {
        (self as u8) as f32
    }
}
