pub struct Set {
    pub num_red: i32,
    pub num_green: i32,
    pub num_blue: i32,
}
impl Set {
    pub(crate) fn new() -> Set {
        Set {
            num_red: 0,
            num_green: 0,
            num_blue: 0,
        }
    }
}

pub struct Game {
    pub id: i32,
    pub sets: Vec<Set>,
}

impl Game {
    pub(crate) fn new() -> Game {
        Game {
            id: -1,
            sets: Vec::new(),
        }
    }

    pub fn is_valid(&mut self, max_r: i32, max_g: i32, max_b: i32) -> bool {
        let total_r = self.get_total(Color::Red);
        let total_g = self.get_total(Color::Green);
        let total_b = self.get_total(Color::Blue);

        println!("GAME {}", self.id);
        println!("({}; {})", total_r, max_r);
        println!("({}; {})", total_g, max_g);
        println!("({}; {})", total_b, max_b);
        println!(
            "VALID: {}",
            total_r <= max_r && total_g <= max_g && total_b <= max_b
        );
        println!("---------");

        total_r <= max_r && total_g <= max_g && total_b <= max_b
    }

    pub fn get_total(&mut self, color: Color) -> i32 {
        let mut total = 0;

        for set in &self.sets {
            match color {
                Color::Red => {
                    if total < set.num_red {
                        total = set.num_red
                    }
                }
                Color::Green => {
                    if total < set.num_green {
                        total = set.num_green
                    }
                }
                Color::Blue => {
                    if total < set.num_blue {
                        total = set.num_blue
                    }
                }
            }
        }

        total
    }
}

pub enum Color {
    Red,
    Green,
    Blue,
}
