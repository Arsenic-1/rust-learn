use eyre::{Result, eyre};

trait Vec2Like {
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn length(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y()).sqrt()
    }

    fn distance<T: Vec2Like>(&self, other: &T) -> f32 {
        ((other.x() - self.x()).powi(2) + (other.y() - self.y()).powi(2)).sqrt()
    }

    fn abs(&self) -> Vec2 {
        Vec2::new(self.x().abs(), self.y().abs())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2Like for Vec2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn normalize(self) -> Result<UnitVec2> {
        let len = self.length();
        if len == 0.0 {
            return Err(eyre!("Cannot normalize a zero-length vector"));
        }

        Ok(UnitVec2 {
            x: self.x / len,
            y: self.y / len,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct UnitVec2 {
    x: f32,
    y: f32,
}

impl Vec2Like for UnitVec2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl UnitVec2 {
    fn new(x: f32, y: f32) -> Result<Self> {
        Vec2::new(x, y).normalize()
    }

    fn from_vec2(v: Vec2) -> Result<Self> {
        v.normalize()
    }

    fn as_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Player {
    position: Vec2,
    direction: UnitVec2,
}

impl Player {
    fn new(position: Vec2, direction: UnitVec2) -> Self {
        Self {
            position,
            direction,
        }
    }
}

fn main() -> eyre::Result<()> {
    // This will error
    let player = Player::new(Vec2::new(0.0, 0.0), UnitVec2::new(1.0, 0.0)?);
    println!("{:?}", player);

    Ok(())
}
