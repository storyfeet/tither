extern crate lazyf;

use lazyf::{Cfg,SGetter};
use Chrono::{UTC};

struct Transaction{
    Date:UTC;
    Currency:i32,
    Amount:int,
    
}


enum Action{
    Trans(Transaction),
    Tithe(Trannsaction),
    SetCurr,
    SetTithe,
}



fn main() {
    let cfg = Cfg::load_first("conf",&["{HOME}/.config/tither/init"]);

    let f = cfg.get_s_def(("-f","config.flag"),"none");
    println!("Hello, {}!",f);
}
