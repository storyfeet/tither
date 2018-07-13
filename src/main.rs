extern crate lazyf;
extern crate chrono;


use lazyf::{Cfg,SGetter};

use std::fs::File;
use std::io::{BufReader,BufRead};


mod money;
use money::Money;

mod action;
use action::{Action};
use Action::*;

use std::collections::HashMap;


struct Tracker{
    t_paid:Money,
    t_req:Money,
    income:Money,
    outgoing:Money, // Should be negative
}

impl Tracker{
    fn new()->Tracker{
        Tracker{
            t_paid:Money::from(0),
            t_req:Money::from(0),
            income:Money::from(0),
            outgoing:Money::from(0),
        }
        
    }
}

fn get_tracker<'a>(mp:&'a mut HashMap<String,Tracker>,curr:&str)->&'a mut  Tracker{
    
    let has_item = match mp.get(curr){
        Some(_)=>true,
        None=>false,
    };
    if !has_item {
        mp.insert(curr.to_string(),Tracker::new());
    }

    mp.get_mut(curr).unwrap()
    
}

fn filter_tags(aa:&[Action],tags:&[String])->Vec<Action>
{
    let mut res = Vec::new();
    
    for a in aa {
        match a {
            &Trans(ref t)=>if t.has_a_tag(tags){
                res.push(Trans((*t).clone()));
            },
            _=>res.push((*a).clone()),
        }
    }
    return res;
}


fn count_tithe(aa:&[Action])
{

    let mut trackers = HashMap::new();
    
    let mut curr = "GBP".to_string();
    

    let mut tithe_pc = 10;
    for a in aa {
        match a{
            &Trans(ref t)=>{
                let c_tracker = get_tracker(&mut trackers,&curr);
                if t.is_tithe(){
                    c_tracker.t_paid += t.amount;
                    
                } else{
                    if t.amount > Money::from(0) {
                        c_tracker.income += t.amount
                    }else {
                        c_tracker.outgoing += t.amount 
                    }
                    c_tracker.t_req += (t.amount * tithe_pc)/100;
                }
            },
            &SetTithe(n)=>{
                tithe_pc = n;
            }
            &SetCurr(ref c)=>{
                curr = c.to_string();
            }
            _=>{},
        }
    }

    for (s,v) in trackers {
        print!("{}\n",s);
        print!("income = {},outgoing = {}, net= {}\n",v.income,v.outgoing,v.income+v.outgoing);
        print!("tithe: req = {}, paid = {}, owed = {}\n",v.t_req,v.t_paid,v.t_req - v.t_paid);
    }
    
}



fn main() {
    let cfg = Cfg::load_first("conf",&["{HOME}/.config/tither/init"]);

    let fname = cfg.get_s(("-f","config.filename")).expect("No Filename given");
    let tags = cfg.get_s(("-t","config.tags"));

    let f = File::open(fname).expect("Could not read file");
    let f = BufReader::new(f);

    let mut v= Vec::new();
    for line in f.lines(){
        let a = Action::from_str(&(line.unwrap()));
        v.push(a);
        //print!("{:?}\n",a);
    }

    if let Some(t) = tags {
        let tgs:Vec<String> = t.split(",").map(|x|x.to_string()).collect();
        print!("Tags = {:?}\n",tgs);
        v = filter_tags(&v,&tgs);
    }

    count_tithe(&v);

}

#[cfg(test)]
mod tests{
    use super::*;
    use super::action::Action::*;
    
    #[test]
    fn test_from_str(){
        assert_eq!(Action::from_str("#hello"),NoAction);
        //assert_eq!(Action::fro
    }

}
