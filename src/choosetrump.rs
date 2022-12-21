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
 pub fn check_if_cards_has_three_same_suits(&self)->bool{
    true
 }
 pub fn check_if_cards_has_two_same_suits(&self)->bool{
    true
 }

}
pub fn get_trump_suit(cards:&Vec<String>)->String{
    let mut trump=Trump::init_trump_count(&mut Trump::default());
    trump.countsuits(&cards);
    trump.gettrumpsuit()
}