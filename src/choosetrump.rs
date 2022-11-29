use std::collections::HashMap;
#[derive(Clone,Default)]
pub struct Trump{
    suits:HashMap<String,u8>,
}

impl Trump{
 pub fn init_trump_count(&mut self)->Trump{
 Trump{ suits:HashMap::from([
        ("D".to_string(),0),
        ("H".to_string(),0),
        ("S".to_string(),0),
        ("C".to_string(),0),
     ]),
    }
}
    pub fn countsuits(&mut self,cards:&Vec<String>){
        for  i in cards{
            let k=i.as_bytes()[1] as char;
            self.suits.insert(k.to_string(),self.suits[&k.to_string()]+1);
        }
    }
    pub fn ret_counted_suits(&self)->HashMap<String,u8>{
        self.suits.clone()
    }
    pub fn display(&self){
    println!("{:?}",self.suits);
    }
pub fn gettrumpsuit(&self)->String{
    let mut cards_with_only_one_suits:Vec<String>=Vec::new();
    for key in self.suits.keys(){
        if self.suits[key]==1{
        cards_with_only_one_suits.push(key.to_string());
        }
        if self.suits[key]>=2{
        return (&key).to_string();
        }
    }
    cards_with_only_one_suits[0].clone()
 }

}

/*
implementation


use std::collections::HashMap;
mod moduleinrust;
use self::moduleinrust::*;
fn main(){
let mut trump=Trump::init_trump_count(&mut moduleinrust::Trump::default());
let cards:Vec<String>=vec!["1H".to_string(),"JS".to_string(),"8C".to_string(),"9D".to_string(),];
trump.display();
trump.countsuits(&cards);
trump.display();
println!("{}",trump.gettrumpsuit());
let counted_suits:HashMap<String,u8>=trump.ret_counted_suits();
println!("{:?}",counted_suits);
}


 */