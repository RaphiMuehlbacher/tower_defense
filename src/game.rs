use crate::tower::{Position, Tower};

pub struct Game {
    pub towers: Vec<Tower>,
    pub health: usize,
}


impl Game {
    pub fn new() -> Game {
        let archer_tower = Tower::new_archer(Position::new(50, 80));
        Game { towers: vec![archer_tower], health: 100 }
    }

    pub fn render(&self, frame: &mut [u8], width: u32, height: u32) {
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            pixel[3] = 255;
        }

        for tower in &self.towers {
            tower.render_tower(frame, width, height);
        }
    }

    pub fn add_tower(&mut self, tower: Tower) {
        self.towers.push(tower);
    }
}
