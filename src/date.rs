
use std::str::FromStr;
use chrono::{Utc,Date,TimeZone};
use std::iter::Iterator;

fn iter_s_to_n<'a,T,S>(it:& mut S)->Result<T,String>
    where S :Iterator<Item=&'a str>,
          T : FromStr,
{
    match it.next() {
        Some(s)=>match s.parse(){
            Ok(n)=>Ok(n),
            Err(e)=>Err("No Parse: ".to_string()),
        }
        None=>Err("not enough elems".to_string()),
    }
}

pub fn date_from_str(s:&str)->Result<Date<Utc>,String>{
    
    let mut ss = s.split("/").map(|x| x.trim()); 

    let d = match iter_s_to_n(&mut ss){
        Ok(n)=>n,
        Err(s)=>{return Err(s)},
    };

    let m = match iter_s_to_n(&mut ss){
        Ok(n)=>n,
        Err(s)=>{return Err(s)},
    };
    
    let y:i32 = match iter_s_to_n(&mut ss){
        Ok(n)=>n,
        Err(s)=>{return Err(s)},
    };

    Ok(Utc.ymd(y + 2000 ,m,d))
}
