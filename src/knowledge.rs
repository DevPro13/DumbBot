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
pub struct MyCARDS{
    pub H:Vec<u8>,
    pub C:Vec<u8>,
    pub D:Vec<u8>,
    pub S:Vec<u8>,
}
fn card_mapto_bitpos(card:char)->u8{
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
    pub fn update_knowledge(&mut self,cards:&Vec<String>){
        for card in cards{
            let suit=card.as_bytes()[1] as char;
            match suit {
                'H'=>self.H^=card_mapto_bitpos(card.as_bytes()[0] as char),
                'C'=>self.C^=card_mapto_bitpos(card.as_bytes()[0] as char),
                'D'=>self.D^=card_mapto_bitpos(card.as_bytes()[0] as char),
                'S'=>self.S^=card_mapto_bitpos(card.as_bytes()[0] as char),
                _=>println!("No matched suit"),
            }
        }
    }
    pub fn check_played_card(&self,key:u8,suit:char)->bool{
        //this funtion takes a key eg "128" and suit 'H' as input and tell it is played or not
        
        match suit {
            'H'=>{
                if (self.H & key!=0){
                    return true;
                }
                false
            },
            'C'=>{
                if (self.C & key!=0){
                    return true;
                }
                false
            },
            'D'=>{
                if (self.D & key!=0){
                    return true;
                }
                false
            },
            'S'=>{
                if (self.S & key!=0){
                    return true;
                }
                false
            },
            _=>false,
        }
    }
        pub fn no_card_greater_than_this_rank_card(&self,key:u8,suit:char)->bool{
            fn check(val:u8,key:u8)->bool{
                if key==128{
                    return false;
                }
                if key==64{
                    return val&128==128;
                }
                if key==32{
                    return (val&128==128||val&64==64);
                }
                if key==16{
                    return (val&128==128||val&64==64||val&32==32) ;
                }   
                if key==8{
                    return(val&128==128||val&64==64||val&32==32||val&16==16);
                }
                if key==4{
                    return (val&128==128||val&64==64||val&32==32||val&16==16||val&8==8);
                }
                false//default
            }
            match suit{
                'H'=>check(self.H,key),
                'C'=>check(self.C,key),
                'D'=>check(self.D,key),
                'S'=>check(self.S,key),
                _=>false,
            }
        }
        fn get_total_cards_not_played(&self,suit:char)->u8{
            let mut count:u8 =0;
           let cards:Vec<u8>=vec![128,64,32,16,8,4,2,1];
           for i in cards.iter(){
            if self.check_played_card(*i, suit){
                count+=1;
            }
           }
           count
        }
        pub fn no_possibility_of_trump_reveal(&self,suit:char,this_suit_my_total_cards:u8)->bool{
            fn check(suit:char,this_suit_my_total_cards: u8)->bool{
                if ((self.get_total_cards_not_played(suit)/8-(this_suit_my_total_cards/8))>0.25){
                    return true;
                }
                false
            }
            match suit{
                'H'=>check(suit,this_suit_my_total_cards),
                'C'=>check(suit,this_suit_my_total_cards),
                'D'=>check(suit,this_suit_my_total_cards),
                'S'=>check(suit,this_suit_my_total_cards),
                _=>false,
            }
        }
}
impl MyCARDS{
    fn init(&mut self)->MyCARDS{
        MyCARDS { H:vec![], C: vec![], D:vec![], S:vec![] }
    }
    pub fn update_my_cards(&mut self,cards:&Vec<String>){
        fn mapsuit(suit:char,card:char){
            match suit{
                'H'=>self.H.push(card_mapto_bitpos(card)),
                'D'=>self.D.push(card_mapto_bitpos(card)),
                'C'=>self.C.push(card_mapto_bitpos(card)),
                'S'=>self.S.push(card_mapto_bitpos(card)),
                _=>0
            }
        }
        for card in cards{
            mapsuit(card.as_bytes()[1] as char,card.as_bytes()[0] as char);
        }
        self.H.sort_by(|a, b| b.cmp(a));
        self.C.sort_by(|a, b| b.cmp(a));
        self.D.sort_by(|a, b| b.cmp(a));
        self.S.sort_by(|a, b| b.cmp(a));
    }
    pub fn map_key_to_card(&self,key:u8,suit:char)->String{
        match key{
            128=>format!("{}{}",'J',suit),
            64=>format!("{}{}",'9',suit),
            32=>format!("{}{}",'1',suit),
            16=>format!("{}{}",'T',suit),
            8=>format!("{}{}",'K',suit),
            4=>format!("{}{}",'Q',suit),
            2=>format!("{}{}",'8',suit),
            1=>format!("{}{}",'7',suit),
            _ =>{
                    println!("Not played card");
                    "X".to_string()
                },
            }
        }
    }
    /*fn get_card(suit:char)->String{
        match suit{
            'H'=>{
                let key=self.
            }
            'H'=>
            'H'=>
            'H'=>


        }



    }
    fn get_high_rank_card(suit:char)->String{
        

    }
}*/