mod api_rust_data;
use super::api_rust_data::{
    Play,
    ThrowCard,
    RevealTrump,
   RevealTrumpAndThrowCard,
};
mod algorithm;
use super::algorithm::{
    Knowledge,
};
fn throwcard(optimal_card:String)->ThrowCard{
    ThrowCard{
       card:optimal_card, 
    }
}
fn reveal_trump()->RevealTrump{
    RevealTrump{
        revealTrump:true,
    }
}
fn reveal_trump_play_card(optimal_card:String)->{
    RevealTrumpAndThrowCard{
        revealTrump:true,
        card:optimal_card,
    }
}
fn get_bid_winnerid(bidhistory:&Vec<(String,u8)>)->String{
    //get bid winner player id
    let mut bid=0;
    let mut winner_id=String::new();
    for i in 0..bidhistory.len(){
        if bidhistory[i].1>=bid{
            bid=bidhistory[i].1;
            winner_id=bidhistory[i].0;
        }
    }
    winner_id
}
fn play_game(payload:&Play)-><T>{
    let mut knowledge=Knowledge::init(&mut moduleinrust::Knowledge::default());
    let Play(trumpSuit:trumpsuit,trumpRevealed:trumprevealed,..)=payload;
    if payload.handsHistory.len()=0 || payload.cards.len()==8{
        //flag this is new game or old game
        is_new_game=true;
    }
    else{
        is_new_game=false;
    }
    /*yedi naya game ho bhaney knowlede feri initialize huna paryo.. else update knowledge */
    let bid_winner_playerid=get_bid_winnerid(&payload.bidHistory);
    //if its your turn throw card
    if payload.played.len()==0{
        //your 1st turn
        return make_first_move();

    }
    let suit:char=payload.played[0].as_bytes()[1] as char;//basically this hand suit

    if payload.played.len()==1{
        //your 2nd turn
        make__second_move();
        
    }
    if payload.played.len()==2{
        //your third turn
        let partner_card=payload.played[0];
        make_third_mode();

        
    }
    if payload.played.len()==3{
        //your 4th turn
        let partner_card=payload.played[1];
        make_fourth_move();
        
    }
    

    
}