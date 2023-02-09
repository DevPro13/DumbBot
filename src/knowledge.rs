use std::collections::HashMap;

use crate::play::GameDetails;

#[derive(Default,Debug)]
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
#[derive(Default,Debug)]
pub struct MyCARDS{
    pub H:Vec<u8>,
    pub C:Vec<u8>,
    pub D:Vec<u8>,
    pub S:Vec<u8>,
}
#[derive(Default,Debug)]
pub struct HandsInformation{
    pub hand:HashMap<u8,(char,char)>,
    pub handhistory:Vec<(u8,Vec<String>,u8)>,
    pub H:Vec<u8>,//collect player who are ran out of this suit
    pub C:Vec<u8>,
    pub D:Vec<u8>,
    pub S:Vec<u8>,
}
pub fn card_mapto_key(card:char)->u8{
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
pub fn map_key_to_card(key:u8,suit:char)->String{
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
                'H'=>self.H^=card_mapto_key(card.as_bytes()[0] as char),
                'C'=>self.C^=card_mapto_key(card.as_bytes()[0] as char),
                'D'=>self.D^=card_mapto_key(card.as_bytes()[0] as char),
                'S'=>self.S^=card_mapto_key(card.as_bytes()[0] as char),
                _=>println!("No matched suit"),
            }
        }
    }
    pub fn check_played_card(&self,key:u8,suit:char)->bool{
        //this funtion takes a key eg "128" and suit 'H' as input and tell it is played or not
        //true==played
        //false==not played
        match suit {
            'H'=>{
                if self.H & key==0{
                    return true;
                }
                false
            },
            'C'=>{
                if self.C & key==0{
                    return true;
                }
                false
            },
            'D'=>{
                if self.D & key==0{
                    return true;
                }
                false
            },
            'S'=>{
                if self.S & key==0{
                    return true;
                }
                false
            },
            _=>false,
        }
    }
        pub fn card_greater_than_this_rank_card_exist(&self,key:u8,suit:char)->bool{
            fn check(val:u8,key:u8)->bool{
                //if true... other cards greater than this card exist
                if key==128{
                    return false;
                    //this is the highest rank card
                }
                if key==64{
                    return val&128==128;
                }
                if key==32{
                    return val&128==128||val&64==64;
                }
                if key==16{
                    return val&128==128||val&64==64||val&32==32 ;
                }   
                if key==8{
                    return val&128==128||val&64==64||val&32==32||val&16==16;
                }
                if key==4{
                    return val&128==128||val&64==64||val&32==32||val&16==16||val&8==8;
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
        pub fn get_total_cards_not_played(&self,suit:char)->u8{
            let mut count:u8 =0;
           let cards:Vec<u8>=vec![128,64,32,16,8,4,2,1];
           for i in cards.iter(){
            if !self.check_played_card(*i, suit){
                count+=1;
            }
           }
           count
        }
        pub fn get_cards_except_these_suits(&self,other_cards:&Vec<String>,suits:&Vec<char>)->Vec<String>{
            if suits.len() as u8==0{
                return other_cards.to_owned();
            }
            let mut cards:Vec<String>=Vec::new();
            for card in other_cards.iter(){
                if !suits.contains(&(card.as_bytes()[1] as char)){
                    cards.push(card.to_string())
                }
            }
            cards
        }
        pub fn get_opponent_cards_not_played(&self,mycards:&MyCARDS)->Vec<String>{
            let mut cards:Vec<String>=Vec::new();
            let card_keys:Vec<u8>=vec![128,64,32,16,8,4,2,1];
            //println!("My cards :{:?}",mycards);
            for i in card_keys.iter(){
                if self.get_total_cards_not_played('H')!=0 && !self.check_played_card(*i, 'H'){
                    if !mycards.H.contains(i){
                        //if this card is not played
                        cards.push(map_key_to_card(*i,'H'));
                    }
                }
                if self.get_total_cards_not_played('S')!=0 && !self.check_played_card(*i, 'S'){
                    if !mycards.S.contains(i){
                        //if this card is not played
                        cards.push(map_key_to_card(*i,'S'));
                    }
                }
                if self.get_total_cards_not_played('C')!=0 && !self.check_played_card(*i, 'C'){
                    if !mycards.C.contains(i){
                        //if this card is not played
                        cards.push(map_key_to_card(*i,'C'));
                    }
                }
                if self.get_total_cards_not_played('D')!=0 && !self.check_played_card(*i, 'D') {
                    if !mycards.D.contains(i){
                        //if this card is not played
                        cards.push(map_key_to_card(*i,'D'));
                    }
                }
            
            }
            //println!("opponenet all cards:{:?}",cards);
            cards

        }
        pub fn get_opp_cards_of_this_suit(&self,suit:char,mycards:&MyCARDS)->Vec<String>{
                if self.get_total_cards_not_played(suit)==0{
                    return vec![];
                }
                let all_cards:Vec<String>=self.get_opponent_cards_not_played(&mycards);
                //println!("ALl CARDS:{:?}",all_cards);
                let mut cards_of_this_suit:Vec<String>=Vec::new();
                for i in all_cards.iter(){
                    if i.as_bytes()[1] as char==suit{
                        cards_of_this_suit.push(i.to_string());
                    }
                }
                //println!("opponenet cards except this suit cards:{:?}",cards_except_this_suit_cards);
                cards_of_this_suit
        }
        pub fn get_opponent_cards_except_this_suit_cards(&self,suit:char,mycards:&MyCARDS)->Vec<String>{
                let all_cards:Vec<String>=self.get_opponent_cards_not_played(&mycards);
                let mut cards_except_this_suit_cards:Vec<String>=Vec::new();
                for i in all_cards.iter(){
                    if i.as_bytes()[1] as char!=suit{
                        cards_except_this_suit_cards.push(i.to_string());
                    }
                }
                //println!("opponenet cards except this suit cards:{:?}",cards_except_this_suit_cards);
                cards_except_this_suit_cards
        }
        pub fn no_possibility_of_trump_reveal(&self,suit:char,this_suit_my_total_cards:u8)->bool{
            let value:f32=((self.get_total_cards_not_played(suit)-this_suit_my_total_cards) as f32/8 as f32)-(this_suit_my_total_cards as f32/8 as f32);
            match suit{
                'H'=>{
                    if value>0.0{
                        return true;
                    }
                    return false;
                },
                'C'=>{
                    if value>0.0{
                    return true;
                }
                return false;
            },
                'D'=>{if value>0.0{
                    return true;
                }
                return false;
            },
                'S'=>{
                    if value>0.0{
                    return true;
                }
                return false;
            },
                _=>true,
            }
        }
}
impl MyCARDS{
    pub fn init(&mut self)->MyCARDS{
        MyCARDS { H:vec![], C: vec![], D:vec![], S:vec![] }
    }
    pub fn update_my_cards(&mut self,cards:&Vec<String>){
        for card in cards{
            match card.as_bytes()[1] as char{
                'H'=>self.H.push(card_mapto_key(card.as_bytes()[0] as char)),
                'D'=>self.D.push(card_mapto_key(card.as_bytes()[0] as char)),
                'C'=>self.C.push(card_mapto_key(card.as_bytes()[0] as char)),
                'S'=>self.S.push(card_mapto_key(card.as_bytes()[0] as char)),
                _=>println!("not found!"),
            }
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
    pub fn you_have_the_higher_rank_card(&self,opp_card_key:u8,suit:char)->bool{
        match suit{
            'H'=>self.H[0]>opp_card_key,
            'D'=>self.D[0]>opp_card_key,
            'C'=>self.C[0]>opp_card_key,
            'S'=>self.S[0]>opp_card_key,
            _=>false,
        }
    }
    pub fn total_card_left(&self,suit:char)->u8{
        match suit{
            'H'=>self.H.len() as u8,
            'D'=>self.D.len() as u8,
            'C'=>self.C.len() as u8,
            'S'=>self.S.len() as u8,
            _=>0,
        }
    }
    pub fn get_card(&self,suit:char,to_maximize:bool)->String{
        match suit{
                'H'=>{
                        if to_maximize{
                            return self.map_key_to_card(self.H[0], suit);
                        }
                        self.map_key_to_card(*self.H.last().unwrap(),suit)
                },
                'D'=>{
                    if to_maximize{
                        return self.map_key_to_card(self.D[0], suit);
                    }
                    self.map_key_to_card(*self.D.last().unwrap(),suit)
            },
                'C'=>{
                    if to_maximize{
                        return self.map_key_to_card(self.C[0],suit);
                    }
                    self.map_key_to_card(*self.C.last().unwrap(),suit)
            },
                'S'=>{
                    if to_maximize{
                        return self.map_key_to_card(self.S[0],suit);
                    }
                    self.map_key_to_card(*self.S.last().unwrap(),suit)
            },
                _=>"NULL".to_string(),
        }
    }
    pub fn non_point_card_exist(&self,suit:char)->bool{
        match suit{
            'H'=>*self.H.last().unwrap()<=8,
            'D'=>*self.D.last().unwrap()<=8,
            'C'=>*self.C.last().unwrap()<=8,
            'S'=>*self.S.last().unwrap()<=8,
            _=>false,
    }
}
    pub fn you_have_this_card(&self,key:u8,suit:char)->bool{
        match suit{
            'H'=>self.H.contains(&key),
            'D'=>self.D.contains(&key),
            'C'=>self.C.contains(&key),
            'S'=>self.S.contains(&key),
            _=>false,
        }
    }
    pub fn get_first_card_of_given_suit(&self,suit:char)->u8{
        match suit{
            'H'=>self.H[0],
            'D'=>self.D[0],
            'C'=>self.C[0],
            'S'=>self.S[0],
            _=>0,
        }
    }
    pub fn get_second_card(&self,suit:char)->u8{
        match suit{
            'H'=>{
                if self.H.len() as u8>=2{
                    return self.H[1];
                }
                else{
                    *self.H.last().unwrap()
                }
                },
            'D'=>{
                if self.D.len() as u8>=2{
                    return self.D[1];
                }
                else{
                    *self.D.last().unwrap()
                }
                },
            'C'=>{
                if self.C.len() as u8>=2{
                    return self.C[1];
                }
                else{
                    *self.C.last().unwrap()
                }
                },
            'S'=>{
                if self.S.len() as u8>=2{
                    return self.S[1];
                }
                else{
                    *self.S.last().unwrap()
                }
                },
            _=>0,
        }
    }
    pub fn get_non_point_cards(&self,trump:char)->Vec<String>{
        let mut cards:Vec<String>=Vec::new();
        if self.H.len() as u8!=0 &&trump!='H'{
            for i in self.H.iter(){
                if *i<=8{
                    cards.push(self.map_key_to_card(*i, 'H'));
                }
            }
        }
        if self.D.len() as u8!=0&&trump!='D'{
            for i in self.D.iter(){
                if *i<=8{
                    cards.push(self.map_key_to_card(*i, 'D'));
                }
            }
        }
        if self.C.len() as u8!=0&&trump!='C'{
            for i in self.C.iter(){
                if *i<=8{
                    cards.push(self.map_key_to_card(*i, 'C'));
                }
            }
        }
        if self.S.len() as u8!=0&&trump!='S'{
            for i in self.S.iter(){
                if *i<=8{
                    cards.push(self.map_key_to_card(*i, 'S'));
                }
            }
        }
        cards
    }
    pub fn get_this_hand_suit_cards(&self,suit:char)->Vec<String>{
        let mut cards:Vec<String>=Vec::new();
        if self.H.len() as u8!=0 &&suit=='H'{
            for i in self.H.iter(){
               
                    cards.push(self.map_key_to_card(*i, 'H'));
                
            }
        }
        if self.D.len() as u8!=0&&suit=='D'{
            for i in self.D.iter(){
               
                    cards.push(self.map_key_to_card(*i, 'D'));
                
            }
        }
        if self.C.len() as u8!=0&&suit=='C'{
            for i in self.C.iter(){
               
                    cards.push(self.map_key_to_card(*i, 'C'));
                
            }
        }
        if self.S.len() as u8!=0&&suit=='S'{
            for i in self.S.iter(){
               
                    cards.push(self.map_key_to_card(*i, 'S'));
                
            }
        }
        cards
    }
    pub fn get_point_cards(&self)->Vec<String>{
        let mut cards:Vec<String>=Vec::new();
        if self.H.len() as u8!=0{
            for i in self.H.iter(){
                if *i>8{
                    cards.push(self.map_key_to_card(*i, 'H'));
                }
            }
        }
        if self.D.len() as u8!=0{
            for i in self.D.iter(){
                if *i>8{
                    cards.push(self.map_key_to_card(*i, 'D'));
                }
            }
        }
        if self.C.len() as u8!=0{
            for i in self.C.iter(){
                if *i>8{
                    cards.push(self.map_key_to_card(*i, 'C'));
                }
            }
        }
        if self.S.len() as u8!=0{
            for i in self.S.iter(){
                if *i>8{
                    cards.push(self.map_key_to_card(*i, 'S'));
                }
            }
        }
        cards
    }
    pub fn get_card_left(&self,suit:char)->u8{
        //returns the total card available in this suits
        match suit{
            'H'=>self.H.len()as u8,
            'D'=>self.D.len() as u8,
            'C'=>self.C.len() as u8,
            'S'=>self.S.len() as u8,
            _=>0,
        }
    }
    pub fn tens_ones_and_non_point_cards(&self,trump:char)->Vec<String>{
        let mut cards:Vec<String>=Vec::new();
        if self.H.len() as u8!=0 &&trump!='H'{
            for i in self.H.iter(){
                if *i<=32{
                    cards.push(self.map_key_to_card(*i, 'H'));
                }
            }
        }
        if self.D.len() as u8!=0&&trump!='D'{
            for i in self.D.iter(){
                if *i<=32{
                    cards.push(self.map_key_to_card(*i, 'D'));
                }
            }
        }
        if self.C.len() as u8!=0&&trump!='C'{
            for i in self.C.iter(){
                if *i<=32{
                    cards.push(self.map_key_to_card(*i, 'C'));
                }
            }
        }
        if self.S.len() as u8!=0&&trump!='S'{
            for i in self.S.iter(){
                if *i<=32{
                    cards.push(self.map_key_to_card(*i, 'S'));
                }
            }
        }
        cards
    }
pub fn get_trap_card_tens(&self,)->Vec<String>{
    let mut cards:Vec<String>=Vec::new();
    if self.H.len() as u8!=0{
        for i in self.H.iter(){
            if *i==16{
                cards.push(self.map_key_to_card(*i, 'H'));
            }
        }
    }
    if self.D.len() as u8!=0{
        for i in self.D.iter(){
            if *i==16{
                cards.push(self.map_key_to_card(*i, 'D'));
            }
        }
    }
    if self.C.len() as u8!=0{
        for i in self.C.iter(){
            if *i==16{
                cards.push(self.map_key_to_card(*i, 'C'));
            }
        }
    }
    if self.S.len() as u8!=0{
        for i in self.S.iter(){
            if *i==16{
                cards.push(self.map_key_to_card(*i, 'S'));
            }
        }
    }
    cards
}
pub fn get_non_point_trump(&self,trump:char)->Vec<String>{
    let mut cards:Vec<String>=Vec::new();
    match trump{
        'H'=>{
            for i in self.H.iter(){
                if *i<16{
                    cards.push(self.map_key_to_card(*i, trump).clone())
                }
            }
        },
        'C'=>{
            for i in self.C.iter(){
                if *i<16{
                    cards.push(self.map_key_to_card(*i, trump).clone())
                }
            }
        },
        'S'=>{
            for i in self.S.iter(){
                if *i<16{
                    cards.push(self.map_key_to_card(*i, trump).clone())
                }
            }
        },
        'D'=>{
            for i in self.D.iter(){
                if *i<16{
                    cards.push(self.map_key_to_card(*i, trump).clone())
                }
            }
        },
        _=>(),
    }
    cards
}
pub fn get_cards_except_trump(&self,trump:char)->Vec<String>{
    let mut cards:Vec<String>=Vec::new();
    if self.H.len() as u8!=0&&trump!='H'{
        for i in self.H.iter(){
                cards.push(self.map_key_to_card(*i, 'H'));
            
        }
    }
    if self.D.len() as u8!=0&&trump!='D'{
        for i in self.D.iter(){
                cards.push(self.map_key_to_card(*i, 'D'));
        }
    }
    if self.C.len() as u8!=0&&trump!='C'{
        for i in self.C.iter(){
                cards.push(self.map_key_to_card(*i, 'C'));
        }
    }
    if self.S.len() as u8!=0&&trump!='S'{
        for i in self.S.iter(){
                cards.push(self.map_key_to_card(*i, 'S'));
        }
    }
    cards
}
pub fn get_card_just_greater_than_this(&self,key:u8,trump:char)->String{
    let mut card_key:u8=0;
    if self.H.len() as u8!=0&&trump=='H'{
        for i in self.H.iter(){
            if key<*i{
                card_key=*i;
            }
        }
        if card_key==0{
            card_key=self.H[0];
        }
    }
    if self.D.len() as u8!=0&&trump=='D'{
        for i in self.D.iter(){
            if key<*i{
                card_key=*i;
            }
        }
        if card_key==0{
            card_key=self.D[0];
        }
    }
    if self.C.len() as u8!=0&&trump=='C'{
        for i in self.C.iter(){
            if key<*i{
                card_key=*i;
            }
        }
        if card_key==0{
            card_key=self.C[0];
        }
    }
    if self.S.len() as u8!=0&&trump=='S'{
        for i in self.S.iter(){
            if key<*i{
                card_key=*i;
            }
        }
        if card_key==0{
            card_key=self.S[0];
        }
    }
    return map_key_to_card(card_key,trump).clone();
}
pub fn get_random_cards(&self,trump:char,knowledge:&Knowledge)->Vec<String>{
    let mut cards:Vec<String>=Vec::new();
    if self.H.len() as u8!=0&&trump!='H'{
        if (!knowledge.check_played_card(128,'H')&&!self.you_have_this_card(128, 'H'))||(!knowledge.check_played_card(64,'H')&&!self.you_have_this_card(64, 'H')){
            if !self.you_have_the_higher_rank_card(16,'H'){
                for i in self.H.iter(){
                    cards.push(self.map_key_to_card(*i, 'H'));
        }
    }
}       
}
if self.S.len() as u8!=0&&trump!='S'{
    if (!knowledge.check_played_card(128,'S')&&!self.you_have_this_card(128, 'S'))||(!knowledge.check_played_card(64,'S')&&!self.you_have_this_card(64, 'S')){
        if !self.you_have_the_higher_rank_card(16,'S'){
            for i in self.S.iter(){
                cards.push(self.map_key_to_card(*i, 'S'));
    }
}
}
    
}
if self.D.len() as u8!=0&&trump!='D'{
    if (!knowledge.check_played_card(128,'D')&&!self.you_have_this_card(128, 'D'))||(!knowledge.check_played_card(64,'D')&&!self.you_have_this_card(64, 'D')){
        if !self.you_have_the_higher_rank_card(16,'D'){
            for i in self.D.iter(){
                cards.push(self.map_key_to_card(*i, 'D'));
    }
}
}
    
}
if self.C.len() as u8!=0&&trump!='C'{
    if (!knowledge.check_played_card(128,'C')&&!self.you_have_this_card(128, 'C'))||(!knowledge.check_played_card(64,'C')&&!self.you_have_this_card(64, 'C')){
        if !self.you_have_the_higher_rank_card(16,'C'){
            for i in self.C.iter(){
                cards.push(self.map_key_to_card(*i, 'C'));
    }
}
}
    
}
    cards
}
}
impl HandsInformation{
    pub fn init(&mut self)->HandsInformation{
       HandsInformation { hand:HashMap::new(),
                            handhistory:Vec::new(),
                            H:Vec::new(),
                            C:Vec::new(),
                            D:Vec::new(),
                            S:Vec::new(),
        }

    }
    pub fn update_hands_info(&mut self,hand:u8,played_card:&String,winner_card:&String){
        //hands starts form 0 to 7
        self.hand.insert(hand,(played_card.as_bytes()[1] as char,winner_card.as_bytes()[1] as char));
    }
    pub fn check_any_player_ran_out_of_this_suit(&self,suit:char)->usize{
        //this function checks if any players is ran out of this suit cards and returns hand
        if self.hand.len() as u8==0{
            return 8 as usize;
        }
        let mut total_hands_completed:u8=(self.hand.len()-1) as u8;
        while total_hands_completed!=0{
            if suit==self.hand[&total_hands_completed].0{
                    if self.hand[&total_hands_completed].0!=self.hand[&total_hands_completed].1{
                        return total_hands_completed as usize;
                    }
            }
            total_hands_completed-=1;
        }
        8 as usize//this means... this suit card was not defeated by any trump winning card..
    }
    pub fn update_suits_info_of_players(&mut self,playerid:u8,suit:char){
        //this will list out the players who are ran out of this suit
        match suit{
            'H'=>self.H.push(playerid),
            'D'=>self.D.push(playerid),
            'C'=>self.C.push(playerid),
            'S'=>self.S.push(playerid),
            _=>(),
        }
    }
    pub fn any_player_ran_out_of_this_suit_cards(&self,playerid:u8,suit:char)->bool{
        //this analizes then hand gives the onclution that player is ran out of hand
        match suit{
            'H'=>self.H.contains(&playerid),
            'D'=>self.D.contains(&playerid),
            'C'=>self.C.contains(&playerid),
            'S'=>self.S.contains(&playerid),
            _=>false,
        }
        
    }
    pub fn player_ran_out_of_these_suits(&self,playerid:u8)->Vec<char>{
        let mut suits:Vec<char>=Vec::new();
        if self.H.contains(&playerid){
            suits.push('H');
        }
        if self.S.contains(&playerid){
            suits.push('S');
        }
        if self.D.contains(&playerid){
            suits.push('D');
        }
        if self.C.contains(&playerid){
            suits.push('C');
        }
        suits
    }
    pub fn probability_that_this_player_ran_out_of_this_suit(&self,player:u8,suit:char,myid:u8)->bool{
        //players: opponenet left and right and my partner
        let mut played_card_by_this_player_in_this_hand=String::new();
        let mut next_player:u8;
        if self.handhistory.len() as u8==0{
            return false;
        }
        for i in self.handhistory.iter(){
            let winner_id=i.2;
            let mut winning_card:String=String::new();
            if i.1[0].as_bytes()[1] as char==suit{
                //if that hand was playedd with this suit
                let mut thrown_by=i.0;
                for k in i.1.iter(){
                    if winner_id==thrown_by{
                        winning_card=k.clone();
                    }
                    thrown_by=(thrown_by+1)%4;
                }
                thrown_by=i.0;
                let mut card_keys:Vec<u8>=Vec::new();
                card_keys.push(card_mapto_key(i.1[0].as_bytes()[0] as char));
                if thrown_by!=player{
                     for j in i.1[1..4].iter(){
                        
                    next_player=(thrown_by+1)%4;
                    if next_player==player{
                        played_card_by_this_player_in_this_hand=j.clone();
                        //break;
                    }
                    card_keys.push(card_mapto_key(j.as_bytes()[0] as char));
                    thrown_by=next_player; 
                }
                }
                else{
                    //thrown by is player
                    played_card_by_this_player_in_this_hand=i.1[0].clone();
                }
                //if me or my team has thrown 128 and this player throws 64
                //he thrown card of different suit
                //println!("Hello why bug here{}",played_card_by_this_player_in_this_hand);
                if played_card_by_this_player_in_this_hand.as_bytes()[1] as char!=i.1[0].as_bytes()[1] as char{
                    return true;
                }
                //println!("Winning card: {}",winning_card);
                if winner_id!=(myid+2)%4{
                    if winning_card.as_bytes()[0] as char=='J' && (played_card_by_this_player_in_this_hand.as_bytes()[0] as char=='7'||played_card_by_this_player_in_this_hand.as_bytes()[0] as char=='8'||played_card_by_this_player_in_this_hand.as_bytes()[0] as char=='Q'||played_card_by_this_player_in_this_hand.as_bytes()[0] as char=='K'){
                        return true;
                    }
                }
                if card_mapto_key(played_card_by_this_player_in_this_hand.as_bytes()[0]as char)==64&&card_keys.contains(&128){
                    thrown_by=i.0;
                    if (myid==(thrown_by+2)%4)||(myid==thrown_by){
                        //it is thrown by my team..
                        if card_mapto_key(i.1[0].as_bytes()[0] as char)==128||card_mapto_key(i.1[2].as_bytes()[0] as char)==128{
                            return true;
                        }
                    }
                    else{
                        if card_mapto_key(i.1[0].as_bytes()[0] as char)!=128||card_mapto_key(i.1[2].as_bytes()[0] as char)!=128{
                                return true;
                        }
                    }
                }

            }   
        }
        false
    }
}

