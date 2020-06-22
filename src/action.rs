use crate::money::{Money, PMoney};
use gobble::*;

#[derive(PartialEq, Debug, Clone)]
pub struct DMoY {
    d: usize,
    m: usize,
    y: Option<isize>,
}

parser! {
    (PDate->DMoY),
    (CommonUInt,last('/',CommonUInt),maybe(last('/',CommonInt)))
        .map(|(d,m,y)|DMoY{d,m,y})
}

impl Transaction {
    pub fn is_tithe(&self) -> bool {
        self.tithe
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
pub struct LineAction {
    pub a: Action,
    pub l: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    Trans(Transaction),
    SetCurr(String),
    SetTithe(isize), //as percent
    SetYear(usize),
    SetDate(DMoY),
}

pub fn setter(s: &'static str) -> impl Parser<Out = &'static str> {
    or!(middle("=", s, ":,".one()), first(s, "="))
}

parser! {
    (PFile->Vec<LineAction>),
    first(rep((NextAction,PAction)),(NextAction,eoi))
        .map(|v|v.into_iter().map(|(l,a)|LineAction{l,a}).collect())

}

parser! {
    (PAction->Action),
    or!(
        (setter("tithe"),CommonInt,maybe("%")).map(|(_,n,_)|Action::SetTithe(n)),
        (setter("year"),CommonUInt).map(|(_,n)|Action::SetYear(n)),
        (setter("curr"),Alpha.star()).map(|(_,s)|Action::SetCurr(s)),
        PDate.map(|d| Action::SetDate(d)),
        PTransaction.map(|t|Action::Trans(t))
    )
}

#[derive(PartialEq, Debug, Clone)]
pub struct Transaction {
    pub amount: Money,
    pub tithe: bool,
    pub items: Vec<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TransactionItem {
    Amount(Money),
    Tithe,
    Item(String),
}

parser! {
    (NextAction->usize)
    skip_2_star("\t ,\n\r".skip_plus(),last("#!".one(),Any.except(",\n").skip_star())).ig_then(line_col).map(|(l,_)|l)
}

parser! {
    (NextItem->usize)
    skip_2_star("\t ,".skip_plus(),last('#',Any.except(",\n").skip_star())).ig_then(line_col).map(|(l,_)|l)
}

parser! {
    (IString->String),
    string((Alpha.plus(),sep((Alpha,NumDigit,"-_").plus(),WS.plus())))
}

parser! {
    (PTranItem -> TransactionItem),
    or!(
        PMoney.map(|m|TransactionItem::Amount(m)),
        ("tithe",Any.except(",\n").star()).map(|_|TransactionItem::Tithe),
        or(CommonStr,IString).map(|s|TransactionItem::Item(s))
    )

}

parser! {
    (PTransaction -> Transaction),
    repeat_until_ig(first(PTranItem,NextItem),'\n').map(|v| {
        let mut amount = Money::from(0);
        let mut items = Vec::new();
        let mut is_tithe = false;
        for i in v{
            match i{
                TransactionItem::Amount(a)=> amount += a,
                TransactionItem::Item(s)=> items.push(s),
                TransactionItem::Tithe=> is_tithe = true,
            }
        }
        Transaction{amount,items,tithe:is_tithe}
    })
}
