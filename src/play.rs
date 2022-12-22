use crate::api_rust_data::{
    Play,
    ThrowCard,
    RevealTrump,
   RevealTrumpAndThrowCard,
};
use crate::algorithm::Knowledge;
fn throwcard(optimal_card:String)->String{
    format!(r#"{{
       "card":{}, 
    }}"#,optimal_card)
}
fn reveal_trump()->String{
    format!(r#"{{
        "revealTrump":{},
    }}"#,true)
}
fn reveal_trump_play_card(optimal_card:String)->String{
    format!(r#"{{
        "revealTrump":{},
        "card":{},
    }}"#,true,optimal_card)
}
fn get_bid_winnerid(bidhistory:&Vec<(String,u8)>)->String{
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
pub fn play_game(payload:&Play)->String{
    let mut knowledge=Knowledge::init(&mut Knowledge::default());//init knowledge
    let Play{trumpSuit:trumpsuit,trumpRevealed:trumprevealed,..}=payload;

    if payload.handsHistory.len()!=0 || payload.cards.len()!=8{
        //update knowledge
        make_knowledge(&mut knowledge, &payload.handsHistory);
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