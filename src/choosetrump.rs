use std::collections::HashMap;
use crate::TrumpSuit;
#[derive(Clone,Default)]
pub struct Trump{
    suits:HashMap<char,u8>,
}
impl Trump{
 pub fn init_trump_count(&mut self)->Trump{
 Trump{ suits:HashMap::from([
        ('D',0),
        ('H',0),
        ('S',0),
        ('C',0),
     ]),
    }
}
    pub fn countsuits(&mut self,cards:&Vec<String>){
        for  i in cards{
            let k=i.as_bytes()[1] as char;
            self.suits.insert(k,self.suits[&k]+1);
        }
    }
pub fn gettrumpsuit(&self)->String{
    let mut cards_with_only_one_suits:Vec<char>=Vec::new();
    for key in self.suits.keys(){
        if self.suits[key]==1{
        cards_with_only_one_suits.push(*key);
        }
        if self.suits[key]>=2{
        return key.to_string();
        }
    }
    cards_with_only_one_suits[0].clone().to_string()
 }
 pub fn check_if_cards_has_three_same_suits(&self)->bool{
    for key in self.suits.keys(){
        if self.suits[key]==3{
            return true;
        }
    }
    false
 }
 pub fn check_if_cards_has_two_same_suits(&self)->bool{
    for key in self.suits.keys(){
        if self.suits[key]==2{
            return true;
        }
    }
    false
 }

}
pub fn get_trump_suit(cards:&Vec<String>)->TrumpSuit{
    let mut trump=Trump::init_trump_count(&mut Trump::default());
    trump.countsuits(&cards);
    TrumpSuit{suit:trump.gettrumpsuit(),}
}