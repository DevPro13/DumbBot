use crate::api_rust_data::{
    Play,
};
use std::collections::{
    HashMap,
    HashSet,
};
use crate::knowledge::Knowledge;
use super::choosetrump::Trump;
pub mod play_game{
    use std::collections::HashMap;

const cards:HashMap<char,(u8,u8)>=HashMap::from([
                //each suit cards ranks and points
                'J',(1,3),
                '9',(2,2),
                '1',(3,1),
                'T',(4,1),
                'K',(5,0),
                'Q',(6,0),
                '8',(7,0),
                '7',(8,0),
    ]);
let mut has_suit_card:bool=true;//used while revealing trump
let mut i_am_bid_winner:bool=false;//used while revealing trump card if you're the one who put it
    pub fn play_card(payload:&Play)->String{
        fn throwcard(optimal_card:String)->String{
            format!(r#"{{
               "card":"{}"
            }}"#,optimal_card)
        }
        fn reveal_trump()->String{
            format!(r#"{{
                "revealTrump":{}
            }}"#,true)
        }
        fn reveal_trump_play_card(optimal_card:String)->String{
            format!(r#"{{
                "revealTrump":{},
                "card":"{}"
            }}"#,true,optimal_card)
        }
        let mut knowledge=Knowledge::init(&mut Knowledge::default());//init knowledge
        let Play{trumpSuit:trumpsuit,trumpRevealed:trumprevealed,..}=payload;
    
        if payload.handsHistory.len()!=0 || payload.cards.len()!=8{
            //update knowledge
            make_knowledge(&mut knowledge, &payload.handsHistory);
        }
        if payload.played.len()!=0{
            make_knowledge(&mut knowledge,payload.played);
        }
      //make knowledge of opponenet and partner player
        let bid_winner_playerid=get_bid_winnerid(&payload.bidHistory);
        //if its your turn throw card
        if payload.played.len()==0{
            //your 1st turn
            //return make_first_move(&payload.cards,&knowledge);
    
        }
        let suit:char=payload.played[0].as_bytes()[1] as char;//basically this hand suit
    
        if payload.played.len()==1{
            //your 2nd turn
            //make__second_move();
            
        }
        if payload.played.len()==2{
            //your third turn
            let partner_card=payload.played[0].clone();
            //make_third_mode();   
        }
        if payload.played.len()==3{
            //your 4th turn
            let partner_card=payload.played[1].clone();
            //make_fourth_move();
        }
        format!(r#"{{"abc"}}"#)//remove it LATER
    }
    fn give_sum_of_points(board)->u8{
        0

    }
}
/* 
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
*/
fn make_second_move(){
    //see if the cards has high rank
    //see if you have the high rank card
    //
}
*/
pub fn get_bid_winnerid(bidhistory:&Vec<(String,u8)>)->String{
    //get bid winner player id
    let mut bid=0;
    let mut winner_id=String::new();
    for i in 0..bidhistory.len(){
        if bidhistory[i].1>=bid{
            bid=bidhistory[i].1;
            winner_id=bidhistory[i].0.clone();
        }
    }
    winner_id
}
fn make_knowledge(knowledge:&mut Knowledge,handshistory:&Vec<(String,Vec<String>,String)>){
    for i in handshistory{
        knowledge.update_knowledge(&i.1);
    }
}
pub fn play_card(payload:&Play)->String{
    fn throwcard(optimal_card:String)->String{
        format!(r#"{{
           "card":"{}"
        }}"#,optimal_card)
    }
    fn reveal_trump()->String{
        format!(r#"{{
            "revealTrump":{}
        }}"#,true)
    }
    fn reveal_trump_play_card(optimal_card:String)->String{
        format!(r#"{{
            "revealTrump":{},
            "card":"{}"
        }}"#,true,optimal_card)
    }
    let mut knowledge=Knowledge::init(&mut Knowledge::default());//init knowledge
    let Play{trumpSuit:trumpsuit,trumpRevealed:trumprevealed,..}=payload;

    if payload.handsHistory.len()!=0 || payload.cards.len()!=8{
        //update knowledge
        make_knowledge(&mut knowledge, &payload.handsHistory);
    }
    if payload.played.len()!=0{
        make_knowledge(&mut knowledge,payload.played);
    }
  //make knowledge of opponenet and partner player
    let bid_winner_playerid=get_bid_winnerid(&payload.bidHistory);
    //if its your turn throw card
    if payload.played.len()==0{
        //your 1st turn
        //return make_first_move(&payload.cards,&knowledge);

    }
    let suit:char=payload.played[0].as_bytes()[1] as char;//basically this hand suit

    if payload.played.len()==1{
        //your 2nd turn
        //make__second_move();
        
    }
    if payload.played.len()==2{
        //your third turn
        let partner_card=payload.played[0].clone();
        //make_third_mode();   
    }
    if payload.played.len()==3{
        //your 4th turn
        let partner_card=payload.played[1].clone();
        //make_fourth_move();
    }
    format!(r#"{{"abc"}}"#)//remove it LATER
}