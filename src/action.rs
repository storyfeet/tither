use crate::error::ParseError;
use crate::money::Money;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub struct DMoY {
    d: i32,
    m: i32,
    y: Option<i32>,
}

impl FromStr for DMoY {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s
            .split("/")
            .map(|s| s.trim().parse::<i32>().map_err(|_| ParseError::DateError));
        let d = ss.next().unwrap()?;
        let m = ss.next().ok_or(ParseError::DateError)??;
        let y = match ss.next() {
            Some(Ok(n)) => Some(n),
            Some(Err(_)) => return Err(ParseError::DateError),
            None => None,
        };
        Ok(DMoY { d, m, y })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParsedTransaction {
    line: usize,
    date: Option<DMoY>,
    pub amount: Money,
    items: Vec<String>,
}

impl ParsedTransaction {
    pub fn is_tithe(&self) -> bool {
        self.items.iter().find(|x| x.starts_with("tithe")) != None
    }
    pub fn has_tag<T: AsRef<str>>(&self, t: T) -> bool {
        self.items.iter().find(|x| *x == t.as_ref()) != None
    }

    pub fn has_a_tag<T: Iterator<Item = S>, S: AsRef<str>>(&self, tags: T) -> bool {
        for t in tags {
            if self.has_tag(t) {
                return true;
            }
        }
        false
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    Trans(ParsedTransaction),
    SetCurr(String),
    SetTithe(i32), //as percent
    SetYear(i32),
    NoAction,
}

impl Action {
    pub fn has_a_tag_or<S: AsRef<str>, T: Iterator<Item = S>>(&self, it: T) -> bool {
        match self {
            Action::Trans(pt) => pt.has_a_tag(it),
            _ => true,
        }
    }

    pub fn from_line(linenum: usize, ss: &str) -> Result<Action, ParseError> {
        use self::Action::*;
        let ss = ss.trim();

        match ss.chars().next() {
            Some('=') => {
                if ss.starts_with("=curr,") {
                    return Ok(SetCurr(ss.trim_start_matches("=curr,").trim().to_string()));
                }
                if ss.starts_with("=tithe,") {
                    return ss
                        .trim_start_matches("=tithe,")
                        .trim()
                        .parse::<i32>()
                        .map(|v| SetTithe(v))
                        .map_err(|_| ParseError::TitheNotSet);
                }
                if ss.starts_with("=year,") {
                    return ss
                        .trim_start_matches("=year,")
                        .trim()
                        .parse::<i32>()
                        .map(|v| SetYear(v))
                        .map_err(|_| ParseError::YearNotSet);
                }
            }
            Some('#') | Some('!') | None => return Ok(NoAction),
            _ => {}
        }
        let mut res_date = None;
        let mut res_amount = Money::from(0);
        let mut res_items = Vec::new();

        for s in ss.split(",").map(|x| x.trim()) {
            match s.chars().next() {
                Some('#') | None => continue,
                _ => {}
            }

            match DMoY::from_str(s) {
                Ok(dparse) => {
                    res_date = Some(dparse);
                    continue;
                }
                Err(_) => {}
            }

            match Money::from_str(s) {
                Ok(mparse) => {
                    res_amount += mparse;
                    continue;
                }
                _ => {}
            }
            res_items.push(s.to_string());
        }
        Ok(Trans(ParsedTransaction {
            line: linenum,
            date: res_date,
            amount: res_amount,
            items: res_items,
        }))
    }
}
