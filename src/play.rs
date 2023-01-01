use crate::api_rust_data::{
    Play,
};
use std::collections::HashMap;
use crate::knowledge::{
    Knowledge,
    MyCARDS,
};
#[derive(Debug)]
struct GameDetails{
        we_are_winning:bool,//if we are winning
        opponent_winning:bool,//if opponenet are winning
        trump_revealed:bool,//its tells trump revealed or not.. initially set true because the data from payload comes in false
        trump_suit:char,//here stores trump_suit
        trump_revealed_by:TrumpRevealedBy,
        i_won_the_bid:bool,
        suits:Vec<char>,

}
//use super::choosetrump::Trump;
pub mod play_game{
    use super::*;
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
    pub fn play_card(payload:&Play)->String{
        if payload.cards.len()==1{
            //if last card remains..throw it..
            return format!(r#"{{"card":"{}"}}"#,payload.cards[0]);
        }
        let mut gamedetails=GameDetails{
                we_are_winning:false,
                opponent_winning:false,
                trump_revealed:true,
                trump_suit:'_',
                trump_revealed_by:TrumpRevealedBy::default(),
                i_won_the_bid:false,
                suits:Vec::new,
        };
        let mut mycards:MyCARDS=MyCARDS::init(&mut MyCARDS::default());
        let mut knowledge=Knowledge::init(&mut Knowledge::default());//init knowledge
        let Play{trumpSuit:trumpsuit,trumpRevealed:trumprevealed,..}=payload;

        //check trump reveal status 
        match trumpSuit{
            TrumpSuitEnum::Suit(suit)=>gamedetails.trump_suit=suit,
          _=>gamedetails.trump_revealed=false,
          }
          match trumprevealed{
            TrumpRevealEnum::RevealedBy(revealer)=>gamedetails.trump_revealed_by=revealer,
          _=>gamedetails.trump_revealed=false,
          }
          if (gamedetails.trump_suit!='_'&& !gamedetails.trump_revealed)||gamedetails.trump_suit=='_'{
              //trump not revealed
              if gamedetails.trump_suit!='_'{
                gamedetails.i_won_the_bid=true;
                gamedetails.trump_revealed=false;
              }
          }
          else{
            //trump_revealed
            gamedetails.trump_revealed=true;
        }

        //organize your cards and arrange suits by rank
        mycards.update_my_cards(&payload.cards);
        gamedetails.suits=arrange_suits(&mycards);//arrange the suits
        //make knowledge from handhistory
        if payload.handsHistory.len()!=0 || payload.cards.len()!=8{
            //update knowledge
           knowledge.update_knowledge(&payload.played);
        }
        //make knowledge of played card
        if payload.played.len()!=0{
            make_knowledge(&mut knowledge,&payload.played);
        }
      //make knowledge of opponenet and partner player
        let bid_winner_playerid=get_bid_winnerid(&payload.bidHistory);// get bid winner id
         
        //trrow card according to your turn
        if payload.played.len()==0{
            //your 1st turn
            return make_first_move(&payload,&mycards,&knowledge,&gamedetails);
        }
        let this_hand_suit:char=payload.played[0].as_bytes()[1] as char;//basically this hand suit
    
        if payload.played.len()==1{
            //your 2nd turn
            //if you have winning card
            if unpredictable_case(){
                //don't know who gonna win
                //throw card greater than priviously thrown or else throw 0 point card
            }
                
            
        }
        if payload.played.len()==2{
            //your third turn
            let partner_card=payload.played[0].clone();
            return make_third_move(this_hand_suit,);  
        }
        if payload.played.len()==3{
            //your 4th turn
            let partner_card=payload.played[1].clone();
            return make_fourth_move();
        }
        format!(r#"{{"card":"{}"}}"#,payload.cards[0])//throw 1st card as default
}
    fn make_first_move(payload:&Play,mycards:&MyCARDS,knowledge:&Knowledge,gamedetails:&GameDetails)->String{
        //make getting point strategy 
        //yedi trump cards chha bhaney tyo nafaalney.. in the beginning.. sakdo try opponeent ko trump card sakkauna
        //if no point getting card.. throw card with min num of suits
        //see if the cards has high rank
        //see if you have the high rank card
        for i in gamedetails.trump_suit.iter(){
            match i{
                    'H'=>{
                        let key=mycards.H[0];
                        match key{
                            //try to get the point
                            64|128=>{
                                    if knowledge.no_card_greater_than_this_rank_card(key,'H')&& knowledge.no_possibility_of_trump_reveal(key,'H',mycards.H.len()){
                                        return throwcard(mycards.map_key_to_card(key,'H'))
                                    }
                            },
                            _=>{
                                //throw less suit cards
                                if mycards.H.len()<=2{
                                    return throwcard(mycards.map_key_to_card(mycards.H.last().unwrap(),'H'))
                                }
                                else{
                                    continue;
                                }
                            },
                        }
                        },
                    'D'=>{
                        let key=mycards.D[0];
                        match key{
                            //try to get the point
                            64|128=>{
                                    if knowledge.no_card_greater_than_this_rank_card(key,suit)&& knowledge.no_possibility_of_trump_reveal(key,'D',mycards.D.len()){
                                        return throwcard(mycards.map_key_to_card(key,'D'))
                                    }
                            },
                            _=>{
                                //throw less suit cards
                                if mycards.D.len()<2{
                                    return throwcard(mycards.map_key_to_card(mycards.D.last().unwrap(),'D'))
                                }
                                else{
                                    continue;
                                }
                            },
                        }

                    },
                    'C'=>{
                        let key=mycards.C[0];//to maximize
                        match key{
                            //try to get the point
                            64|128=>{
                                    if knowledge.no_card_greater_than_this_rank_card(key,suit)&& knowledge.no_possibility_of_trump_reveal(key,'C',mycards.C.len()){
                                        return throwcard(mycards.map_key_to_card(key,'C'))
                                    }
                            },
                            _=>{
                                //throw less suit cards
                                if mycards.C.len()<2{
                                    return throwcard(mycards.map_key_to_card(mycards.C.last().unwrap(),'C'))
                                }
                                else{
                                    continue;
                                }
                            },
                        }
                    },
                    'S'=>{
                        let key=mycards.S[0];
                        match key{
                            //try to get the point
                            64|128=>{
                                    if knowledge.no_card_greater_than_this_rank_card(key,suit)&& knowledge.no_possibility_of_trump_reveal(key,'S',mycards.S.len()){
                                        return throwcard(mycards.map_key_to_card(key,'S'))
                                    }
                            },
                            _=>{
                                //throw less suit cards
                                if mycards.S.len()<2{
                                    return throwcard(mycards.map_key_to_card(mycards.S.last().unwrap(),'S'))
                                }
                                else{
                                    continue;
                                }
                            },
                        }
                    },
                }
    }
    fn make_second_move(){
        //see if the cards has high rank
        //see if you have the high rank card


    }
    fn make_third_move(){
        //see if the cards has high rank
        //see if you have the high rank card        
}
    fn make_fourth_move(){
        //see if the cards has high rank
        //see if you have the high rank card
    }
    fn check_team_winning_or_not(played:&Vec<String>){
        if playes,len()==1{


        }
        if playes,len()==2{
            
        }
        if playes,len()==3{
            
        }
    }
    fn give_sum_of_points(board)->u8{
        0
}
    fn check_players_has_the_card_of_given_suit(knowledge:&Knowledge,suit:char)->bool{
        //check if any player has the card of given suit..
                knowledge[suit]==0

    }
    fn make_knowledge(knowledge:&mut Knowledge,handshistory:&Vec<(String,Vec<String>,String)>){
        for i in handshistory{
            knowledge.update_knowledge(&i.1);
        }
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
    fn arrange_suits(mycards:&MyCARDS)->Vec<char>{
        let mut data:Vec<(char,u8)>=Vec::new();
            if mycards.H.len()>0{
                data.push(('H',mycards.H.1));
            }
            if mycards.D.len()>0{
                data.push(('D',mycards.D.1));
            }
            if mycards.S.len()>0{
                data.push(('S',mycards.S.1));
            }
            if mycards.C.len()>0{
                data.push(('C',mycards.C.1));
            }
            if data.len()>=2{
                for i in 0..data.len(){
                    let mut temp:(char,u8)=data[i];
                    for j in (i+1)..4{
                        if data[j].1>temp.1{
                            temp=data[j];
                            data[j]=data[i];
                            data[i]=temp;
                        }
                        
                    }
                }
            }
            let mut suits:Vec<char>=Vec::new();
            for i in data.iter(){
                    suits.push(i.0);
            }
            suits
        }
        

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