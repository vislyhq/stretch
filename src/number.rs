use core::ops;
use decorum::R32;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Number {
    Defined(R32),
    Undefined,
}

pub trait ToNumber {
    fn to_number(self) -> Number;
}

pub trait OrElse<T> {
    fn or_else(self, other: T) -> T;
}

impl Default for Number {
    fn default() -> Number {
        Number::Undefined
    }
}

impl OrElse<R32> for Number {
    fn or_else(self, other: R32) -> R32 {
        match self {
            Number::Defined(val) => val,
            Number::Undefined => other,
        }
    }
}

impl OrElse<Number> for Number {
    fn or_else(self, other: Number) -> Number {
        match self {
            Number::Defined(_) => self,
            Number::Undefined => other,
        }
    }
}

impl Number {
    pub fn is_defined(self) -> bool {
        match self {
            Number::Defined(_) => true,
            Number::Undefined => false,
        }
    }

    pub fn is_undefined(self) -> bool {
        match self {
            Number::Defined(_) => false,
            Number::Undefined => true,
        }
    }
}

pub trait MinMax<In, Out> {
    fn maybe_min(self, rhs: In) -> Out;
    fn maybe_max(self, rhs: In) -> Out;
}

impl MinMax<Number, Number> for Number {
    fn maybe_min(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val.min(other)),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }

    fn maybe_max(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val.max(other)),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}

impl MinMax<R32, Number> for Number {
    fn maybe_min(self, rhs: R32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val.min(rhs)),
            Number::Undefined => Number::Undefined,
        }
    }

    fn maybe_max(self, rhs: R32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val.max(rhs)),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl MinMax<Number, R32> for R32 {
    fn maybe_min(self, rhs: Number) -> R32 {
        match rhs {
            Number::Defined(val) => self.min(val),
            Number::Undefined => self,
        }
    }

    fn maybe_max(self, rhs: Number) -> R32 {
        match rhs {
            Number::Defined(val) => self.max(val),
            Number::Undefined => self,
        }
    }
}

impl ToNumber for R32 {
    fn to_number(self) -> Number {
        Number::Defined(self)
    }
}

impl ops::Add<R32> for Number {
    type Output = Number;

    fn add(self, rhs: R32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val + rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val + other),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Sub<R32> for Number {
    type Output = Number;

    fn sub(self, rhs: R32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val - rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val - other),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Mul<R32> for Number {
    type Output = Number;

    fn mul(self, rhs: R32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val * rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val * other),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Div<R32> for Number {
    type Output = Number;

    fn div(self, rhs: R32) -> Number {
        match self {
            Number::Defined(val) => Number::Defined(val / rhs),
            Number::Undefined => Number::Undefined,
        }
    }
}

impl ops::Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Number {
        match self {
            Number::Defined(val) => match rhs {
                Number::Defined(other) => Number::Defined(val / other),
                Number::Undefined => self,
            },
            Number::Undefined => Number::Undefined,
        }
    }
}
