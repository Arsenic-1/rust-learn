use eyre::{Result, eyre};
use num_traits::{Float, Num, NumCast, Signed};
use std::fmt::Debug;

/// Trait alias for numeric types that can be used in `Vec2`.
pub trait Scalar: Num + NumCast + Copy {}
impl<T: Num + NumCast + Copy> Scalar for T {}

/// Trait alias for floating-point types that can also be cast from other numbers.
pub trait FloatCast: Float + NumCast {}
impl<T: Float + NumCast> FloatCast for T {}

/// Trait for 2D vector-like types.
trait Vec2Like<T: Scalar> {
    fn x(&self) -> T;
    fn y(&self) -> T;

    /// Euclidean length of the vector as a floating-point value.
    fn length<F: FloatCast>(&self) -> F {
        let x = F::from(self.x()).expect("cast failed");
        let y = F::from(self.y()).expect("cast failed");

        (x * x + y * y).sqrt()
    }

    /// Euclidean distance between `self` and another vector.
    fn distance<F: FloatCast>(&self, other: &impl Vec2Like<T>) -> F {
        let dx = F::from(other.x() - self.x()).expect("cast failed");
        let dy = F::from(other.y() - self.y()).expect("cast failed");

        (dx * dx + dy * dy).sqrt()
    }

    /// Absolute value per component (only for signed types).
    fn abs(&self) -> Vec2<T>
    where
        T: Signed,
    {
        Vec2::new(self.x().abs(), self.y().abs())
    }
}

/// A generic 2D vector type.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2<T: Scalar> {
    x: T,
    y: T,
}

impl<T: Scalar> Vec2Like<T> for Vec2<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }
}

impl<T: Scalar> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Normalize into a `UnitVec2`, erroring if zero-length.
    fn normalize<F: FloatCast>(self) -> Result<UnitVec2<F>> {
        let len = self.length::<F>();
        if len == F::zero() {
            return Err(eyre!("Cannot normalize a zero-length vector"));
        }

        let x = F::from(self.x).expect("cast failed");
        let y = F::from(self.y).expect("cast failed");

        Ok(UnitVec2 {
            x: x / len,
            y: y / len,
        })
    }
}

impl<T: Scalar> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Vec2::new(x, y)
    }
}

impl<T: Scalar> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Vec2::new(x, y)
    }
}

impl<T: Scalar> Into<(T, T)> for Vec2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

/// A 2D unit vector (always length = 1).
#[derive(Debug, Clone, Copy, PartialEq)]
struct UnitVec2<T: Scalar> {
    x: T,
    y: T,
}

impl<T: Scalar> Vec2Like<T> for UnitVec2<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }
}

impl<F: FloatCast> UnitVec2<F> {
    fn new<T: Scalar>(x: T, y: T) -> Result<Self> {
        Vec2::new(x, y).normalize::<F>()
    }

    fn from_vec2<T: Scalar>(v: Vec2<T>) -> Result<Self> {
        v.normalize::<F>()
    }

    fn as_vec2(self) -> Vec2<F> {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Player<T: Scalar, F: FloatCast> {
    position: Vec2<T>,
    direction: UnitVec2<F>,
}

impl<T: Scalar, F: FloatCast> Player<T, F> {
    fn new(position: Vec2<T>, direction: UnitVec2<F>) -> Self {
        Self {
            position,
            direction,
        }
    }
}

fn main() -> Result<()> {
    let player_i8 = Player::new((3i8, 4i8).into(), UnitVec2::<f32>::new(1i8, 0i8)?);
    println!("i8 player: {:?}", player_i8);

    let player_u128 = Player::new((3u128, 4u128).into(), UnitVec2::<f64>::new(0u128, 1u128)?);
    println!("u128 player: {:?}", player_u128);

    let player_isize = Player::new(
        (3isize, 4isize).into(),
        UnitVec2::<f32>::new(1isize, 0isize)?,
    );
    println!("isize player: {:?}", player_isize);

    let player_f32 = Player::new(
        (3.0f32, 4.0f32).into(),
        UnitVec2::<f32>::new(1.0f32, 0.0f32)?,
    );
    println!("f32 player: {:?}", player_f32);

    let player_f64 = Player::new(
        (3.0f64, 4.0f64).into(),
        UnitVec2::<f64>::new(1.0f64, 0.0f64)?,
    );
    println!("f64 player: {:?}", player_f64);

    Ok(())
}
