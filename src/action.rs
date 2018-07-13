use money::Money;
use chrono::{NaiveDate};
use std::str::FromStr;

#[derive(PartialEq,Debug,Clone)]
pub struct Transaction{
    date:NaiveDate,
    pub amount:Money,
    items:Vec<String>, 
}

impl Transaction {
    pub fn is_tithe(&self)->bool{
        self.items.iter().find(|x| x.starts_with("tithe")) != None 
    }
    pub fn has_tag(&self,t:&str)->bool{
        self.items.iter().find(|x| *x == t) != None
    }

    pub fn has_a_tag(&self,tags:&[String])->bool
    {
        for t in tags{
            if self.has_tag(&t){
                return true;
            }
        }
        false
    }
}


#[derive(PartialEq,Debug,Clone)]
pub enum Action{
    Trans(Transaction),
    SetCurr(String),
    SetTithe(i32),//as percent
    NoAction,
}



impl Action {

    pub fn from_str(ss:&str)->Action{
        use self::Action::*;
        let ss = ss.trim();

        match ss.chars().next(){
            Some('=')=>{
                if ss.starts_with("=curr,"){
                    return SetCurr(ss.trim_left_matches("=curr,").trim().to_string());
                }
                if ss.starts_with("=tithe,"){
                    let ps = ss.trim_left_matches("=tithe,").trim();
                    return match ps.parse::<i32>(){
                        Ok(n)=>SetTithe(n),
                        Err(_)=>NoAction,
                    }
                }
            },
            Some('#')|Some('!')|None=>{return NoAction},
            _=>{},
        }
        let mut res_date = NaiveDate::from_ymd(1,1,1); 
        let mut res_amount = Money::from(0);
        let mut res_items= Vec::new();

        for s in ss.split(",").map(|x|x.trim()){
            match s.chars().next(){
                Some('#')|None => continue,
                _=> {},
            }

            match NaiveDate::parse_from_str(s,"%d/%m/%y"){
                Ok(dparse)=>{
                    res_date = dparse;
                    continue;
                },
                Err(_) =>{},
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
