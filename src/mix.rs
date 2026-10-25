#![allow(dead_code, non_snake_case)]

#[derive(Debug, Clone)]
pub enum Types {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    String(String),
    Bool(bool),
    Usize(usize),
    Vec(Vec<Types>),
    None,
}

pub trait IsTypes {
    fn is_numeric(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn is_none(&self) -> bool;
    fn is_vec(&self) -> bool;
}

pub trait UnwrapTypes {
    fn unwrap_int(&self) -> Result<i128, Types>;
    fn unwrap_uns(&self) -> Result<u128, Types>;
    fn unwrap_float(&self) -> Result<f64, Types>;
    fn unwrap_str(&self) -> Result<String, Types>;
    fn unwrap_bool(&self) -> Result<bool, Types>;
    fn unwrap_vec(&self) -> Result<Vec<Types>, Types>;
}

pub trait ExpectTypes {
    fn expect_int(&self, msg: &str) -> i128;
    fn expect_uns(&self, msg: &str) -> u128;
    fn expect_float(&self, msg: &str) -> f64;
    fn expect_str(&self, msg: &str) -> String;
    fn expect_bool(&self, msg: &str) -> bool;
    fn expect_vec(&self, msg: &str) -> Vec<Types>;
}

pub trait ConvertTypes {
    fn to_float(&self) -> Types;
    fn to_int(&self) -> Types;
    fn to_uns(&self) -> Types;
    fn as_str(&self) -> &str;
}

pub trait Get {
    fn get_types(&self, arg: &[usize]) -> Vec<Types>;
    fn get_type(&self, arg: usize) -> Types;
}

pub trait Math {
    fn sum(&self) -> f64;
    fn min(&self) -> f64;
    fn mul(&self) -> f64;
    fn dev(&self) -> f64;
}

impl Math for Vec<Types> {
    fn sum(&self) -> f64 {
        let mut res: f64 = 0.0;
        for i in self {
            match i {
                Types::U8(x) => res += *x as f64,
                Types::U16(x) => res += *x as f64,
                Types::U32(x) => res += *x as f64,
                Types::U64(x) => res += *x as f64,
                Types::U128(x) => res += *x as f64,
                Types::I8(x) => res += *x as f64,
                Types::I16(x) => res += *x as f64,
                Types::I32(x) => res += *x as f64,
                Types::I64(x) => res += *x as f64,
                Types::I128(x) => res += *x as f64,
                Types::F32(x) => res += *x as f64,
                Types::F64(x) => res += *x as f64,
                _ => {}
            }
        }
        res
    }

    fn min(&self) -> f64 {
        let mut res: f64 = 1.0;
        for i in self {
            match i {
                Types::U8(x) => res -= *x as f64,
                Types::U16(x) => res -= *x as f64,
                Types::U32(x) => res -= *x as f64,
                Types::U64(x) => res -= *x as f64,
                Types::U128(x) => res -= *x as f64,
                Types::I8(x) => res -= *x as f64,
                Types::I16(x) => res -= *x as f64,
                Types::I32(x) => res -= *x as f64,
                Types::I64(x) => res -= *x as f64,
                Types::I128(x) => res -= *x as f64,
                Types::F32(x) => res -= *x as f64,
                Types::F64(x) => res -= *x as f64,
                _ => {}
            }
        }
        res
    }

    fn mul(&self) -> f64 {
        let mut res: f64 = 1.0;
        for i in self {
            match i {
                Types::U8(x) => res *= *x as f64,
                Types::U16(x) => res *= *x as f64,
                Types::U32(x) => res *= *x as f64,
                Types::U64(x) => res *= *x as f64,
                Types::U128(x) => res *= *x as f64,
                Types::I8(x) => res *= *x as f64,
                Types::I16(x) => res *= *x as f64,
                Types::I32(x) => res *= *x as f64,
                Types::I64(x) => res *= *x as f64,
                Types::I128(x) => res *= *x as f64,
                Types::F32(x) => res *= *x as f64,
                Types::F64(x) => res *= *x as f64,
                _ => {}
            }
        }
        res
    }

    fn dev(&self) -> f64 {
        let mut res: f64 = 1.0;
        for i in self {
            match i {
                Types::U8(x) => res /= *x as f64,
                Types::U16(x) => res /= *x as f64,
                Types::U32(x) => res /= *x as f64,
                Types::U64(x) => res /= *x as f64,
                Types::U128(x) => res /= *x as f64,
                Types::I8(x) => res /= *x as f64,
                Types::I16(x) => res /= *x as f64,
                Types::I32(x) => res /= *x as f64,
                Types::I64(x) => res /= *x as f64,
                Types::I128(x) => res /= *x as f64,
                Types::F32(x) => res /= *x as f64,
                Types::F64(x) => res /= *x as f64,
                _ => {}
            }
        }
        res
    }
}

impl Get for Vec<Types> {
    fn get_types(&self, arg: &[usize]) -> Vec<Types> {
        let mut vec: Vec<Types> = Vec::new();

        for i in arg {
            match self.get(*i) {
                Some(x) => match x {
                    Types::U8(x) => vec.push(Types::U8(*x)),
                    Types::U16(x) => vec.push(Types::U16(*x)),
                    Types::U32(x) => vec.push(Types::U32(*x)),
                    Types::U64(x) => vec.push(Types::U64(*x)),
                    Types::U128(x) => vec.push(Types::U128(*x)),
                    Types::I8(x) => vec.push(Types::I8(*x)),
                    Types::I16(x) => vec.push(Types::I16(*x)),
                    Types::I32(x) => vec.push(Types::I32(*x)),
                    Types::I64(x) => vec.push(Types::I64(*x)),
                    Types::I128(x) => vec.push(Types::I128(*x)),
                    Types::F32(x) => vec.push(Types::F32(*x)),
                    Types::F64(x) => vec.push(Types::F64(*x)),
                    Types::String(x) => vec.push(Types::String(x.clone())),
                    Types::Bool(x) => vec.push(Types::Bool(*x)),
                    Types::None => vec.push(Types::None),
                    Types::Usize(x) => vec.push(Types::Usize(*x)),
                    Types::Vec(x) => vec.push(Types::Vec(x.clone())),
                },
                None => vec.push(Types::None),
            }
        }

        vec
    }
    fn get_type(&self, arg: usize) -> Types {
        match self.get(arg) {
            Some(x) => match x {
                Types::U8(x) => Types::U8(*x),
                Types::U16(x) => Types::U16(*x),
                Types::U32(x) => Types::U32(*x),
                Types::U64(x) => Types::U64(*x),
                Types::U128(x) => Types::U128(*x),
                Types::I8(x) => Types::I8(*x),
                Types::I16(x) => Types::I16(*x),
                Types::I32(x) => Types::I32(*x),
                Types::I64(x) => Types::I64(*x),
                Types::I128(x) => Types::I128(*x),
                Types::F32(x) => Types::F32(*x),
                Types::F64(x) => Types::F64(*x),
                Types::String(x) => Types::String(x.clone()),
                Types::Bool(x) => Types::Bool(*x),
                Types::None => Types::None,
                Types::Usize(x) => Types::Usize(*x),
                Types::Vec(x) => Types::Vec(x.clone()),
            },
            None => Types::None,
        }
    }
}

impl ConvertTypes for Types {
    fn to_float(&self) -> Types {
        match self {
            Types::U8(x) => Types::F64(*x as f64),
            Types::U16(x) => Types::F64(*x as f64),
            Types::U32(x) => Types::F64(*x as f64),
            Types::U64(x) => Types::F64(*x as f64),
            Types::U128(x) => Types::F64(*x as f64),
            Types::I8(x) => Types::F64(*x as f64),
            Types::I16(x) => Types::F64(*x as f64),
            Types::I32(x) => Types::F64(*x as f64),
            Types::I64(x) => Types::F64(*x as f64),
            Types::I128(x) => Types::F64(*x as f64),
            Types::F32(x) => Types::F64(*x as f64),
            Types::F64(x) => Types::F64(*x as f64),
            Types::Usize(x) => Types::F64(*x as f64),
            _ => panic!("Invalid Type"),
        }
    }

    fn to_uns(&self) -> Types {
        match self {
            Types::U8(x) => Types::U128(*x as u128),
            Types::U16(x) => Types::U128(*x as u128),
            Types::U32(x) => Types::U128(*x as u128),
            Types::U64(x) => Types::U128(*x as u128),
            Types::U128(x) => Types::U128(*x as u128),
            Types::I8(x) => Types::U128(*x as u128),
            Types::I16(x) => Types::U128(*x as u128),
            Types::I32(x) => Types::U128(*x as u128),
            Types::I64(x) => Types::U128(*x as u128),
            Types::I128(x) => Types::U128(*x as u128),
            Types::F32(x) => Types::U128(*x as u128),
            Types::F64(x) => Types::U128(*x as u128),
            Types::Usize(x) => Types::U128(*x as u128),
            _ => panic!("Eror"),
        }
    }

    fn to_int(&self) -> Types {
        match self {
            Types::U8(x) => Types::I128(*x as i128),
            Types::U16(x) => Types::I128(*x as i128),
            Types::U32(x) => Types::I128(*x as i128),
            Types::U64(x) => Types::I128(*x as i128),
            Types::U128(x) => Types::I128(*x as i128),
            Types::I8(x) => Types::I128(*x as i128),
            Types::I16(x) => Types::I128(*x as i128),
            Types::I32(x) => Types::I128(*x as i128),
            Types::I64(x) => Types::I128(*x as i128),
            Types::I128(x) => Types::I128(*x as i128),
            Types::F32(x) => Types::I128(*x as i128),
            Types::F64(x) => Types::I128(*x as i128),
            Types::Usize(x) => Types::I128(*x as i128),
            _ => panic!("Eror"),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Types::String(x) => x.as_str(),
            _ => panic!("Invalid Type"),
        }
    }
}

impl IsTypes for Types {
    fn is_numeric(&self) -> bool {
        match self {
            Types::U8(_) => true,
            Types::U16(_) => true,
            Types::U32(_) => true,
            Types::U64(_) => true,
            Types::U128(_) => true,
            Types::I8(_) => true,
            Types::I16(_) => true,
            Types::I32(_) => true,
            Types::I64(_) => true,
            Types::I128(_) => true,
            Types::F32(_) => true,
            Types::F64(_) => true,
            Types::Usize(_) => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        match self {
            Types::String(_) => true,
            _ => false,
        }
    }

    fn is_bool(&self) -> bool {
        match self {
            Types::Bool(_) => true,
            _ => false,
        }
    }

    fn is_none(&self) -> bool {
        match self {
            Types::None => true,
            _ => false,
        }
    }

    fn is_vec(&self) -> bool {
        match self {
            Types::Vec(_) => true,
            _ => false,
        }
    }
}

impl UnwrapTypes for Types {
    fn unwrap_int(&self) -> Result<i128, Types> {
        match self {
            Types::U8(x) => Ok(*x as i128),
            Types::U16(x) => Ok(*x as i128),
            Types::U32(x) => Ok(*x as i128),
            Types::U64(x) => Ok(*x as i128),
            Types::U128(x) => Ok(*x as i128),
            Types::I8(x) => Ok(*x as i128),
            Types::I16(x) => Ok(*x as i128),
            Types::I64(x) => Ok(*x as i128),
            Types::I128(x) => Ok(*x as i128),
            Types::F32(x) => Ok(*x as i128),
            Types::F64(x) => Ok(*x as i128),
            Types::Usize(x) => Ok(*x as i128),
            _ => Err(Types::None),
        }
    }

    fn unwrap_uns(&self) -> Result<u128, Types> {
        match self {
            Types::U8(x) => Ok(*x as u128),
            Types::U16(x) => Ok(*x as u128),
            Types::U32(x) => Ok(*x as u128),
            Types::U64(x) => Ok(*x as u128),
            Types::U128(x) => Ok(*x as u128),
            Types::I8(x) => Ok(*x as u128),
            Types::I16(x) => Ok(*x as u128),
            Types::I64(x) => Ok(*x as u128),
            Types::I128(x) => Ok(*x as u128),
            Types::F32(x) => Ok(*x as u128),
            Types::F64(x) => Ok(*x as u128),
            Types::Usize(x) => Ok(*x as u128),
            _ => Err(Types::None),
        }
    }

    fn unwrap_float(&self) -> Result<f64, Types> {
        match self {
            Types::U8(x) => Ok(*x as f64),
            Types::U16(x) => Ok(*x as f64),
            Types::U32(x) => Ok(*x as f64),
            Types::U64(x) => Ok(*x as f64),
            Types::U128(x) => Ok(*x as f64),
            Types::I8(x) => Ok(*x as f64),
            Types::I16(x) => Ok(*x as f64),
            Types::I64(x) => Ok(*x as f64),
            Types::I128(x) => Ok(*x as f64),
            Types::F32(x) => Ok(*x as f64),
            Types::F64(x) => Ok(*x as f64),
            Types::Usize(x) => Ok(*x as f64),
            _ => Err(Types::None),
        }
    }

    fn unwrap_str(&self) -> Result<String, Types> {
        match self {
            Types::String(x) => Ok(x.clone()),
            _ => Err(Types::None),
        }
    }

    fn unwrap_bool(&self) -> Result<bool, Types> {
        match self {
            Types::Bool(x) => Ok(*x),
            _ => Err(Types::None),
        }
    }

    fn unwrap_vec(&self) -> Result<Vec<Types>, Types> {
        match self {
            Types::Vec(x) => Ok(x.clone()),
            _ => Err(Types::None),
        }
    }
}

impl ExpectTypes for Types {
    fn expect_int(&self, msg: &str) -> i128 {
        match self {
            Types::U8(x) => *x as i128,
            Types::U16(x) => *x as i128,
            Types::U32(x) => *x as i128,
            Types::U64(x) => *x as i128,
            Types::U128(x) => *x as i128,
            Types::I8(x) => *x as i128,
            Types::I16(x) => *x as i128,
            Types::I32(x) => *x as i128,
            Types::I64(x) => *x as i128,
            Types::I128(x) => *x as i128,
            Types::F32(x) => *x as i128,
            Types::F64(x) => *x as i128,
            Types::Usize(x) => *x as i128,
            _ => panic!("{}", msg),
        }
    }

    fn expect_uns(&self, msg: &str) -> u128 {
        match self {
            Types::U8(x) => *x as u128,
            Types::U16(x) => *x as u128,
            Types::U32(x) => *x as u128,
            Types::U64(x) => *x as u128,
            Types::U128(x) => *x as u128,
            Types::I8(x) => *x as u128,
            Types::I16(x) => *x as u128,
            Types::I32(x) => *x as u128,
            Types::I64(x) => *x as u128,
            Types::I128(x) => *x as u128,
            Types::F32(x) => *x as u128,
            Types::F64(x) => *x as u128,
            Types::Usize(x) => *x as u128,
            _ => panic!("{}", msg),
        }
    }

    fn expect_float(&self, msg: &str) -> f64 {
        match self {
            Types::U8(x) => *x as f64,
            Types::U16(x) => *x as f64,
            Types::U32(x) => *x as f64,
            Types::U64(x) => *x as f64,
            Types::U128(x) => *x as f64,
            Types::I8(x) => *x as f64,
            Types::I16(x) => *x as f64,
            Types::I32(x) => *x as f64,
            Types::I64(x) => *x as f64,
            Types::I128(x) => *x as f64,
            Types::F32(x) => *x as f64,
            Types::F64(x) => *x as f64,
            Types::Usize(x) => *x as f64,
            _ => panic!("{}", msg),
        }
    }

    fn expect_str(&self, msg: &str) -> String {
        match self {
            Types::String(x) => x.clone(),
            _ => panic!("{}", msg),
        }
    }

    fn expect_bool(&self, msg: &str) -> bool {
        match self {
            Types::Bool(x) => *x,
            _ => panic!("{}", msg),
        }
    }

    fn expect_vec(&self, msg: &str) -> Vec<Types> {
        match self {
            Types::Vec(x) => x.clone(),
            _ => panic!("{}", msg),
        }
    }
}

pub fn new() -> Vec<Types> {
    let vec: Vec<Types> = Vec::new();
    vec
}

pub fn from(arg: Types) -> Vec<Types> {
    let mut vec: Vec<Types> = Vec::new();
    vec.push(arg);
    vec
}

pub fn from_arr(arg: &[Types]) -> Vec<Types> {
    let mut vec: Vec<Types> = Vec::new();
    for i in arg {
        match i {
            Types::U8(x) => vec.push(Types::U8(*x)),
            Types::U16(x) => vec.push(Types::U16(*x)),
            Types::U32(x) => vec.push(Types::U32(*x)),
            Types::U64(x) => vec.push(Types::U64(*x)),
            Types::U128(x) => vec.push(Types::U128(*x)),
            Types::I8(x) => vec.push(Types::I8(*x)),
            Types::I16(x) => vec.push(Types::I16(*x)),
            Types::I32(x) => vec.push(Types::I32(*x)),
            Types::I64(x) => vec.push(Types::I64(*x)),
            Types::I128(x) => vec.push(Types::I128(*x)),
            Types::F32(x) => vec.push(Types::F32(*x)),
            Types::F64(x) => vec.push(Types::F64(*x)),
            Types::String(x) => vec.push(Types::String(x.clone())),
            Types::Bool(x) => vec.push(Types::Bool(*x)),
            Types::None => vec.push(Types::None),
            Types::Usize(x) => vec.push(Types::Usize(*x)),
            Types::Vec(x) => vec.push(Types::Vec(x.clone())),
        }
    }
    vec
}
