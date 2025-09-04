use eyre::{Result, eyre};
use num_traits::{Num, NumCast, Signed};
use std::fmt::Debug;

trait Vec2Like<T: Num + NumCast + Copy> {
    fn x(&self) -> T;
    fn y(&self) -> T;

    fn length(&self) -> Result<f64> {
        let x: f64 = NumCast::from(self.x()).ok_or_else(|| eyre!("Failed to cast x"))?;
        let y: f64 = NumCast::from(self.y()).ok_or_else(|| eyre!("Failed to cast y"))?;
        Ok((x * x + y * y).sqrt())
    }

    fn distance<U: Vec2Like<T>>(&self, other: &U) -> Result<f64> {
        let dx_val = other.x() - self.x();
        let dy_val = other.y() - self.y();
        let dx: f64 = NumCast::from(dx_val).ok_or_else(|| eyre!("Failed to cast dx"))?;
        let dy: f64 = NumCast::from(dy_val).ok_or_else(|| eyre!("Failed to cast dy"))?;
        Ok((dx * dx + dy * dy).sqrt())
    }

    fn abs(&self) -> Result<Vec2<T>>
    where
        T: Signed,
    {
        Ok(Vec2::new(self.x().abs(), self.y().abs()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2<T: Num + NumCast + Copy> {
    x: T,
    y: T,
}

impl<T: Num + NumCast + Copy> Vec2Like<T> for Vec2<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }
}

impl<T: Num + NumCast + Copy> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn normalize(self) -> Result<UnitVec2<f64>> {
        let len = self.length()?;
        if len == 0.0 {
            return Err(eyre!("Cannot normalize a zero-length vector"));
        }

        let x: f64 = NumCast::from(self.x).ok_or_else(|| eyre!("Failed to cast x"))?;
        let y: f64 = NumCast::from(self.y).ok_or_else(|| eyre!("Failed to cast y"))?;

        Ok(UnitVec2 {
            x: x / len,
            y: y / len,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct UnitVec2<T: Num + NumCast + Copy> {
    x: T,
    y: T,
}

impl<T: Num + NumCast + Copy> Vec2Like<T> for UnitVec2<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }
}

impl UnitVec2<f64> {
    fn new<T: Num + NumCast + Copy>(x: T, y: T) -> Result<Self> {
        Vec2::new(x, y).normalize()
    }

    fn from_vec2<T: Num + NumCast + Copy>(v: Vec2<T>) -> Result<Self> {
        v.normalize()
    }

    fn as_vec2(self) -> Vec2<f64> {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Player<T: Num + NumCast + Copy> {
    position: Vec2<T>,
    direction: UnitVec2<f64>,
}

impl<T: Num + NumCast + Copy> Player<T> {
    fn new(position: Vec2<T>, direction: UnitVec2<f64>) -> Self {
        Self {
            position,
            direction,
        }
    }
}

fn main() -> Result<()> {
    // i8
    let player_i8 = Player::new(Vec2::new(3i8, 4i8), UnitVec2::new(1i8, 0i8)?);
    println!("i8 player: {:?}", player_i8);

    // u128
    let player_u128 = Player::new(Vec2::new(3u128, 4u128), UnitVec2::new(0u128, 1u128)?);
    println!("u128 player: {:?}", player_u128);

    // isize
    let player_isize = Player::new(Vec2::new(3isize, 4isize), UnitVec2::new(1isize, 0isize)?);
    println!("isize player: {:?}", player_isize);

    // f32
    let player_f32 = Player::new(Vec2::new(3.0f32, 4.0f32), UnitVec2::new(1.0f32, 0.0f32)?);
    println!("f32 player: {:?}", player_f32);

    // f64
    let player_f64 = Player::new(Vec2::new(3.0f64, 4.0f64), UnitVec2::new(1.0f64, 0.0f64)?);
    println!("f64 player: {:?}", player_f64);

    Ok(())
}
