use spritesheet.SpriteSheet;

pub struct SpriteSet{
    width: u32,
    height: u32, 
    name: String,
    currentframe: u32,
    sprits: Vec[Sprite],
    spritesheet: SpriteSheet
}
impl SpriteSet {
    pub fn getParrent(&self) -> SpriteSheet;
    pub fn getCurrentSprite(&Self) -> Sprite;
}
pub struct Sprite{
    x: u32,
    y: u32,
    framerate: u32,
    spriteset: SpriteSet
}

