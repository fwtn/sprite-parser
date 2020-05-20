pub struct SpriteSheet{
    path: String,
    height: u32,
    width: u32,
    isLoaded: bool
    sprite: Vec[Sprite]
}

impl SpriteSheet{
    pub fn new(path) -> spritesheet;
    pub fn load($self) -> result;
    pub fn unload(&self) -> result;
}
