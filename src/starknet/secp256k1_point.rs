use super::U256;
use crate::Value;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Secp256k1Point {
    pub x: U256,
    pub y: U256,
}

impl Secp256k1Point {
    pub(crate) fn into_value(self) -> Value {
        Value::Struct(vec![
            Value::U256(self.x.lo, self.x.hi),
            Value::U256(self.y.lo, self.y.hi),
        ])
    }
}
