use std::str::FromStr;
use std::num::ParseIntError;


#[derive(PartialEq,Copy,Clone,Debug)]
pub struct Money(i32);

impl From<i32>for Money{    
    fn from(n:i32)->Money{
        Money(n)
    }
}

impl FromStr for Money{
    type Err = ParseIntError;
    fn from_str(s:&str)->Result<Money,Self::Err>{
        let v:Vec<&str> = s.trim().split(".").collect();
        let pds = v[0].parse::<i32>()?*100;
        if v.len() == 1 {
            return Ok(Money(pds));
        }
         
        let mut pns = v[1].parse::<u32>()?;
        while pns >= 100 {
            pns = pns/10;
        }
        return Ok(Money(pds+pns as i32));

    }
}


#[cfg(test)]
mod tests{
    use money::*;
    #[test]
    fn t_parse_money(){
        assert_eq!("4".parse::<Money>(),Ok(Money::from(400)));
        assert_eq!("-4.5".parse::<Money>(),Ok(Money::from(-450)));
    }
}




