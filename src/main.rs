extern crate failure_derive;
use bogobble::*;

//use lazy_conf::config;

//use error::LineError;
//mod transaction;

mod money;
use crate::money::Money;

mod action;
use crate::action::{Action, LineAction};

use std::collections::HashMap;

use clap_conf::prelude::*;

struct Tracker {
    t_paid: Money,
    t_req: Money,
    income: Money,
    outgoing: Money, // Should be negative
}

impl Tracker {
    fn new() -> Tracker {
        Tracker {
            t_paid: Money::from(0),
            t_req: Money::from(0),
            income: Money::from(0),
            outgoing: Money::from(0),
        }
    }
}

fn get_tracker<'a>(mp: &'a mut HashMap<String, Tracker>, curr: &str) -> &'a mut Tracker {
    let has_item = match mp.get(curr) {
        Some(_) => true,
        None => false,
    };
    if !has_item {
        mp.insert(curr.to_string(), Tracker::new());
    }

    mp.get_mut(curr).unwrap()
}

fn count_tithe(aa: &[LineAction]) {
    let mut trackers = HashMap::new();

    let mut curr = "GBP".to_string();

    let mut tithe_pc = 10;
    for a in aa {
        match &a.a {
            &Action::Trans(ref t) => {
                let c_tracker = get_tracker(&mut trackers, &curr);
                if t.is_tithe() {
                    c_tracker.t_paid += t.amount;
                } else {
                    if t.amount > Money::from(0) {
                        c_tracker.income += t.amount
                    } else {
                        c_tracker.outgoing += t.amount
                    }
                    c_tracker.t_req += (t.amount * tithe_pc) / 100;
                }
            }
            &Action::SetTithe(n) => {
                tithe_pc = n as isize;
            }
            &Action::SetCurr(ref c) => {
                curr = c.to_string();
            }
            _ => {}
        }
    }

    for (s, v) in trackers {
        print!("{}\n", s);
        print!(
            "income = {},outgoing = {}, net= {}\n",
            v.income,
            v.outgoing,
            v.income + v.outgoing
        );
        print!(
            "tithe: req = {}, paid = {}, owed = {}\n",
            v.t_req,
            v.t_paid,
            v.t_req - v.t_paid
        );
    }
}

fn main() -> Result<(), failure::Error> {
    let clap = clap_app!(Tither =>
                    (about:"A program to calculate tithe due")
                    (author:"Matt Stoodley")
                    (version:crate_version!())
                    (@arg filename:-f --filename +takes_value "filename to work on")
                    (@arg tags:-t --tags +takes_value ... "Tags to search for")
    )
    .get_matches();

    let cfg = with_toml_env(&clap, &["{HOME}/.config/tither/init"]);

    let fname = cfg.grab().arg("filename").conf("config.filename").req()?;

    let tags = clap.values_of("tags");

    let fd = std::fs::read_to_string(fname)?;

    let mut line_acs = action::PFile.parse_s(&fd).map_err(|e| e.strung())?;

    if let Some(t) = tags {
        let tt: Vec<String> = t.map(|v| v.to_string()).collect();
        print!("Tags = {:?}\n", tt);
        line_acs = line_acs
            .into_iter()
            .filter(|la| match &la.a {
                Action::Trans(t) => t.has_a_tag(tt.iter()),
                _ => true,
            })
            .collect();
    }

    count_tithe(&line_acs);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::action::Action::*;
    use super::*;
}
