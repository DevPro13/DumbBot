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