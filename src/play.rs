use crate::api_rust_data::{
    Play,
    TrumpRevealedBy,
    TrumpRevealEnum,
    TrumpSuitEnum,
};
use std::collections::HashMap;
use crate::knowledge::{
    Knowledge,
    MyCARDS,
    card_mapto_key,
};
#[derive(Debug)]
struct GameDetails{
        card_map_to_rank_point:HashMap<char,(u8,u8)>,
        we_are_winning:bool,//if we are winning
        trump_revealed:bool,//its tells trump revealed or not.. initially set true because the data from payload comes in false
        trump_suit:char,//here stores trump_suit
        trump_revealed_by:TrumpRevealedBy,
        i_won_the_bid:bool,
        suits:Vec<char>,
        last_hand_winner:String,
        this_hand_suit:char,
        partner_card:u8,//keep track of your partners card
        sum_of_points:u8,//keep track of points,
        trump_revealed_in_this_hand:bool,//check if trump was revealed in this hand
        trump_revealed_by_you:bool,//check if it was you who revealed the trump
}
pub mod play_game{
    use super::*;
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
                card_map_to_rank_point:HashMap::from([
                    //each suit cards ranks and points
                    ('J',(1,3)),
                    ('9',(2,2)),
                    ('1',(3,1)),
                    ('T',(4,1)),
                    ('K',(5,0)),
                    ('Q',(6,0)),
                    ('8',(7,0)),
                    ('7',(8,0))
    ]),
                we_are_winning:false,
                trump_revealed:true,
                trump_suit:'_',
                trump_revealed_by:TrumpRevealedBy::default(),
                i_won_the_bid:false,
                suits:Vec::new(),
                last_hand_winner:String::new(),
                this_hand_suit:'_',
                partner_card:0,
                sum_of_points:0,
                trump_revealed_in_this_hand:false,
                trump_revealed_by_you:false,
        };
        let mut mycards:MyCARDS=MyCARDS::init(&mut MyCARDS::default());
        let mut knowledge=Knowledge::init(&mut Knowledge::default());//init knowledge
        let Play{trumpSuit:trumpsuit,trumpRevealed:trumprevealed,..}=payload;
       match trumpsuit{
            TrumpSuitEnum::Suit(s)=>gamedetails.trump_suit=*s,
          _=>gamedetails.trump_revealed=false,
          }
          match trumprevealed{
            TrumpRevealEnum::RevealedBy(revealer)=>gamedetails.trump_revealed_by=revealer.clone(),
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

        //check when was trump revealed and who revealed
        if gamedetails.trump_revealed{
            gamedetails.trump_revealed_in_this_hand=payload.handsHistory.len() as u8==gamedetails.trump_revealed_by.hand-1;
            gamedetails.trump_revealed_by_you=payload.playerId==gamedetails.trump_revealed_by.playerId;
        }
        //organize your cards and arrange suits by rank
        mycards.update_my_cards(&payload.cards);
        gamedetails.suits=arrange_suits(&mycards);//arrange the suits
        //make knowledge from handhistory
        if payload.handsHistory.len()==0{
            //technically 1st hand winner is the one who first throww the card.
            if payload.played.len()==2{
                gamedetails.last_hand_winner=payload.playerIds[((payload.playerIds.iter().position(|r| r == &payload.playerId)).unwrap()+(2 as usize))%4].clone();
                //it is my partner
            }
            if payload.played.len()==3{
                let previous_player_id_index=payload.playerIds.iter().position(|r| r == &payload.playerId).unwrap()-1;
                gamedetails.last_hand_winner=payload.playerIds[(previous_player_id_index+(2 as usize))%4].clone();//it is one of my opponent
            }
        }
        if payload.handsHistory.len()!=0 || payload.cards.len()!=8{
            //make knowledge
            make_knowledge(&mut knowledge, &payload.handsHistory,&mut gamedetails);
        }
        //make knowledge of played card
        if payload.played.len()!=0{
            knowledge.update_knowledge(&payload.played);
        }
      //make knowledge of opponenet and partner player
        let bid_winner_playerid:String=get_bid_winnerid(&payload.bidHistory);// get bid winner id
         
        //trrow card according to your turn
        if payload.played.len()==0{
            //your 1st turn
            return make_first_move(&payload,&mycards,&knowledge,&gamedetails);
        }
        //get sum of points from thrown cards
        gamedetails.sum_of_points=get_total_points(&payload.played, &gamedetails.card_map_to_rank_point);

        //if this is your 2nd third or fourth turn
        gamedetails.this_hand_suit=payload.played[0].as_bytes()[1] as char;//basically this hand suit
        if payload.played.len()==1{
            
            //your 2nd turn
            return make_second_move(&payload,&mycards,&knowledge,&gamedetails);
        }
        //get partners card
        if payload.played.len()==2{
            gamedetails.partner_card=card_mapto_key(payload.played[0].as_bytes()[0] as char);
        }
        if payload.played.len()==3{
            gamedetails.partner_card=card_mapto_key(payload.played[1].as_bytes()[0] as char);
        }
        let leading_player:String=pridict_winning_player(&payload.playerIds, &payload.played, &gamedetails);
        if  payload.playerIds[((payload.playerIds.iter().position(|r| r == &leading_player).unwrap())+2)%4]==payload.playerId{
            gamedetails.we_are_winning=true;
        }
        println!("Gamedetails{:?}",gamedetails);
        if gamedetails.we_are_winning{
            //maximize card points
            return throw_max(&mycards,&gamedetails,&knowledge);//try to get max points
        }   
        else{
            //minimize 
            return throw_min(&mycards,&gamedetails,&knowledge);//give min points
        }
}
    fn throw_max(mycards:&MyCARDS,gamedetails:&GameDetails,knowledge:&Knowledge)->String{
            //give max point
            //avoid using trump card
            //check opponenet has cards or not
            //reveal trump if it wasn't you
            if gamedetails.suits.contains(&(gamedetails.this_hand_suit)){
                //if i have this hand suit and my team is winning
                        return throwcard(mycards.get_card(gamedetails.this_hand_suit,true));
            }
            //doesnot contains this hand suit......
            //...................................
            else if !gamedetails.trump_revealed && !gamedetails.i_won_the_bid{
                    return reveal_trump();
            }
            else if !gamedetails.trump_revealed && gamedetails.i_won_the_bid{
                //trump isn't revealed and you are the bid winner
                if knowledge.get_total_cards_not_played(gamedetails.this_hand_suit)>=1{
                    //get total cards not played of this hand suit
                    if knowledge.card_greater_than_this_rank_card_exist(gamedetails.partner_card, gamedetails.this_hand_suit){
                        //check if opponent has card greater than played by my partner
                        //throw trump and check for points
                            if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                                return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit, true));
                            }
                            else{
                                //throw some random minimum card
                                return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                            }
                        }
                        //other player has card rank lesser than my partner
                    else{
                        //throw other card and check for points
                        //check duita suits.. yedi trump_suit hoina ra 
                        //aruko trump card sakkauney kaam garnu paryo
                        
                        if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                            // check if you have the winning suits
                            if knowledge.check_played_card(128, gamedetails.trump_suit) &&mycards.you_have_this_card(128,gamedetails.trump_suit){
                                //if i have the J card... throw it
                                return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit, true));
                            }
                            else if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.trump_suit),gamedetails.trump_suit){
                                return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit, true));
                            }
                            else{
                                for i in gamedetails.suits.iter(){
                                    if gamedetails.suits.len()!=1{
                                        if i!=&gamedetails.trump_suit{
                                            //avoid throwing trump card
                                            return throwcard(mycards.get_card(*i, false));
                                        }
                                }
                                else{
                                    continue;
                                }
                            }
                            return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                            }
                            
                        }
                        else{
                            //throw some random minimum card
                            return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                        }
                    }
                }
                else{
                    //no more this hand suit card left .. all used..
                    if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                        //you have trump suits
                        // check if you have the winning suits
                        if knowledge.check_played_card(128, gamedetails.trump_suit) &&mycards.you_have_this_card(128,gamedetails.trump_suit){
                            return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit, true));
                        }
                        else if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.trump_suit),gamedetails.trump_suit){
                            return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit, true));
                        }
                        else{
                            for suit in gamedetails.suits.iter(){
                                if gamedetails.suits.len()!=1{
                                    if suit!=&gamedetails.trump_suit{
                                        //avoid throwing trump card
                                        return throwcard(mycards.get_card(*suit, false));
                                    }
                            }
                            else{
                                continue;
                            }
                            }
                            return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                        }
                        
                    }
                    else{
                        //you don't have a trump suit catds..
                        return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                    }
                }
            }
            //trump has been revealed
        else {
            //i am out of this hand suit..
            if gamedetails.trump_revealed_in_this_hand && gamedetails.trump_revealed_by_you{
                //if trump revealed in this hand and it was me.
                if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                    return throwcard(mycards.get_card(gamedetails.trump_suit, true));
                }
                else if !knowledge.card_greater_than_this_rank_card_exist(gamedetails.partner_card, gamedetails.this_hand_suit) && knowledge.get_total_cards_not_played(gamedetails.this_hand_suit)>=1 {
                    return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(),true));
                }
                else{
                    return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(),false));
                }
            }
            if knowledge.get_total_cards_not_played(gamedetails.this_hand_suit)>=1{
                //if any this hand suit cards left to play with the opponenents
                if knowledge.card_greater_than_this_rank_card_exist(gamedetails.partner_card, gamedetails.this_hand_suit){
                    if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                        //throw max trump card
                        return throwcard(mycards.get_card(gamedetails.trump_suit, true));
                    }
                    else{
                        //throw any random minimim card
                        return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                    }
                }
                else{
                    //my team is winning so throw any max point card
                    for i in gamedetails.suits.iter(){
                        if gamedetails.suits.len()!=1{
                            if i!=&gamedetails.trump_suit{
                                //avoid throwing trump card
                                return throwcard(mycards.get_card(*i, true));
                            }
                    }
                    else{
                        continue;
                    }
                }
                return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), true));
                }
            }
            else {
                //this hand suit card over... opponent will also throw trump card.. so throw max..
                //throw either trump or any random minimum card
                //check if opponent has any trump card left
                if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                    // check if you have the winning suits
                    if knowledge.check_played_card(128, gamedetails.trump_suit) &&mycards.you_have_this_card(128,gamedetails.trump_suit){
                        return throwcard(mycards.get_card(gamedetails.trump_suit, true));
                    }
                    else if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.trump_suit),gamedetails.trump_suit){
                        return throwcard(mycards.get_card(gamedetails.trump_suit, true));
                    }
                    else{
                        for i in gamedetails.suits.iter(){
                            if gamedetails.suits.len()!=1{
                                if i!=&gamedetails.trump_suit{
                                    //avoid throwing trump card
                                    return throwcard(mycards.get_card(*i, false));
                                }
                        }
                        else{
                            continue;
                        }
                    }
                    return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                    }
                    
                }
                else{
                    //throw any random minimim card
                    return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                }
            }
        }
    }
    fn throw_min(mycards:&MyCARDS,gamedetails:&GameDetails,knowledge:&Knowledge)->String{
        if gamedetails.suits.contains(&(gamedetails.this_hand_suit)){
            //if i have this hand suit and my team is winning
                    return throwcard(mycards.get_card(gamedetails.this_hand_suit,false));
        }
        else if !gamedetails.trump_revealed{
                //trump not revealed
                if gamedetails.i_won_the_bid{
                    if knowledge.get_total_cards_not_played(gamedetails.this_hand_suit)>=1{
                        if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                            //throw max trump card
                            return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit, true));
                        }
                        else{
                            //throw any random minimim card
                            return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                        } 
                    }
                    else{
                        for i in gamedetails.suits.iter(){
                            if gamedetails.suits.len()!=1{
                                if i!=&gamedetails.trump_suit{
                                    //avoid throwing trump card
                                    return throwcard(mycards.get_card(*i, false));
                                }
                        }
                        else{
                            continue;
                        }
                    }
                    return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
                    }
                }
                else{
                    return reveal_trump();
                }
            }
        else{
            //trump revealed
            if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                //throw max trump card
                if knowledge.check_played_card(128, gamedetails.trump_suit) &&mycards.you_have_this_card(128,gamedetails.trump_suit){
                    return throwcard(mycards.get_card(gamedetails.trump_suit, true));
                }
                else if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.trump_suit),gamedetails.trump_suit){
                    return throwcard(mycards.get_card(gamedetails.trump_suit, true));
                }
                else{
                    return throwcard(mycards.get_card(gamedetails.trump_suit,false));
                }
            }
            else{
                //throw any random minimim card
                return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
            }

        }
    }
    fn make_first_move(payload:&Play,mycards:&MyCARDS,knowledge:&Knowledge,gamedetails:&GameDetails)->String{
        //make getting point strategy 
        //yedi trump cards chha bhaney tyo nafaalney.. in the beginning.. sakdo try opponeent ko trump card sakkauna
        //if no point getting card.. throw card with min num of suits
        //see if the cards has high rank
        //see if you have the high rank card
        for i in gamedetails.suits.iter(){
            match i{
                    'H'=>{
                        let key=mycards.H[0];
                        match key{
                            //try to get the point
                            128=>{
                                return throwcard(mycards.map_key_to_card(key,'H'))
                            },
                            64=>{
                                    if knowledge.card_greater_than_this_rank_card_exist(key,'H')&& knowledge.no_possibility_of_trump_reveal('H',mycards.H.len()as u8){
                                        return throwcard(mycards.map_key_to_card(key,'H'))
                                    }
                            },
                            _=>{
                                //throw less suit cards
                                if mycards.H.len()<=2{
                                    return throwcard(mycards.map_key_to_card(*mycards.H.last().unwrap(),'H'))
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
                            128=>{
                                return throwcard(mycards.map_key_to_card(key,'D'))
                            },
                            64=>{
                                    if knowledge.card_greater_than_this_rank_card_exist(key,'D')&& knowledge.no_possibility_of_trump_reveal('D',mycards.D.len()as u8){
                                        return throwcard(mycards.map_key_to_card(key,'D'))
                                    }
                            },
                            _=>{
                                //throw less suit cards
                                if mycards.D.len()<2{
                                    return throwcard(mycards.map_key_to_card(*mycards.D.last().unwrap(),'D'))
                                }
                                else{
                                    continue;
                                }
                            },
                        }
                    },
                    &'C'=>{
                        let key=mycards.C[0];//to maximize
                        match key{
                            //try to get the point
                            128=>{
                                return throwcard(mycards.map_key_to_card(key,'C'))
                            },
                            64=>{
                                    if knowledge.card_greater_than_this_rank_card_exist(key,'C')&& knowledge.no_possibility_of_trump_reveal('C',mycards.C.len() as u8){
                                        return throwcard(mycards.map_key_to_card(key,'C'))
                                    }
                            },
                            _=>{
                                //throw less suit cards
                                if mycards.C.len()<2{
                                    return throwcard(mycards.map_key_to_card(*mycards.C.last().unwrap(),'C'))
                                }
                                else{
                                    continue;
                                }
                            },
                        }
                    },
                    &'S'=>{
                        let key=mycards.S[0];
                        match key{
                            //try to get the point
                            128=>{
                                return throwcard(mycards.map_key_to_card(key,'S'))
                            },
                            64=>{
                                    if knowledge.card_greater_than_this_rank_card_exist(key,'S')&& knowledge.no_possibility_of_trump_reveal('S',mycards.S.len() as u8){
                                        return throwcard(mycards.map_key_to_card(key,'S'))
                                    }
                            },
                            _=>{
                                //throw less suit cards
                                if mycards.S.len()<2{
                                    return throwcard(mycards.map_key_to_card(*mycards.S.last().unwrap(),'S'))
                                }
                                else{
                                    continue;
                                }
                            },
                        }
                    },
                    _=>{},
                }
    }
    format!(r#"{{"card":"{}"}}"#,payload.cards[0])
}
    fn make_second_move(payload:&Play,mycards:&MyCARDS,knowledge:&Knowledge,gamedetails:&GameDetails)->String{
        let opp_card_key:u8=card_mapto_key(payload.played[0].as_bytes()[0] as char);
        if gamedetails.suits.contains(&(gamedetails.this_hand_suit)){
            if mycards.you_have_the_higher_rank_card(opp_card_key,gamedetails.this_hand_suit){
                //you have card greater than played card
                return throwcard(mycards.get_card(gamedetails.this_hand_suit,true));
            }
            else{
                return throwcard(mycards.get_card(gamedetails.this_hand_suit,false));
            }   
        }
        else if !gamedetails.trump_revealed && !gamedetails.i_won_the_bid{
                return reveal_trump();
        }
        else if !gamedetails.trump_revealed && gamedetails.i_won_the_bid{
                //yedi tyo suit ko aru thuprai high rank cards chhan
                if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                    //yedi mah sanga trump card chha bhaney throw it
                return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit,true));
                }
                else{
                    //chhaina bhaney throw random minimum card..
                    return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(),false));
                }
        }
        //trumpRevealed part
        else if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                return throwcard(mycards.get_card(gamedetails.trump_suit, true));
            }
        else{
                return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(), false));
            }
    }
    fn make_knowledge(knowledge:&mut Knowledge,handshistory:&Vec<(String,Vec<String>,String)>,gamedetails:&mut GameDetails){
          //update last hand winner
          gamedetails.last_hand_winner=(handshistory.last().unwrap().2).clone();
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
                data.push(('H',mycards.H[0]));
            }
            if mycards.D.len()>0{
                data.push(('D',mycards.D[0]));
            }
            if mycards.S.len()>0{
                data.push(('S',mycards.S[0]));
            }
            if mycards.C.len()>0{
                data.push(('C',mycards.C[0]));
            }
            if data.len()>=2{
                for i in 0..data.len(){
                    let mut temp:(char,u8)=data[i];
                    for j in (i+1)..data.len(){
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
    fn pridict_winning_player(players:&Vec<String>,played:&Vec<String>,gamedetails:&GameDetails)->String{
        //if winner team is yours.. maximize the point. else minimize
            let mut possible_winner:&String=&gamedetails.last_hand_winner;
            let mut thrown_by:&String=&gamedetails.last_hand_winner;
            let mut winner_rank_point:(u8,u8)=gamedetails.card_map_to_rank_point[&(played[0].as_bytes()[0] as char)];
            println!("thrown by player ={}",thrown_by);
            let mut winning_suit:char=gamedetails.this_hand_suit;//if trump card. change it to trump_suit
            for i in played[1..played.len()].iter(){
                thrown_by= &(players[((players.iter().position(|r| r == thrown_by)).unwrap()+(1 as usize))%4]);
                println!("thrown by player ={}",thrown_by);
                let played_suit=i.as_bytes()[1] as char;
                let rank_point:(u8,u8)=gamedetails.card_map_to_rank_point[&(i.as_bytes()[0] as char)];
                if winning_suit==played_suit && rank_point.0<winner_rank_point.0{
                    winner_rank_point=rank_point;
                    possible_winner=&thrown_by;
                }
                else if played_suit==gamedetails.trump_suit{
                    if winning_suit!=gamedetails.trump_suit{
                        winning_suit=gamedetails.trump_suit;
                        winner_rank_point=rank_point;
                        possible_winner=&thrown_by; 
                    }
                    else if rank_point.0<winner_rank_point.0 {
                        winner_rank_point=rank_point;
                        possible_winner=&thrown_by;
                    } 
                }   
            }
            possible_winner.to_string()
    }
    fn get_total_points(played:&Vec<String>,card_map_to_rank_points:&HashMap<char,(u8,u8)>)->u8{
        let mut sum=0;
        for i in played.iter(){
            sum+=card_map_to_rank_points[&(i.as_bytes()[0] as char)].1;
        }
        sum
    }     
}
/*
My next implementation

Optimize bidding
optimize throwing  9 card
try to get max point.. 
try to give minimum point
*/