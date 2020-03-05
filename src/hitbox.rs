use nalgebra::Point2;
use ggez::graphics::Rect;

pub struct Hitbox {
    rect: Rect,
    target: String,
}

pub struct HitboxManager {
    hitboxes: Vec<Hitbox>,
}

impl HitboxManager {
    pub fn new() -> Self {
        Self {
            hitboxes: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.hitboxes = vec![];
    }

    pub fn add(&mut self, rect: Rect, target: &str) {
        let hitbox = Hitbox {
            rect,
            target: target.to_owned(),
        };

        self.hitboxes.push(hitbox);
    }

    pub fn targets_at(&mut self, p: Point2<f32>) -> Vec<String> {
        self.hitboxes.iter().filter_map(|hitbox| {
            if hitbox.rect.contains(p) {
                Some(hitbox.target.to_owned())
            } else {
                None
            }
        }).collect()
    }
}
