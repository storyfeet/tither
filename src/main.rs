extern crate lazyf;
extern crate chrono;

use lazyf::{Cfg,SGetter};
use chrono::{Utc,Date,TimeZone};


mod money;
use money::Money;

#[derive(PartialEq,Debug)]
struct Transaction{
    date:Date<Utc>,
    currency:String,
    amount:Money,
    source:String, 
}


#[derive(PartialEq,Debug)]
enum Action{
    Trans(Transaction),
    Tithe(Transaction),
    SetCurr(String),
    SetTithe(i32),
    NoAction,
}


impl Action {

    pub fn from_str(ss:&str)->Action{
        use Action::*;
        let ss = ss.trim();

        match ss.chars().next(){
            Some('=')=>{
            },
            Some('#')|None=>{return NoAction},
            _=>{},
        }
        let mut res_dt = Utc::today(); 
        let mut res_am = 0;
        let mut res_items = "".to_string();
        let mut is_tithe = false;

        for s in ss.split(",").map(|x|x.trim()){
            if s.len() == 0 {
                continue;
            }
            if s.chars().next() == Some('#') {
                continue;
            }
            match Utc.datetime_from_str(s,"&d/&m/&Y"){
                Ok(dparse)=>{
                    res_dt = dparse.date();
                    continue;
                },
                _=>{},
            }

            
            


        }
        Trans(Transaction{
            date:res_dt,
            currency:"GBP".to_string(),
            amount:Money::from(0),
            source:"".to_string(),
            
        })
    }
}


fn main() {
    let cfg = Cfg::load_first("conf",&["{HOME}/.config/tither/init"]);

    let f = cfg.get_s_def(("-f","config.flag"),"none");
    println!("Hello, {}!",f);
}


#[cfg(test)]
mod tests{
    use super::*;
    use super::Action::*;
    
    #[test]
    fn test_from_str(){
        assert_eq!(Action::from_str("#hello"),NoAction);
        //assert_eq!(Action::fro
    }

}
