use image::imageops::FilterType;

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Sprite {
    pub fn load(path: &str, target_size: u32) -> Self {
        let img = image::open(path).unwrap_or_else(|e| panic!("載入 {path} 失敗: {e}"));
        let rgba = img
            .resize_exact(target_size, target_size, FilterType::Nearest)
            .to_rgba8();

        let pixels = rgba
            .pixels()
            .map(|p| {
                let [r, g, b, a] = p.0;
                let a_u32 = a as u32;
                let r = (r as u32 * a_u32) / 255;
                let g = (g as u32 * a_u32) / 255;
                let b = (b as u32 * a_u32) / 255;
                (a_u32 << 24) | (r << 16) | (g << 8) | b
            })
            .collect();

        Self {
            width: target_size as usize,
            height: target_size as usize,
            pixels,
        }
    }
}

pub struct SpriteSet {
    pub idle: Sprite,
    pub pet: Sprite,
    pub type_frames: [Sprite; 2],
    pub sleep_frames: [Sprite; 3],
}

impl SpriteSet {
    pub fn load_all(dir: &str, size: u32) -> Self {
        Self {
            idle: Sprite::load(&format!("{dir}/idle.png"), size),
            pet: Sprite::load(&format!("{dir}/pet.png"), size),
            type_frames: [
                Sprite::load(&format!("{dir}/type1.png"), size),
                Sprite::load(&format!("{dir}/type2.png"), size),
            ],
            sleep_frames: [
                Sprite::load(&format!("{dir}/sleep1.png"), size),
                Sprite::load(&format!("{dir}/sleep2.png"), size),
                Sprite::load(&format!("{dir}/sleep3.png"), size),
            ],
        }
    }
}
