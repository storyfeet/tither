extern crate lazyf;
extern crate chrono;

use lazyf::{Cfg,SGetter};
use chrono::{Utc};

#[derive(PartialEq,Debug)]
struct Transaction{
    date:Utc,
    currency:String,
    amount:i32,
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
        let mut dt = Utc::today(); 

        match ss.chars().next(){
            Some('=')=>{
            },
            Some('#')|None=>{return NoAction},
            _=>{},
        }
        let mut is_tithe = false;
        for s in ss.split(",").map(|x|x.trim()){
            if s.len() == 0 {
                continue;
            }


        }
        Trans({
            date:date,
            currency:"GBP".to_string,
            
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
    
    #[test]
    fn test_from_str(){
        assert_eq!(Action::from_str("#hello"),NoAction);
        //assert_eq!(Action::fro
    }

}
