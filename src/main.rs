extern crate lazyf;
extern crate chrono;

use lazyf::{Cfg,SGetter};
use chrono::{Utc,Date,TimeZone};

use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader,BufRead};


mod money;
use money::Money;

#[derive(PartialEq,Debug)]
struct Transaction{
    date:Date<Utc>,
    amount:Money,
    items:Vec<String>, 
}


#[derive(PartialEq,Debug)]
enum Action{
    Trans(Transaction),
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
                //TODO
            },
            Some('#')|None=>{return NoAction},
            _=>{},
        }
        let mut res_date = Utc::today(); 
        let mut res_amount = Money::from(0);
        let mut res_items= Vec::new();

        for s in ss.split(",").map(|x|x.trim()){
            match s.chars().next(){
                Some('#')|None => continue,
                _=> {},
            }

            match Utc.datetime_from_str(s,"&d/&m/&y"){
                Ok(dparse)=>{
                    res_date = dparse.date();
                    continue;
                },
                _=>{},
            }

            match Money::from_str(s){
                Ok(mparse)=>{
                    res_amount = mparse;
                    continue;
                }
                _=>{},
            }
            res_items.push(s.to_string());
        }
        Trans(Transaction{
            date:res_date,
            amount:res_amount,
            items:res_items,
            
        })
    }
}


fn main() {
    let cfg = Cfg::load_first("conf",&["{HOME}/.config/tither/init"]);

    let fname = cfg.get_s(("-f","config.filename")).expect("No Filename given");

    let f = File::open(fname).expect("Could not read file");
    let f = BufReader::new(f);

    for line in f.lines(){
        
        let a = Action::from_str(&(line.unwrap()));
        print!("{:?}\n",a);
    }



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
