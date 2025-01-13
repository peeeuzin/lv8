use std::{fmt::Debug, ops};

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Debug, R: Debug> Debug for Either<L, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::Left(value) => write!(f, "{:?}", value),
            Either::Right(value) => write!(f, "{:?}", value),
        }
    }
}

impl ops::Add for Either<isize, f64> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Either::Left(left), Either::Left(right)) => Either::Left(left + right),
            (Either::Right(left), Either::Right(right)) => Either::Right(left + right),
            (Either::Left(left), Either::Right(right)) => Either::Right(left as f64 + right),
            (Either::Right(left), Either::Left(right)) => Either::Right(left + right as f64),
        }
    }
}

impl ops::Sub for Either<isize, f64> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Either::Left(left), Either::Left(right)) => Either::Left(left - right),
            (Either::Right(left), Either::Right(right)) => Either::Right(left - right),
            (Either::Left(left), Either::Right(right)) => Either::Right(left as f64 - right),
            (Either::Right(left), Either::Left(right)) => Either::Right(left - right as f64),
        }
    }
}

impl ops::Mul for Either<isize, f64> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Either::Left(left), Either::Left(right)) => Either::Left(left * right),
            (Either::Right(left), Either::Right(right)) => Either::Right(left * right),
            (Either::Left(left), Either::Right(right)) => Either::Right(left as f64 * right),
            (Either::Right(left), Either::Left(right)) => Either::Right(left * right as f64),
        }
    }
}

impl ops::Div for Either<isize, f64> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Either::Left(left), Either::Left(right)) => Either::Left(left / right),
            (Either::Right(left), Either::Right(right)) => Either::Right(left / right),
            (Either::Left(left), Either::Right(right)) => Either::Right(left as f64 / right),
            (Either::Right(left), Either::Left(right)) => Either::Right(left / right as f64),
        }
    }
}

impl ops::Rem for Either<isize, f64> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (Either::Left(left), Either::Left(right)) => Either::Left(left % right),
            (Either::Right(left), Either::Right(right)) => Either::Right(left % right),
            (Either::Left(left), Either::Right(right)) => Either::Right(left as f64 % right),
            (Either::Right(left), Either::Left(right)) => Either::Right(left % right as f64),
        }
    }
}
