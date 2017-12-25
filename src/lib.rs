extern crate num;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


use num::bigint::{BigInt, BigUint};
use std::ops::{Add, Deref, Sub};
use num::traits::ops::checked::{CheckedAdd, CheckedSub};
use serde::ser::Serialize;
use serde::{Deserialize, Deserializer, Serializer};
use std::str::FromStr;


#[derive(Clone, Debug, PartialEq)]
pub struct Uint256(BigUint);

impl Deref for Uint256 {
    type Target = BigUint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Uint256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_str_radix(10))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Uint256 {
    fn deserialize<D>(deserializer: D) -> Result<Uint256, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;

        BigUint::from_str(s).map(|v| Uint256(v)).map_err(
            serde::de::Error::custom,
        )
    }
}

impl Add for Uint256 {
    type Output = Uint256;
    fn add(self, v: Uint256) -> Self::Output {
        let num = self.0 + v.0;
        if num.bits() > 256 {
            panic!("overflow");
        }
        Uint256(num)
    }
}

impl CheckedAdd for Uint256 {
    fn checked_add(&self, v: &Uint256) -> Option<Uint256> {
        let num = self.0.clone() + v.0.clone();
        if num.bits() > 256 {
            return None;
        }
        Some(Uint256(num))
    }
}

impl Sub for Uint256 {
    type Output = Uint256;
    fn sub(self, v: Uint256) -> Self::Output {
        Uint256(self.0 - v.0)
    }
}

impl CheckedSub for Uint256 {
    fn checked_sub(&self, v: &Uint256) -> Option<Uint256> {
        if self.0 < v.0 {
            return None;
        }
        let num = self.0.clone() - v.0.clone();
        //let inum = self.clone() - v.clone();
        Some(Uint256(num))
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Int256(BigInt);

impl Deref for Int256 {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Int256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_str_radix(10))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Int256 {
    fn deserialize<D>(deserializer: D) -> Result<Int256, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;

        BigInt::from_str(s).map(|v| Int256(v)).map_err(
            serde::de::Error::custom,
        )
    }
}

impl Add for Int256 {
    type Output = Int256;
    fn add(self, v: Int256) -> Self::Output {
        let num = self.0 + v.0;
        if num.bits() > 255 {
            panic!("overflow");
        }
        Int256(num)
    }
}

impl CheckedAdd for Int256 {
    fn checked_add(&self, v: &Int256) -> Option<Int256> {
        // drop down to wrapped bigint to stop from panicing in fn above
        let num = self.0.clone() + v.0.clone();
        if num.bits() > 255 {
            return None;
        }
        Some(Int256(num))
    }
}

impl Sub for Int256 {
    type Output = Int256;
    fn sub(self, v: Int256) -> Self::Output {
        let num = self.0 - v.0;
        if num.bits() > 255 {
            panic!("overflow");
        }
        Int256(num)
    }
}

impl CheckedSub for Int256 {
    fn checked_sub(&self, v: &Int256) -> Option<Int256> {
        // drop down to wrapped bigint to stop from panicing in fn above
        let num = self.0.clone() - v.0.clone();
        if num.bits() > 255 {
            return None;
        }
        Some(Int256(num))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use num::pow::pow;
    use num::traits::ops::checked::{CheckedAdd, CheckedSub};
    use num::traits::cast::ToPrimitive;
    use serde_json;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MyStruct {
        uint: Uint256,
        int: Int256,
    }

    #[test]
    fn serialize() {
        let struc = MyStruct {
            uint: Uint256(BigUint::from(234 as u32)),
            int: Int256(BigInt::from(333)),
        };


        let expected = "{\"uint\":\"234\",\"int\":\"333\"}";

        let j = serde_json::to_string(&struc).unwrap();


        assert_eq!(expected, j);
        let m: MyStruct = serde_json::from_str(expected).unwrap();

        assert_eq!(Uint256(BigUint::from(234 as u32)), m.uint);
        assert_eq!(Int256(BigInt::from(333)), m.int);
    }

    #[test]
    fn test_uint256() {
        let biggest = Uint256(pow(BigUint::from(2 as u32), 256) - BigUint::from(1 as u32));

        assert!(
            biggest
                .checked_add(&Uint256(BigUint::from(32 as u32)))
                .is_none(),
            "should return None adding 1 to biggest"
        );

        assert!(
            biggest
                .checked_add(&Uint256(BigUint::from(0 as u32)))
                .is_some(),
            "should return None adding 0 to biggest"
        );

        assert!(
            Uint256(BigUint::from(1 as u32))
                .checked_sub(&Uint256(BigUint::from(2 as u32)))
                .is_none(),
            "should return None if RHS is larger than LHS"
        );

        assert!(
            Uint256(BigUint::from(1 as u32))
                .checked_sub(&Uint256(BigUint::from(1 as u32)))
                .is_some(),
            "should return Some if RHS is not larger than LHS"
        );

        let num = Uint256(BigUint::from(1 as u32))
            .checked_sub(&Uint256(BigUint::from(1 as u32)))
            .unwrap()
            .to_u32()
            .unwrap();

        assert_eq!(num, 0, "1 - 1 should = 0");

        let num2 = Uint256(BigUint::from(346 as u32))
            .checked_sub(&Uint256(BigUint::from(23 as u32)))
            .unwrap()
            .to_u32()
            .unwrap();

        assert_eq!(num2, 323, "346 - 23 should = 323");
    }

    #[test]
    fn test_unchecked_uint256() {
        let biggest = Uint256(pow(BigUint::from(2 as u32), 256) - BigUint::from(1 as u32));
        let second_biggest = Uint256(pow(BigUint::from(2 as u32), 256) - BigUint::from(1 as u32));
        let third_biggest = Uint256(pow(BigUint::from(2 as u32), 256) - BigUint::from(1 as u32));
        
        let smallest = Uint256(BigUint::from(0 as u32));
        
        let cloned_smallest = smallest.clone();
        let cloned_biggest = biggest.clone();

        assert_eq!(
            biggest.sub(second_biggest),
            Uint256(BigUint::from(0 as u32)),
            "should return Zero subtracting  biggest from biggest"
        );

        assert_eq!(
            smallest.add(cloned_smallest),
            Uint256(BigUint::from(0 as u32)),
            "should return Zero  for adding  smallest to smallest"
        );

        assert_eq!(
            third_biggest.add(Uint256(BigUint::from(0 as u32))),
            cloned_biggest,
            "should return biggest for adding 0 to biggest"
        );

        let num = Uint256(BigUint::from(1 as u32))
            .add(Uint256(BigUint::from(1 as u32)))
            .to_u32()
            .unwrap();

        assert_eq!(num, 2, "1 + 1 should = 2");

        let num2 = Uint256(BigUint::from(346 as u32))
            .sub(Uint256(BigUint::from(23 as u32)))
            .to_u32()
            .unwrap();

        assert_eq!(num2, 323, "346 - 23 should = 323");
    }

    #[test]
    fn test_checked_int256() {
        let biggest = Int256(pow(BigInt::from(2), 255) - BigInt::from(1));
        let smallest = Int256(pow(BigInt::from(-2), 255) + BigInt::from(1));

        assert!(
            biggest.checked_add(&Int256(BigInt::from(64))).is_none(),
            "should return None adding 1 to biggest"
        );
        assert!(
            biggest.checked_add(&Int256(BigInt::from(0))).is_some(),
            "should return Some adding 1 to biggest"
        );

        assert!(
            smallest.checked_sub(&Int256(BigInt::from(1))).is_none(),
            "should return None subtracting 1 from smallest"
        );
        assert!(
            smallest.checked_sub(&Int256(BigInt::from(0))).is_some(),
            "should return Some subtracting 0 from smallest"
        );

        let num = Int256(BigInt::from(345))
            .checked_sub(&Int256(BigInt::from(44)))
            .unwrap()
            .to_u32()
            .unwrap();

        assert_eq!(num, 301, "345 - 44 should = 301");
    }

    #[test]
    fn test_unchecked_int256() {
        let biggest = Int256(pow(BigInt::from(2), 255) - BigInt::from(1));
        let smallest = Int256(pow(BigInt::from(-2), 255) + BigInt::from(1));

        let cloned_smallest = smallest.clone();
        let sub_zero_smallest = cloned_smallest.sub(Int256(BigInt::from(0)));

        assert_eq!(
            smallest,
            sub_zero_smallest,
            "should return smallest for subtracting 0 from smallest"
        );

        let cloned_biggest = biggest.clone();
        let add_one_biggest = cloned_biggest.add(Int256(BigInt::from(0)));

        assert_eq!(
            biggest,
            add_one_biggest,
            "should return biggest for subtracting 0 from biggest"
        );

        let num = Int256(BigInt::from(345))
            .sub(Int256(BigInt::from(44)))
            .to_u32()
            .unwrap();

        assert_eq!(num, 301, "345 - 44 should = 301");


        let num = Int256(BigInt::from(-345))
            .add(Int256(BigInt::from(44)))
            .to_i32()
            .unwrap();

        assert_eq!(num, -301, "-345 + 44 should = -301");
    }

}
