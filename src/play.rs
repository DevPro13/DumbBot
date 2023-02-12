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
    HandsInformation,
    card_mapto_key,
};
use std::cell::RefCell;
use std::rc::Rc;
use crate::mcts_algorithm::MCTSTwentyNineGameTreeNode;
#[derive(Debug,Default)]
pub struct GameDetails{
        pub playerid:u8,
        pub card_map_to_rank_point:HashMap<char,(u8,u8)>,
        pub we_are_winning:bool,//if we are winning
        pub trump_revealed:bool,//its tells trump revealed or not.. initially set true because the data from payload comes in false
        pub trump_suit:char,//here stores trump_suit
        pub trump_revealed_by:TrumpRevealedBy,
        pub bid_winner_playerid:u8,//holds the id of bid winner
        pub i_won_the_bid:bool,
        pub suits:Vec<char>,//suits arrange form max cards
        pub suits_arrange_from_min:Vec<char>,//suits arranged from min cards
        pub last_hand_winner:u8,
        pub this_hand_suit:char,
        pub partner_card:u8,//keep track of your partners card
        pub sum_of_points:u8,//keep track of points,
        pub trump_revealed_in_this_hand:bool,//check if trump was revealed in this hand
        pub trump_revealed_by_you:bool,//check if it was you who revealed the trump
}
pub mod play_game{
    use std::borrow::BorrowMut;
    use super::*;
    use crate::knowledge::{map_key_to_card, self};

    use super::*;
    fn map_string_playerid_to_number(players:&Vec<String>,player:&String)->u8{
        players.iter().position(|r|r==player).unwrap() as u8
    }
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
                playerid:map_string_playerid_to_number(&payload.playerIds, &payload.playerId),
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
                bid_winner_playerid:0,//default:player 0
                i_won_the_bid:false,
                suits:Vec::new(),//arrange suits from max cards
                suits_arrange_from_min:Vec::new(),//arrange suits from min cards
                last_hand_winner:4,//if 4 means unknown
                this_hand_suit:'_',
                partner_card:0,
                sum_of_points:0,
                trump_revealed_in_this_hand:false,
                trump_revealed_by_you:false,
        };
        let mut mycards:MyCARDS=MyCARDS::init(&mut MyCARDS::default());
        let mut knowledge=Knowledge::init(&mut Knowledge::default());//init knowledge
        let mut hands_info=HandsInformation::init(&mut HandsInformation::default());//for hands info
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
        gamedetails.suits=arrange_suits_from_max(&mycards);//arrange the suits from max cards
        gamedetails.suits_arrange_from_min=arrange_suits_from_min(&mycards);//arrange the suits from min cards
        //make knowledge from handhistory
        if payload.handsHistory.len()as u8==0{
            //technically 1st hand winner is the one who first throww the card.
            if payload.played.len()as u8==0{
                gamedetails.last_hand_winner=gamedetails.playerid;//it is one of my opponent in the left
            }
            if payload.played.len()as u8==1{
                gamedetails.last_hand_winner=(gamedetails.playerid+3)%4;//it is one of my opponent in the left
            }
            if payload.played.len()as u8==2{
                 //its my 3th turn
                 //so my partner is the 1st player of this hand
                gamedetails.last_hand_winner=(gamedetails.playerid+2)%4;
                //it is my partner
            }
            if payload.played.len() as u8==3{
                //its my 4th turn
                //player before me is my opponent
                //that opponent player index+2% 4 is the 1st player of this hand
                gamedetails.last_hand_winner=(((gamedetails.playerid+1)%4)+2)%4;//it is one of my opponent
            }
        }
        if payload.handsHistory.len() as u8!=0 || payload.cards.len() as u8!=8{
            //make knowledge
            make_knowledge(&payload.playerIds,&mut knowledge, &payload.handsHistory,&mut gamedetails,&mut hands_info);
            //update suits and cards info for players
            update_players_suits_info(&mut hands_info, &payload.playerIds, &payload.handsHistory);
            update_hands_history_in_handsinfo(&mut hands_info,&payload);
        }
        println!("HandsInfo: {:?}",hands_info);
        //make knowledge of played card
        if payload.played.len()as u8!=0{
            knowledge.update_knowledge(&payload.played);
        }
      //make knowledge of opponenet and partner player
        gamedetails.bid_winner_playerid=map_string_playerid_to_number(&payload.playerIds,&get_bid_winnerid(&payload.bidHistory));// get bid winner id
        //............YOUR 1ST TURN.............
        if payload.played.len() as u8==0{
            return make_first_move(&payload,&mycards,&knowledge,&mut gamedetails,&hands_info);
        }
        //get sum of points from thrown cards
        gamedetails.sum_of_points=get_total_points(&payload.played, &gamedetails.card_map_to_rank_point);

        //if this is your 2nd third or fourth turn
        gamedetails.this_hand_suit=payload.played[0].as_bytes()[1] as char;//basically this hand suit

        //your 2nd turn to throw a card
        if payload.played.len() as u8==1{
            
            //your 2nd turn
            return make_second_move(&payload,&mycards,&knowledge,&mut gamedetails,&hands_info);
        }
        //get partners card
        if payload.played.len() as u8==2{
            gamedetails.partner_card=card_mapto_key(payload.played[0].as_bytes()[0] as char);
        }
        if payload.played.len() as u8==3{
            gamedetails.partner_card=card_mapto_key(payload.played[1].as_bytes()[0] as char);
        }
        let leading_player:(u8,String)=pridict_winning_player(&payload.played, &gamedetails);// holds player id and thrown card
        //println!("\n\n{:?}",leading_player);
        if gamedetails.playerid==(leading_player.0+2)%4 ||gamedetails.playerid==leading_player.0{
            //we are leading the game
            gamedetails.we_are_winning=true;
        }
        println!("Gamedetails{:?}",gamedetails);
        if gamedetails.we_are_winning{
            println!("1st card {:?}",mycards.S);
            //maximize card points
            return throw_max(&mycards,&mut gamedetails,&knowledge,&payload,&hands_info);//try to get max points
        }   
        else{
            //minimize 
            return throw_min(&mycards,&mut gamedetails,&knowledge,&payload,&hands_info);//give min points
        }
}
    fn throw_max(mycards:&MyCARDS,gamedetails:&mut GameDetails,knowledge:&Knowledge,payload:&Play,handsinfo:&HandsInformation)->String{
            //give max point
            //avoid using trump card
            //check sonenet has cards or not
            //reveal trump if it wasn't you
            let result=pridict_winning_player(&payload.played, gamedetails);
            if gamedetails.suits.contains(&(gamedetails.this_hand_suit)){
               return throwcard(mycards.get_card(gamedetails.this_hand_suit,true));   
            }
            if !gamedetails.suits.contains(&(gamedetails.this_hand_suit))&&knowledge.no_possibility_of_trump_reveal(gamedetails.this_hand_suit, 0)&&!gamedetails.trump_revealed_in_this_hand&&!gamedetails.trump_revealed_by_you&&!gamedetails.trump_revealed{
                if payload.played.len() as u8==2&&knowledge.card_greater_than_this_rank_card_exist(card_mapto_key(result.1.as_bytes()[0] as char),gamedetails.this_hand_suit){
                    if !knowledge.check_played_card(64, gamedetails.this_hand_suit)||!knowledge.check_played_card(128, gamedetails.this_hand_suit){
                        return reveal_trump();
                    }
                }
                //if i have this hand suit and my team is winning
                    return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
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
                                return reveal_trump_play_card(get_the_winning_trump(&payload, gamedetails, &mycards));
                            }
                            else{
                                return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                            }
                        }
                        //other player has card rank lesser than my partner
                    else{
                        //throw other card and check for points
                        //check duita suits.. yedi trump_suit hoina ra 
                        //aruko trump card sakkauney kaam garnu paryo
                        if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                            if result.1.as_bytes()[1] as char!=gamedetails.trump_suit&&gamedetails.sum_of_points<2{
                                return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                            }
                            // check if you have the winning suits
                            if knowledge.check_played_card(128, gamedetails.trump_suit) &&mycards.you_have_this_card(128,gamedetails.trump_suit){
                                //if i have the J card... throw it
                                return reveal_trump_play_card(get_the_winning_trump(&payload, gamedetails, &mycards));
                             }
                            else if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.trump_suit),gamedetails.trump_suit){
                                return reveal_trump_play_card(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                            }
                            else{
                                return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                            }
                            
                        }
                        else{
                            //throw some random minimum card
                            return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                        }
                    }
                }
                else{
                    //no more this hand suit card left .. all used..
                    if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                        if result.1.as_bytes()[1] as char!=gamedetails.trump_suit&&gamedetails.sum_of_points<2{
                            return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                        }
                        //you have trump suits
                        // check if you have the winning suits
                        if knowledge.check_played_card(128, gamedetails.trump_suit) &&mycards.you_have_this_card(128,gamedetails.trump_suit){
                            return reveal_trump_play_card(get_the_winning_trump(&payload, gamedetails, &mycards));
                        }
                        else if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.trump_suit),gamedetails.trump_suit){
                            return reveal_trump_play_card(get_the_winning_trump(&payload, gamedetails, &mycards));
                        }
                        else{
                            return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                        }
                        
                    }
                    else{
                        //you don't have a trump suit catds..
                        return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                    }
                }
            }
 //......................TRUMP REVEALED.................................           
            //trump has been revealed
        else {
            //i am out of this hand suit..
            if gamedetails.trump_revealed_in_this_hand && gamedetails.trump_revealed_by_you{
                if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                    if result.1.as_bytes()[1] as char==gamedetails.trump_suit&&mycards.you_have_the_higher_rank_card(card_mapto_key(result.1.as_bytes()[0]as char), gamedetails.trump_suit){
                        return throwcard(mycards.get_card_just_greater_than_this(card_mapto_key(result.1.as_bytes()[0] as char),gamedetails.trump_suit));
                    }
                    return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                }
                //if trump revealed in this hand and it was me.
                return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
            }
            if knowledge.get_total_cards_not_played(gamedetails.this_hand_suit)>=1{
                //if any this hand suit cards left to play with the opponenents
                if knowledge.card_greater_than_this_rank_card_exist(gamedetails.partner_card, gamedetails.this_hand_suit){
                    if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                        //throw max trump card
                        return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                    }
                    else{
                        //throw any random minimim card
                        return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                    }
                }
                else{
                    return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                }
            }
            else {
                //this hand suit card over... opponent will also throw trump card.. so throw max..
                //throw either trump or any random minimum card
                //check if opponent has any trump card left
                if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                    if !handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4,gamedetails.this_hand_suit)&&result.1.as_bytes()[1] as char!=gamedetails.trump_suit{
                        //yedi mah sanga trump card chha bhaney throw it
                        //throw card that maximizes points
                        return throwcard(mycards.get_card(gamedetails.trump_suit,false));
                    }
                    // check if you have the winning suits
                    if knowledge.check_played_card(128, gamedetails.trump_suit) &&mycards.you_have_this_card(128,gamedetails.trump_suit){
                        return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                    }
                    else if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.trump_suit),gamedetails.trump_suit){
                        return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                    }
                    else{
                        return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                    }
                    
                }
                else{
                    //throw any random minimim card
                    return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                }
            }
        }
    }
    fn get_the_winning_trump(payload:&Play,gamedetails:&mut GameDetails,mycards:&MyCARDS)->String{
        gamedetails.trump_revealed=true;
        let winner:(u8,String)=pridict_winning_player(&payload.played, &gamedetails);
        if winner.1.as_bytes()[1] as char==gamedetails.trump_suit{
            return mycards.get_card_just_greater_than_this(card_mapto_key( winner.1.as_bytes()[0] as char),gamedetails.trump_suit);
        }
        else{
            return mycards.get_card(gamedetails.trump_suit,false).clone();
        }
    }
    fn throw_min(mycards:&MyCARDS,gamedetails:&mut GameDetails,knowledge:&Knowledge,payload:&Play,handsinfo:&HandsInformation)->String{
        let result:(u8,String)=pridict_winning_player(&payload.played, &gamedetails);
        if gamedetails.suits.contains(&(gamedetails.this_hand_suit)){
            //if i have this hand suit and my team is winning
            if mycards.get_card_left(gamedetails.this_hand_suit)==1|| (result.1.as_bytes()[1] as char==gamedetails.this_hand_suit&&mycards.you_have_the_higher_rank_card(card_mapto_key(result.1.as_bytes()[0]as char), gamedetails.this_hand_suit)){
                if payload.played.len() as u8==2{
                    if knowledge.card_greater_than_this_rank_card_exist(card_mapto_key(mycards.get_card(gamedetails.this_hand_suit,true).as_bytes()[0]as char),gamedetails.this_hand_suit){
                        return throwcard(mycards.get_card(gamedetails.this_hand_suit,true));
                    }   
                }
                return throwcard(mycards.get_card_just_greater_than_this(card_mapto_key( result.1.as_bytes()[0] as char),gamedetails.this_hand_suit));
            }
            return throwcard(mycards.get_card(gamedetails.this_hand_suit,false));
        }
        else if !gamedetails.trump_revealed{
                //trump not revealed
                if gamedetails.i_won_the_bid{
                    if knowledge.get_total_cards_not_played(gamedetails.this_hand_suit)>=1{
                        if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                            if result.1.as_bytes()[1] as char!=gamedetails.trump_suit&&gamedetails.sum_of_points<2{
                                return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                            }
                            //throw max trump card
                            return reveal_trump_play_card(get_the_winning_trump(&payload, gamedetails, &mycards));
                        }
                        else{
                            //throw any random minimim card
                            return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                        } 
                    }
                    else{
                        return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                    }
                }
                else{
                    return reveal_trump();
                }
            }
        else{
            //trump revealed
            //i am out of this hand suit..
            if gamedetails.trump_revealed_in_this_hand && gamedetails.trump_revealed_by_you{
                if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                    if !handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, gamedetails.this_hand_suit)&&result.1.as_bytes()[1] as char!=gamedetails.trump_suit{
                        //yedi mah sanga trump card chha bhaney throw it
                        //throw card that maximizes points
                        return throwcard(mycards.get_card(gamedetails.trump_suit,false));
                    }
                    if result.1.as_bytes()[1] as char==gamedetails.trump_suit&&mycards.you_have_the_higher_rank_card(card_mapto_key(result.1.as_bytes()[0]as char), gamedetails.trump_suit){
                        return throwcard(mycards.get_card_just_greater_than_this(card_mapto_key(result.1.as_bytes()[0] as char),gamedetails.trump_suit));
                    }
                    return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                }
                else if !knowledge.card_greater_than_this_rank_card_exist(gamedetails.partner_card, gamedetails.this_hand_suit) && knowledge.get_total_cards_not_played(gamedetails.this_hand_suit)>=1&&payload.played.len() as u8==2{
                    return throwcard(mycards.get_card(*gamedetails.suits.last().unwrap(),true));
                }
                else{
                    return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                }
            }
            if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                if !handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, gamedetails.this_hand_suit)&&result.1.as_bytes()[1] as char!=gamedetails.trump_suit{
                    //yedi mah sanga trump card chha bhaney throw it
                    //throw card that maximizes points
                    return throwcard(mycards.get_card(gamedetails.trump_suit,false));
                }
                if result.1.as_bytes()[1] as char!=gamedetails.trump_suit&&gamedetails.sum_of_points<2{
                    return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                }
                if payload.played.len() as u8==2 && knowledge.no_possibility_of_trump_reveal(gamedetails.this_hand_suit,0){
                    return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                }
                if payload.played.len()as u8==3{
                    if result.1.as_bytes()[1] as char!=gamedetails.trump_suit{
                        return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                    }
                    else if mycards.you_have_the_higher_rank_card(card_mapto_key(result.1.as_bytes()[0]as char), gamedetails.trump_suit){
                        return throwcard(mycards.get_card_just_greater_than_this(card_mapto_key( result.1.as_bytes()[0] as char),gamedetails.trump_suit));
                    }
            }
                //throw max trump card//throw max trump card
                let result=pridict_winning_player(&payload.played, &gamedetails);
                if result.1.as_bytes()[1] as char==gamedetails.trump_suit{
                    if !mycards.you_have_the_higher_rank_card(card_mapto_key(result.1.as_bytes()[0] as char),gamedetails.trump_suit){
                        return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                    }
                    else{
                        return throwcard(mycards.get_card(gamedetails.trump_suit,true));
                    }
                }
                if knowledge.check_played_card(128, gamedetails.trump_suit) &&mycards.you_have_this_card(128,gamedetails.trump_suit){
                    return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                }
                if knowledge.check_played_card(128, gamedetails.trump_suit) &&!mycards.you_have_this_card(128,gamedetails.trump_suit){
                    if payload.played.len() as u8==3 &&knowledge.check_played_card(64, gamedetails.trump_suit) &&mycards.you_have_this_card(64,gamedetails.trump_suit){
                        return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                    }
                    if knowledge.no_possibility_of_trump_reveal(gamedetails.this_hand_suit,0){
                        return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                    }
                    return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                }
                else if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.trump_suit),gamedetails.trump_suit){
                    return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
                }
                else if check_any_trump_card_played_in_this_hand(&payload.played,gamedetails.trump_suit){
                    for i in payload.played.iter(){
                        if i.as_bytes()[1] as char==gamedetails.trump_suit{
                            if card_mapto_key(i.as_bytes()[0] as char)>mycards.get_first_card_of_given_suit(gamedetails.trump_suit){
                                return throwcard(mycards.get_card(gamedetails.trump_suit,false));
                            }
                        }
                    }
                    return throwcard(mycards.get_card(gamedetails.trump_suit,true));
                }
                else{
                    //throw any random card
                    return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
                }
            }
            else{
                //throw any random minimim card
                return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
            }

        }
    }
    fn make_first_move(payload:&Play,mycards:&MyCARDS,knowledge:&Knowledge,gamedetails:&mut GameDetails,handsinfo:&HandsInformation)->String{
        //make getting point strategy 
        //yedi trump cards chha bhaney tyo nafaalney.. in the beginning.. sakdo try opponeent ko trump card sakkauna
        //if no point getting card.. throw card with min num of suits
        //see if the cards has high rank
        //see if you have the high rank card
        let mut run_out_suits:Vec<char>=Vec::new();
        if gamedetails.suits.len() as u8==1{
            if mycards.get_card_left(gamedetails.suits[0])<=2{
                if !knowledge.card_greater_than_this_rank_card_exist(mycards.get_first_card_of_given_suit(gamedetails.suits[0]), gamedetails.suits[0]){
                    return throwcard(mycards.get_card(gamedetails.suits[0],true));
                }
                return throwcard(mycards.get_card(gamedetails.suits[0],false));
            }
            let _root=Rc::new(RefCell::new(MCTSTwentyNineGameTreeNode::init()));
                _root.as_ref().borrow_mut().state=Some(payload.played.to_owned());
                _root.as_ref().borrow_mut().expand_tree(&payload.cards,Rc::clone(&_root));
                    for _ in 0..num_of_simulation(payload.cards.len() as u8){
                        let node=_root.borrow().select_node();
                        //let mut node_ref=node.borrow();
                        let winner_id=node.borrow().rollout(&knowledge, &mycards, gamedetails, &handsinfo);
                        node.as_ref().borrow_mut().backpropagate(winner_id, gamedetails.playerid);
                    }
            let best_score_node=_root.borrow().best_score_node();
                let worst_score_node=_root.borrow().best_score_node();
                if best_score_node.as_ref().borrow().wins>0{
                    return throwcard(best_score_node.borrow().get_best_score_card(0 as u8));
                }
                return throwcard(worst_score_node.borrow().get_best_score_card(0 as u8));
        }
        for i in gamedetails.suits.iter(){
            //use point getting cards first.
            //don't throw trump cards and point getting cards of trump suit
            if gamedetails.trump_suit==*i{
                continue;
            }
            if handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, *i)||handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+3)%4, *i)||probability_that_this_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, &payload, *i, &gamedetails)||probability_that_this_player_ran_out_of_this_suit_cards((gamedetails.playerid+3)%4, &payload, *i, &gamedetails){
                    run_out_suits.push(*i);
                    continue;

                //
            }
            match i{
                    'H'=>{
                        let key=mycards.H[0];
                        match key{
                            //try to get the point
                            128=>{
                                return throwcard(mycards.map_key_to_card(key,'H'))
                            },
                            _=>{
                                //throw less suit cards
                                continue;
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
                            _=>{
                                //throw less suit cards
                                continue;
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
                            _=>{
                                //throw less suit cards
                                continue;
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
                            _=>{
                                //throw less suit cards
                                continue;
                            },
                        }
                    },
                    _=>{},
                }
    }
    //you have point cards...  throw if it can get point
    //now no point getting cards left
    //try to remove lower cards for other suits cards
    //try to finish their trump suit
    //try to finish your non trump suits..
      //try to finish your non trump suits..
    //let trap_ten=mycards.get_trap_card_tens();
    if payload.handsHistory.len()as u8 >0{
        let last_play_result:(u8,String)=pridict_winning_player(&payload.handsHistory.last().unwrap().1,&gamedetails);
        if gamedetails.suits.contains(&(last_play_result.1.as_bytes()[1] as char)){
            if card_mapto_key(last_play_result.1.as_bytes()[0] as char)==128&& mycards.get_first_card_of_given_suit(last_play_result.1.as_bytes()[1] as char)==64{
                if knowledge.no_possibility_of_trump_reveal(last_play_result.1.as_bytes()[1] as char, mycards.get_card_left(last_play_result.1.as_bytes()[1] as char)){
                    return throwcard(mycards.get_card(last_play_result.1.as_bytes()[1] as char,true));
                }
        }
        if (gamedetails.trump_revealed||gamedetails.i_won_the_bid)&&mycards.non_point_card_exist(last_play_result.1.as_bytes()[1] as char)&&last_play_result.1.as_bytes()[1] as char!=gamedetails.trump_suit{
            return throwcard(mycards.get_card(last_play_result.1.as_bytes()[1] as char,false));
        }
    }
    }
    if run_out_suits.len()as u8!=0{
        //donot throw cards which carry points
        //throw cards which don't carry points so that you can finish your opponent trump cards
        for i in run_out_suits.iter(){
            if mycards.non_point_card_exist(*i){
                return throwcard(mycards.get_card(*i,false));
            }
            continue;
        }
    }
     let non_point_cards_and_tens_ones=mycards.tens_ones_and_non_point_cards(gamedetails.trump_suit);
     if non_point_cards_and_tens_ones.len() as u8 >1{
        println!("Mah yeha chhu 637");
        let _root=Rc::new(RefCell::new(MCTSTwentyNineGameTreeNode::init()));
        _root.as_ref().borrow_mut().state=Some(payload.played.to_owned());
         _root.as_ref().borrow_mut().expand_tree(&non_point_cards_and_tens_ones,Rc::clone(&_root));
                for _ in 0..num_of_simulation(non_point_cards_and_tens_ones.len() as u8){
                    let node=_root.borrow().select_node();
                    //let mut node_ref=node.borrow();
                    let winner_id=node.borrow().rollout(&knowledge, &mycards, gamedetails, &handsinfo);
                    node.as_ref().borrow_mut().backpropagate(winner_id, gamedetails.playerid);
                }
                let best_score_node=_root.borrow().best_score_node();
                let worst_score_node=_root.borrow().best_score_node();
                if best_score_node.as_ref().borrow().wins>0{
                    return throwcard(best_score_node.borrow().get_best_score_card(0 as u8));
                }
                return throwcard(worst_score_node.borrow().get_best_score_card(0 as u8));
     }
    let _root=Rc::new(RefCell::new(MCTSTwentyNineGameTreeNode::init()));
    _root.as_ref().borrow_mut().state=Some(payload.played.to_owned());
     _root.as_ref().borrow_mut().expand_tree(&payload.cards,Rc::clone(&_root));
            for _ in 0..num_of_simulation(payload.cards.len() as u8){
                let node=_root.borrow().select_node();
                //let mut node_ref=node.borrow();
                let winner_id=node.borrow().rollout(&knowledge, &mycards, gamedetails, &handsinfo);
                node.as_ref().borrow_mut().backpropagate(winner_id, gamedetails.playerid);
            }
            let best_score_node=_root.borrow().best_score_node();
            let worst_score_node=_root.borrow().worst_score_node();
            if best_score_node.as_ref().borrow().wins>0{
                return throwcard(best_score_node.borrow().get_best_score_card(0 as u8));
            }
            return throwcard(worst_score_node.borrow().get_best_score_card(0 as u8));
     
}
fn num_of_simulation(num_of_cards:u8)->u32{
    if num_of_cards==2{
        return 2000;
    }
    if num_of_cards==3{
        return 4000;
    }
    if num_of_cards==4{
        return 7000;
    }
    if num_of_cards==5{
        return 8000;
    }
    if num_of_cards==6{
        return 9000;
    }
    if num_of_cards==7{
        return 10000;
    }
    if num_of_cards==8{
        return 12000;
    }
    1000
}
    fn make_second_move(payload:&Play,mycards:&MyCARDS,knowledge:&Knowledge,gamedetails:&mut GameDetails,handsinfo:&HandsInformation)->String{
        let opponent_card_key:u8=card_mapto_key(payload.played[0].as_bytes()[0] as char);
        if gamedetails.suits.contains(&(gamedetails.this_hand_suit)){
            if mycards.you_have_the_higher_rank_card(opponent_card_key, gamedetails.this_hand_suit){
                return throwcard(mycards.get_card(gamedetails.this_hand_suit,true));
            }
            return throwcard(mycards.get_card(gamedetails.this_hand_suit,false));
        }
        //i am run out of this hand suit
        else if !gamedetails.trump_revealed && !gamedetails.i_won_the_bid{
            if gamedetails.sum_of_points>=2{
                return reveal_trump();
            }
            if !knowledge.check_played_card(64,gamedetails.this_hand_suit)||!knowledge.check_played_card(128,gamedetails.this_hand_suit) && !handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4,gamedetails.this_hand_suit){
                return reveal_trump();
            }
            if knowledge.no_possibility_of_trump_reveal(gamedetails.this_hand_suit,0){
                if !knowledge.check_played_card(64,gamedetails.this_hand_suit)||!knowledge.check_played_card(128,gamedetails.this_hand_suit){
                    return reveal_trump();
                }
            }
            return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
        }
        else if !gamedetails.trump_revealed && gamedetails.i_won_the_bid{
                //i am run out of this hand suit
                if gamedetails.suits.contains(&(gamedetails.trump_suit)){
                    if gamedetails.sum_of_points>=2{
                        return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit,false));
                    }
                    if !knowledge.check_played_card(64,gamedetails.this_hand_suit)||!knowledge.check_played_card(128,gamedetails.this_hand_suit) && !handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4,gamedetails.this_hand_suit){
                        return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit,false));
                    }
                    if knowledge.no_possibility_of_trump_reveal(gamedetails.this_hand_suit,0)&&(!knowledge.check_played_card(64,gamedetails.this_hand_suit)||!knowledge.check_played_card(128,gamedetails.this_hand_suit)){
                            return reveal_trump_play_card(mycards.get_card(gamedetails.trump_suit,false));
                    }
                }
                    //messa doesnot have a trump
                    //check mero opponent sanga yo hand ko card chha ki nai..
                     //throw any random minimim card
                    return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
    
        }
        //trumpRevealed part......
        else if gamedetails.suits.contains(&(gamedetails.trump_suit)){
            if !handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, gamedetails.this_hand_suit)&&knowledge.no_possibility_of_trump_reveal(gamedetails.this_hand_suit,0){
                //yedi mah sanga trump card chha bhaney throw it
                //throw card that maximizes points
                return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
            }
             return throwcard(mycards.map_key_to_card(get_trump_card_that_maximizes(&mycards, &gamedetails, &knowledge),gamedetails.trump_suit));
            }
        else{
            return throwcard(get_random_card(gamedetails, &mycards, &payload, &knowledge, &handsinfo));
            }
            
    }
    fn make_knowledge(players:&Vec<String>,knowledge:&mut Knowledge,handshistory:&Vec<(String,Vec<String>,String)>,gamedetails:&mut GameDetails,handsinfo:&mut HandsInformation){
          //update last hand winner
          gamedetails.last_hand_winner=map_string_playerid_to_number(&players,&(handshistory.last().unwrap().2).clone());
          let mut hand:u8=0;
        for i in handshistory{
            let this_hand_winner:(u8,String)=pridict_winning_player(&i.1, &gamedetails);
            knowledge.update_knowledge(&i.1);
            handsinfo.update_hands_info(hand,&i.1[0],&this_hand_winner.1);
            hand+=1;
        }
    }
    fn update_hands_history_in_handsinfo(hands_info:&mut HandsInformation,payload:&Play){
        for i in payload.handsHistory.iter(){
            hands_info.handhistory.push((map_string_playerid_to_number(&payload.playerIds,&i.0),i.1.to_owned(),map_string_playerid_to_number(&payload.playerIds,&i.2)));
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
    fn check_any_trump_card_played_in_this_hand(played:&Vec<String>,suit:char)->bool{
        for i in played.iter(){
            if i.as_bytes()[1] as char==suit{
                return true;
            }
        }
        false
    }
    fn arrange_suits_from_max(mycards:&MyCARDS)->Vec<char>{
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
    fn arrange_suits_from_min(mycards:&MyCARDS)->Vec<char>{
        let mut data:Vec<(char,u8)>=Vec::new();
            if mycards.H.len() as u8>0{
                data.push(('H',*mycards.H.last().unwrap()));
            }
            if mycards.D.len() as u8>0{
                data.push(('D',*mycards.D.last().unwrap()));
            }
            if mycards.S.len() as u8>0{
                data.push(('S',*mycards.S.last().unwrap()));
            }
            if mycards.C.len()as u8>0{
                data.push(('C',*mycards.C.last().unwrap()));
            }
            if data.len() as u8>=2{
                for i in 0..data.len(){
                    let mut temp:(char,u8)=data[i];
                    for j in (i+1)..data.len(){
                        if data[j].1<temp.1{
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
    fn pridict_winning_player(played:&Vec<String>,gamedetails:&GameDetails)->(u8,String){
        //if winner team is yours.. maximize the point. else minimize
        //return possible winner and thrown card
            let mut possible_winner:u8=gamedetails.last_hand_winner;
            let mut thrown_by:u8=gamedetails.last_hand_winner;
            let mut winner_rank_point:(u8,u8)=gamedetails.card_map_to_rank_point[&(played[0].as_bytes()[0] as char)];
            let mut winning_suit:char=gamedetails.this_hand_suit;//if trump card. change it to trump_suit
            let mut thrown_card:&String=&played[0];
            for i in played[1..played.len()].iter(){
                thrown_by= (thrown_by+1)%4;
                let played_suit=i.as_bytes()[1] as char;
                let rank_point:(u8,u8)=gamedetails.card_map_to_rank_point[&(i.as_bytes()[0] as char)];
                if winning_suit==played_suit && rank_point.0<winner_rank_point.0{
                    winner_rank_point=rank_point;
                    possible_winner=thrown_by;
                    thrown_card=i;
                }
                else if played_suit==gamedetails.trump_suit{
                    if winning_suit!=gamedetails.trump_suit{
                        winning_suit=gamedetails.trump_suit;
                        winner_rank_point=rank_point;
                        possible_winner=thrown_by; 
                        thrown_card=i;
                    }
                    else if rank_point.0<winner_rank_point.0 {
                        winner_rank_point=rank_point;
                        possible_winner=thrown_by;
                        thrown_card=i;
                    } 
                }   
            }
            (possible_winner,thrown_card.to_string())
    }
    fn get_total_points(played:&Vec<String>,card_map_to_rank_points:&HashMap<char,(u8,u8)>)->u8{
        let mut sum=0;
        for i in played.iter(){
            sum+=card_map_to_rank_points[&(i.as_bytes()[0] as char)].1;
        }
        sum
    }
    fn get_suits_that_has_the_possibility_of_trump_reveal(suits:&Vec<char>,knowledge:&Knowledge,mycards:&MyCARDS)->Vec<char>{
        let mut suit:Vec<char>=Vec::new();
        for i in suits.iter(){
            if !knowledge.no_possibility_of_trump_reveal(*i, mycards.get_card_left(*i)){
                suit.push(*i);
            }
        }
        suit
    }
    fn get_random_card(gamedetails:&mut GameDetails,mycards:&MyCARDS,payload:&Play,knowledge:&Knowledge,handsinfo:&HandsInformation)->String{
        let mut _point_cards=mycards.get_point_cards(); 
        let mut tens_ones_and_non_point_cards=mycards.tens_ones_and_non_point_cards(gamedetails.trump_suit);
        let low_grade_suits=get_suits_that_has_the_possibility_of_trump_reveal(&gamedetails.suits, &knowledge, &mycards);
        let get_rand_cards=mycards.get_random_cards(gamedetails.trump_suit,&knowledge);
        if get_rand_cards.len() as u8!=0{
            return get_rand_cards.last().unwrap().clone();
        }
        if _point_cards.len() as u8==0{
            _point_cards=payload.cards.to_owned();
        }
        if tens_ones_and_non_point_cards.len() as u8==0{
            tens_ones_and_non_point_cards=payload.cards.to_owned();
        }
        if gamedetails.we_are_winning{
            //we are winning
            if payload.played.len() as u8==2&&knowledge.no_possibility_of_trump_reveal(gamedetails.this_hand_suit, 0)&&gamedetails.sum_of_points>=2{
                if low_grade_suits.len() as u8!=0{
                    return mycards.get_card(low_grade_suits[0],true);
                }
                for i in _point_cards.iter(){
                    if i.as_bytes()[1] as char==gamedetails.trump_suit{
                        continue;
                    }
                    if !knowledge.no_possibility_of_trump_reveal(i.as_bytes()[1]as char, mycards.get_card_left(i.as_bytes()[1]as char)){
                        return i.clone();
                    }
                    if handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+3)%4, i.as_bytes()[1]as char)||handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, i.as_bytes()[1]as char){
                        return i.clone();
                    }
                    if handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, gamedetails.this_hand_suit)&&handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, gamedetails.trump_suit){
                        return i.clone();
                    }
                }
                for i in _point_cards.iter(){
                    if i.as_bytes()[1] as char==gamedetails.trump_suit{
                        continue;
                    }
                    if card_mapto_key(i.as_bytes()[0] as char)==128 &&!knowledge.check_played_card(64,i.as_bytes()[1] as char){
                        continue;
                    }
                    if card_mapto_key(i.as_bytes()[0] as char)==64 &&!knowledge.check_played_card(128,i.as_bytes()[1] as char){
                        return i.to_string();
                    }
                }
            }
            if payload.played.len() as u8==3{
                if low_grade_suits.len() as u8!=0{
                    return mycards.get_card(low_grade_suits[0],true);
                }
                for i in _point_cards.iter(){
                    if i.as_bytes()[1] as char==gamedetails.trump_suit{
                        continue;
                    }
                    if handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+3)%4, i.as_bytes()[1]as char)||handsinfo.any_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, i.as_bytes()[1]as char){
                        return i.clone();
                    }
                }
            }
            if payload.played.len() as u8==3&&gamedetails.sum_of_points>=2{
                if low_grade_suits.len() as u8!=0{
                    return mycards.get_card(low_grade_suits[0],true);
                }
                for i in _point_cards.iter(){
                    if i.as_bytes()[1] as char==gamedetails.trump_suit{
                        continue;
                    }
                    if card_mapto_key(i.as_bytes()[0] as char)==128 &&!knowledge.check_played_card(64,i.as_bytes()[1] as char){
                        continue;
                    }
                    if card_mapto_key(i.as_bytes()[0] as char)==64 &&!knowledge.check_played_card(128,i.as_bytes()[1] as char){
                        return i.to_string();
                    }
                }
            }
            if tens_ones_and_non_point_cards.len()as u8>=4{
                   let _root=Rc::new(RefCell::new(MCTSTwentyNineGameTreeNode::init()));
                   _root.as_ref().borrow_mut().state=Some(vec![].to_owned());
                   _root.as_ref().borrow_mut().expand_tree(&tens_ones_and_non_point_cards,Rc::clone(&_root));
                   for _ in 0..num_of_simulation(tens_ones_and_non_point_cards.len() as u8){
                        let node=_root.borrow().select_node();
                        //let mut node_ref=node.borrow();
                        let winner_id=node.borrow().rollout(&knowledge, &mycards, gamedetails, &handsinfo);
                        node.as_ref().borrow_mut().backpropagate(winner_id, gamedetails.playerid);
                    }
                    let worst_score_node=_root.borrow().worst_score_node();
                    return worst_score_node.borrow().get_best_score_card(0 as u8);

            }
            if tens_ones_and_non_point_cards.len() as u8!=0{
                for i in tens_ones_and_non_point_cards.iter(){
                    if payload.played.len() as u8==2{
                        if knowledge.card_greater_than_this_rank_card_exist(card_mapto_key( i.as_bytes()[0] as char), i.as_bytes()[1] as char) && probability_that_this_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, &payload, i.as_bytes()[1] as char,&gamedetails)&&knowledge.get_total_cards_not_played(i.as_bytes()[1] as char)<=1{
                            return i.to_string();
                        }
                    }
                    if payload.played.len() as u8==3&& knowledge.get_total_cards_not_played(i.as_bytes()[1] as char)<=1{
                        return i.to_string();
                    }
                    
                }
                return mycards.get_card(gamedetails.suits_arrange_from_min[0],false);
            }
            else{

                for i in gamedetails.suits_arrange_from_min.iter(){
                    if *i ==gamedetails.trump_suit{
                        continue;
                    }
                    return mycards.get_card(*i,false);
                }
                return mycards.get_card(gamedetails.suits_arrange_from_min[0],false);
            }
        }
            // we are losing
            if low_grade_suits.len()as u8!=0{
                for i in low_grade_suits.iter(){
                    if mycards.non_point_card_exist(*i){
                        return mycards.get_card(*i, false);
                    }
                }
            }
            if tens_ones_and_non_point_cards.len() as u8>=2{
            let _root=Rc::new(RefCell::new(MCTSTwentyNineGameTreeNode::init()));
            _root.as_ref().borrow_mut().state=Some(vec![].to_owned());
            _root.as_ref().borrow_mut().expand_tree(&tens_ones_and_non_point_cards,Rc::clone(&_root));
            for _ in 0..num_of_simulation(tens_ones_and_non_point_cards.len() as u8){
                 let node=_root.borrow().select_node();
                 //let mut node_ref=node.borrow();
                 let winner_id=node.borrow().rollout(&knowledge, &mycards,gamedetails, &handsinfo);
                 node.as_ref().borrow_mut().backpropagate(winner_id, gamedetails.playerid);
             }
             let worst_score_node=_root.borrow().worst_score_node();
             return worst_score_node.borrow().get_best_score_card(0 as u8); //we are losing
        }
        else{
             if tens_ones_and_non_point_cards.len() as u8!=0{
                for i in tens_ones_and_non_point_cards.iter(){
                    if payload.played.len() as u8==2{
                        if knowledge.card_greater_than_this_rank_card_exist(card_mapto_key( i.as_bytes()[0] as char), i.as_bytes()[1] as char) && probability_that_this_player_ran_out_of_this_suit_cards((gamedetails.playerid+1)%4, &payload, i.as_bytes()[1] as char,&gamedetails)&&knowledge.get_total_cards_not_played(i.as_bytes()[1] as char)<=1{
                            return i.to_string();
                        }
                    }
                    if payload.played.len() as u8==3&& knowledge.get_total_cards_not_played(i.as_bytes()[1] as char)<=1{
                        return i.to_string();
                    }
                    
            }
        }
        return tens_ones_and_non_point_cards.last().unwrap().to_string();
        }
} 
    fn get_trump_card_that_maximizes(mycards:&MyCARDS,gamedetails:&GameDetails,knowledge:&Knowledge)->u8{
        match gamedetails.trump_suit{
            'H'=>{
                if mycards.get_card_left(gamedetails.trump_suit)==1{
                    return mycards.H[0];
                }
                for i in mycards.H.iter(){
                    //println!("{}",i);
                    match *i{
                        64=>{
                            if (gamedetails.sum_of_points+2)>=3 ||(knowledge.card_greater_than_this_rank_card_exist(64, 'H')&& !mycards.you_have_this_card(128, 'H')){
                                return 64;
                            }
                        },
                        32|16=>{
                            if (gamedetails.sum_of_points+1)>=2&&!mycards.non_point_card_exist(gamedetails.trump_suit){
                                if *i==32{
                                    return 32;
                                }
                                else{
                                    return 16;
                                }
                            }
                        },
                        _=>{
                            return *mycards.H.last().unwrap();
                        },
                    }
                }
            },
            'D'=>{
                if mycards.get_card_left(gamedetails.trump_suit)==1{
                    return mycards.D[0];
                }
                for i in mycards.D.iter(){
                    match i{
                        64=>{
                            if (gamedetails.sum_of_points+2)>=3 ||(knowledge.card_greater_than_this_rank_card_exist(64, 'D')&& !mycards.you_have_this_card(128, 'D')){
                                return 64;
                            }
                        },
                        32|16=>{
                            if (gamedetails.sum_of_points+1)>=2&&!mycards.non_point_card_exist(gamedetails.trump_suit){
                                if *i==32{
                                    return 32;
                                }
                                else{
                                    return 16;
                                }
                            }
                        },
                        _=>{
                            return *mycards.D.last().unwrap();
                        },
                    }
                }
            },
            'C'=>{
                if mycards.get_card_left(gamedetails.trump_suit)==1{
                    return mycards.C[0];
                }
                for i in mycards.C.iter(){
                    match i{
                        64=>{
                            if (gamedetails.sum_of_points+2)>=3 ||(knowledge.card_greater_than_this_rank_card_exist(64, 'C')&& !mycards.you_have_this_card(128, 'C')){
                                return 64;
                            }
                        },
                        32|16=>{
                            if (gamedetails.sum_of_points+1)>=2&&!mycards.non_point_card_exist(gamedetails.trump_suit){
                                if *i==32{
                                    return 32;
                                }
                                else{
                                    return 16;
                                }
                            }
                        },
                        _=>{
                            return *mycards.C.last().unwrap();
                        },
                    }
                }
            }
            'S'=>{
                if mycards.get_card_left(gamedetails.trump_suit)==1{
                    return mycards.S[0];
                }
                for i in mycards.S.iter(){
                    match i{
                        64=>{
                            if (gamedetails.sum_of_points+2)>=3 ||(knowledge.card_greater_than_this_rank_card_exist(64, 'S')&& !mycards.you_have_this_card(128, 'S')){
                                return 64;
                            }
                        },
                        32|16=>{
                            if (gamedetails.sum_of_points+1)>=2 &&!mycards.non_point_card_exist(gamedetails.trump_suit){
                                if *i==32{
                                    return 32;
                                }
                                else{
                                    return 16;
                                }
                            }
                        },
                        _=>{
                            return *mycards.S.last().unwrap();
                        },
                    }
                }
            },
            _=>(),
        }
        //println!("Reached here!");
        card_mapto_key(mycards.get_card(gamedetails.trump_suit,false).as_bytes()[0] as char)
    }
    fn update_players_suits_info(handsinfo:&mut HandsInformation,playerids:&Vec<String>,handshistory:&Vec<(String,Vec<String>,String)>){
        //update that if any players run out of any suits
            for i in handshistory.iter(){
                let mut played_by:u8=map_string_playerid_to_number(&playerids, &i.0);
                let this_hand_played_suit:char=i.1[0].as_bytes()[1]as char;//this previous hand suit
                for j in i.1.iter(){
                    //now if played hand suit is not winner suit... then we conclde that this players is run out of this suit
                    if j.as_bytes()[1] as char !=this_hand_played_suit{
                    //this means player is ran out of this suit
                        //update hands information
                        handsinfo.update_suits_info_of_players(played_by,this_hand_played_suit);
                    }
                    played_by=(played_by+1)%4;
            }
        }
    }
    fn probability_that_this_player_ran_out_of_this_suit_cards(player:u8,payload:&Play,suit:char,gamedetails:&GameDetails)->bool{
        //players: opponenet left and right and my partner
        let mut played_card_by_this_player_in_this_hand=String::new();
        let mut next_player:u8;
        if payload.handsHistory.len()==0{
            return false;
        }
        for i in payload.handsHistory.iter(){
            let winner_id=map_string_playerid_to_number(&payload.playerIds,&i.2);
            let mut winning_card:String=String::new();
            if i.1[0].as_bytes()[1] as char==suit{
                //if that hand was playedd with this suit
                let mut thrown_by:u8=map_string_playerid_to_number(&payload.playerIds,&i.0);
                for k in i.1.iter(){
                    if winner_id==thrown_by{
                        winning_card=k.clone();
                    }
                    thrown_by=(thrown_by+1)%4;
                }
                thrown_by=map_string_playerid_to_number(&payload.playerIds,&i.0);
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
                if winner_id!=(gamedetails.playerid+2)%4{
                    if winning_card.as_bytes()[0] as char=='J' && (played_card_by_this_player_in_this_hand.as_bytes()[0] as char=='7'||played_card_by_this_player_in_this_hand.as_bytes()[0] as char=='8'||played_card_by_this_player_in_this_hand.as_bytes()[0] as char=='Q'||played_card_by_this_player_in_this_hand.as_bytes()[0] as char=='K'){
                        return true;
                    }
                }
                if card_mapto_key(played_card_by_this_player_in_this_hand.as_bytes()[0]as char)==64&&card_keys.contains(&128){
                    thrown_by=map_string_playerid_to_number(&payload.playerIds,&i.0);
                    if (gamedetails.playerid==(thrown_by+2)%4)||(gamedetails.playerid==thrown_by){
                        //it is thrown by my team.
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