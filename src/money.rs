use std::ops::{Add, AddAssign, Div, Mul, Sub};

use std::str::FromStr;
//use std::num::ParseIntError;
use std::fmt;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct Money(isize);

impl From<isize> for Money {
    fn from(n: isize) -> Money {
        Money(n)
    }
}

use bogobble::*;

parser! {
    (PMoney->Money),
    (maybe("$£".one()),common::Int,maybe(('.',NumDigit.star()))).map(|(_,n,op)| match op{
        None=>Money(n*100),
        Some((_,ds))=> {
            let mut res = n * 100;
            let sig = match n.signum() { 0=>1, v=>v};
            for (p,c) in (0..2).zip(ds.chars()){
                res+= sig * (c as isize - '0' as isize) * (1 + 9* (1-p))
            }
            Money(res)
        }
    })
}

impl FromStr for Money {
    type Err = String;
    fn from_str(s: &str) -> Result<Money, Self::Err> {
        PMoney.parse_s(s).map_err(|e| e.to_string())
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
        //TODO decide if error assert!("670.tp".parse::<Money>().is_err());
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

impl Div<isize> for Money {
    type Output = Self;
    fn div(self, rhs: isize) -> Self {
        Money(self.0 / rhs)
    }
}

impl Mul<isize> for Money {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        Money(self.0 * rhs)
    }
}
