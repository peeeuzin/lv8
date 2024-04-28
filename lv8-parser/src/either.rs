use std::ops;

#[derive(Clone, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl ToString for Either<i64, f64> {
    fn to_string(&self) -> String {
        match self {
            Either::Left(value) => value.to_string(),
            Either::Right(value) => value.to_string(),
        }
    }
}

impl ops::Add for Either<i64, f64> {
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

impl ops::Sub for Either<i64, f64> {
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

impl ops::Mul for Either<i64, f64> {
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

impl ops::Div for Either<i64, f64> {
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

impl ops::Rem for Either<i64, f64> {
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
