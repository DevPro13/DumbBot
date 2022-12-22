use std::collections::{
    HashMap,
    HashSet,
};
use super::choosetrump::Trump;
#[derive(Default)]
pub struct Knowledge{
    //this will give the knowledge of played and un-played cards of respective suits
    //each variable represent a suit of 1 byte
    //MSB of each byte represents card of JAck of respective suit and LSB represents & card of 9 of respective suit
    //if any bit flag 0, it represent that card is played.. if 1, it is not played card
    //initially all bits are set.. means all cards are not played..
    H:u8,//for cards of Heart suit
    C:u8,//for cards of Club suit
    D:u8,//for cards of Diamond suit
    S:u8,//for cards of Spades suit
}
impl Knowledge{
    pub fn init(&mut self)->Knowledge{
        Knowledge{
            //initially all cards are not played
            H:255,
            C:255,
            D:255,
            S:255,
        }
    }
    fn card_mapto_bitpos(&self,card:char)->u8{
        match card{
            'J'=>128,
            '9'=>64,
            '1'=>32,
            'T'=>16,
            'K'=>8,
            'Q'=>4,
            '8'=>2,
            '7'=>1,
             _ => {
                    println!("Not played card");
                    0
                },
        }
    }
    pub fn update_knowledge(&mut self,cards:&Vec<String>){
        for card in cards{
            let suit=card.as_bytes()[1] as char;
            match suit {
                'H'=>self.H^=self.card_mapto_bitpos(card.as_bytes()[0] as char),
                'C'=>self.C^=self.card_mapto_bitpos(card.as_bytes()[0] as char),
                'D'=>self.D^=self.card_mapto_bitpos(card.as_bytes()[0] as char),
                'S'=>self.S^=self.card_mapto_bitpos(card.as_bytes()[0] as char),
                _=>println!("No matched suit"),
            }
        }
    }
    pub fn check_played_card(&self,card:String)->bool{
        //this funtion takes a card eg "JS" as input and tell it is played or not
        let suit=card.as_bytes()[1] as char;
        match suit {
            'H'=>{
                if (self.H & self.card_mapto_bitpos(card.as_bytes()[0] as char))!=0{
                    return true;
                }
                false
            },
            'C'=>{
                if (self.C & self.card_mapto_bitpos(card.as_bytes()[0] as char))!=0{
                    return true;
                }
                false
            },
            'D'=>{
                if (self.D & self.card_mapto_bitpos(card.as_bytes()[0] as char))!=0{
                    return true;
                }
                false
            },
            'S'=>{
                if (self.S & self.card_mapto_bitpos(card.as_bytes()[0] as char))!=0{
                    return true;
                }
                false
            },
            _=>false,
        }
    }
}
/*pub mod make_optimal_move{
const cards=HashMap::from([
                //each suit cards ranks and points
                'J':(1,3),
                '9':(2,2),
                '1':(3,1),
                'T':(4,1)
                'K':(5,0),
                'Q':(6,0),
                '8':(7,0),
                '7':(8,0),
    ]);
    fn give_sum_of_points(board)->u8{
        0

    }
}

fn make_first_move(cards:&Vec<String>,knowledge:&Knowledge){
    let mut trump=Trump::init_trump_count(&mut moduleinrust::Trump::default());
    if *cards.len()>1{
        trump.count_suits(&cards);
    }
    //initially try to get points
    //throw 9 if you have J also
    if *knowledge.check_played_card("JS".to_string()) || *cards.contains(&"JS".to_string()) && *cards.contains(&"9S".to_string()) && trump_not_revealed{
        return "9S".to_string();
    }
    if *cards.contains(&"JD".to_string()) && *cards.contains(&"9D".to_string()) && trump_not_revealed{
        return "9D".to_string();
    }
    if *cards.contains(&"JH".to_string()) && *cards.contains(&"9H".to_string()) && trump_not_revealed{
        return "9H".to_string();
    }
    if *cards.contains(&"JC".to_string()) && *cards.contains(&"9C".to_string()) && trump_not_revealed{
        return "9C".to_string();
    }
    //throw J
    if *cards.contains(&"JS".to_string() && trump_not_revealed){
        return "JS".to_string();
    }
    if *cards.contains(&"JC".to_string() && trump_not_revealed){
        return "JC".to_string();
    }
    if *cards.contains(&"JH".to_string() && trump_not_revealed){
        return "JH".to_string();
    }
    if *cards.contains(&"JD".to_string() && trump_not_revealed){
        return "JD".to_string();
    }
   //try to remove low suit cards if you are bid winner 

    //throw 9 card if J already played and trump

    if *knowledge.check_played_card("JS".to_string()) && trump_not_revealed{
        return "9S"
    }
    //try to remove card with low suits
    

    //if trump revealed

}
fn make_second_move(){
    //see if the cards has high rank
    //see if you have the high rank card
    //
}
*/