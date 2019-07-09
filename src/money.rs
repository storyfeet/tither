use std::ops::{Add, AddAssign, Div, Mul, Sub};

use std::str::FromStr;
//use std::num::ParseIntError;
use std::fmt;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct Money(i32);

impl From<i32> for Money {
    fn from(n: i32) -> Money {
        Money(n)
    }
}

impl FromStr for Money {
    type Err = String;
    fn from_str(s: &str) -> Result<Money, Self::Err> {
        let v: Vec<&str> = s.trim().split(".").collect();
        let pds = match v[0].parse::<i32>() {
            Ok(n) => n * 100,
            Err(_) => return Err("No convert".to_string()),
        };
        if v.len() == 1 {
            return Ok(Money(pds));
        }

        let mut ch = v[1].chars();
        let mut pns = 0;

        for _ in 0..2 {
            pns *= 10;
            match ch.next() {
                Some(n) => {
                    if n < '0' || n > '9' {
                        return Err("no convert".to_string());
                    }
                    pns += (n as i32) - 48
                }
                _ => {
                    break;
                }
            }
        }
        match pds >= 0 {
            true => Ok(Money(pds + pns)),
            false => Ok(Money(pds - pns)),
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (n, minus) = match self.0 >= 0 {
            true => (self.0, "".to_string()),
            false => (-self.0, "-".to_string()),
        };
        let pnds = n / 100;
        let pns = n % 100;
        if pns < 10 {
            return write!(f, "{}{}.0{}", minus, pnds, pns);
        }
        write!(f, "{}{}.{}", minus, pnds, pns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_parse_money() {
        assert_eq!("4".parse::<Money>(), Ok(Money::from(400)));
        assert_eq!("-4.5".parse::<Money>(), Ok(Money::from(-450)));
        assert_eq!("-4.0005".parse::<Money>(), Ok(Money::from(-400)));
        assert!("670.tp".parse::<Money>().is_err());
        assert!("tp.5".parse::<Money>().is_err());
    }

    #[test]
    fn t_print_money() {
        assert_eq!(&Money::from(400).to_string(), "4.00");
        assert_eq!(&Money::from(41).to_string(), "0.41");
        assert_eq!(&Money::from(-420).to_string(), "-4.20");
        assert_eq!(&Money::from(4000).to_string(), "40.00");
        assert_eq!(&Money::from(-20).to_string(), "-0.20");
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, other: Money) -> Self {
        Money(self.0 + other.0)
    }
}
impl Sub for Money {
    type Output = Self;
    fn sub(self, other: Money) -> Self {
        Money(self.0 - other.0)
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Div<i32> for Money {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        Money(self.0 / rhs)
    }
}

impl Mul<i32> for Money {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Money(self.0 * rhs)
    }
}
