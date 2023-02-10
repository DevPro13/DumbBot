use std::collections::HashMap;
use super::choosetrump::Trump;
#[derive(Clone,Default,Debug)]
pub struct CountHighestRankCards{
    cards:HashMap<char,u8>,
}
use super::api_rust_data::{InBid,Bid};
impl CountHighestRankCards{
    pub fn init_count(&mut self)->CountHighestRankCards{
        CountHighestRankCards{
            cards:HashMap::from([
                ('J',0),
                ('9',0),
                ('T',0),
                ('1',0),
            ]),
        }
    }
    fn count_highest_rank_cards(&mut self,cards:&Vec<String>){
        for  i in cards{
            let k=i.as_bytes()[0] as char;
            if k=='J' || k=='9'||k=='T'||k=='1'{
                self.cards.insert(k,self.cards[&k]+1);
            }
        }
    }
    fn check_atleast_one_present(&self,card:char)->bool{
        if self.cards[&card]>0{
            return true;
        }
        false
    }
    fn check_atleast_two_present(&self,card:char)->bool{
        if self.cards[&card]>1{
            return true;
        }
        false
    }
}
pub fn get_bid(bid_payload:&InBid)->Bid{
    fn check_all_players_pass(playerid:&String,players:&Vec<String>,bidhis:&Vec<(String,u8)>)->bool{
    (bidhis[0].0!=bidhis[2].0)&&( players[(players.iter().position(|r|r==&bidhis[2].0).unwrap()+2)%4]==*playerid )&& (bidhis[0].1==0&&bidhis[1].1==0&&bidhis[2].1==0)
   }
    let InBid{cards,bidState:in_bid_state,..}=bid_payload;
    let mut suits=Trump::init_trump_count(&mut Trump::default());
    suits.countsuits(&cards);
    let mut my_high_rank_cards=CountHighestRankCards::init_count(&mut CountHighestRankCards::default());
    my_high_rank_cards.count_highest_rank_cards(&cards);
    println!("my high rank cards: {:?}",my_high_rank_cards);
    //bidding decision starts here
    if bid_payload.bidHistory.len()==0 && can_get_max_bid(&my_high_rank_cards,&suits){
        return Bid{bid:16,};//pass minimum bid
    }
    if bid_payload.bidHistory.len()==3 && check_all_players_pass(&bid_payload.playerId,&bid_payload.playerIds,&bid_payload.bidHistory){
        //yedi sabai players ley suru mai pass gardai gayoo.. ra last ma you player matra baaki bho bhaney bid minimum
        return Bid{bid:16,};
    }
    //yedi def and chalenger is betn team.. one of them must pass to minimize max bid.
    let index = bid_payload.playerIds.iter().position(|playerid| *playerid == in_bid_state.defenderId).unwrap();
    if bid_payload.playerIds[(index+2)%4]==in_bid_state.challengerId{
        if in_bid_state.defenderBid==0 && in_bid_state.challengerBid==0&&can_get_max_bid(&my_high_rank_cards, &suits){
            return Bid{bid:16,};
        }
        return Bid{bid:0,};//return because i don't want to increase the bid
    }
    if can_get_max_bid(&my_high_rank_cards,&suits){
    if bid_payload.playerId==in_bid_state.defenderId{
        if in_bid_state.challengerBid>18{// i don't wanna go any further
            return Bid{bid:0,};
        }
         //check if 3 same suit present
         if suits.check_if_cards_has_three_same_suits() &&in_bid_state.challengerBid<=18{
                 return Bid{
                bid:in_bid_state.challengerBid,
            };
        }
       //if this true bid more than challenger bid
        if suits.check_if_cards_has_two_same_suits() && my_high_rank_cards.check_atleast_one_present('J')&& my_high_rank_cards.check_atleast_one_present('9') && in_bid_state.challengerBid<17{
            return Bid{
                bid:in_bid_state.challengerBid,
            };
        }
        //else bid equal
        if suits.check_if_cards_has_two_same_suits() && my_high_rank_cards.check_atleast_two_present('J')&& in_bid_state.challengerBid<17{
            return Bid{
                bid:in_bid_state.challengerBid,
            };
        }
   //if this true bid more than challenger bid
    if (suits.check_if_cards_has_three_same_suits()||suits.check_if_cards_has_two_same_suits()) && my_high_rank_cards.check_atleast_two_present('J')&& my_high_rank_cards.check_atleast_two_present('9')&&in_bid_state.challengerBid<17{
        return Bid{
            bid:in_bid_state.challengerBid,
        };
    }
    //yedi atleast duita js cards chha bhaney make challenge bid
    if my_high_rank_cards.check_atleast_two_present('J')&& (my_high_rank_cards.check_atleast_two_present('9')&&my_high_rank_cards.check_atleast_one_present('1')||my_high_rank_cards.check_atleast_one_present('T')) && in_bid_state.challengerBid<17{
        return Bid{
            bid:in_bid_state.challengerBid,
        };
    }
    //if two or more same suits and same bid less than or equals 17
    if in_bid_state.challengerBid<17&&suits.check_if_cards_has_three_same_suits()||(suits.check_if_cards_has_two_same_suits() && (my_high_rank_cards.check_atleast_one_present('J')&&my_high_rank_cards.check_atleast_two_present('9'))){
        return Bid{
            bid:in_bid_state.challengerBid,
        };
    }
    //if atleast 2 same suits and 2 J 1 9 1 T or 1
    if (suits.check_if_cards_has_two_same_suits()||suits.check_if_cards_has_three_same_suits())&&(my_high_rank_cards.check_atleast_two_present('J')&&my_high_rank_cards.check_atleast_one_present('9'))&&(my_high_rank_cards.check_atleast_one_present('1')||my_high_rank_cards.check_atleast_one_present('T'))&&in_bid_state.challengerBid<17{
        return Bid{
            bid:in_bid_state.challengerBid,
        };
    }
        //pass
        return Bid{
            bid:0,
        };
    }
    if bid_payload.playerId==in_bid_state.challengerId{
        //bid more or pass
        if in_bid_state.defenderBid==0 {
            //yedi defender bid 0 chha..bhaney..
            if in_bid_state.challengerBid==0&& can_get_max_bid(&my_high_rank_cards, &suits){
               return Bid{bid:16,};
            }
            else{
                 return Bid{bid:0,};
            }
        }
    if in_bid_state.defenderBid>=18{// i don't wanna go any further
        if suits.check_if_cards_has_three_same_suits()&&in_bid_state.defenderBid<=18{
            if my_high_rank_cards.check_atleast_two_present('J'){
                 return Bid{bid:in_bid_state.defenderBid+1,};
            }

        }
          return Bid{bid:0,};
}
//atleast 3 same suit  or 2 same suit with one J or 2 9
if suits.check_if_cards_has_three_same_suits()&&my_high_rank_cards.check_atleast_one_present('J'){
    if in_bid_state.defenderBid<=18{
    return Bid{
        bid:in_bid_state.defenderBid+1,
    };
}
return Bid{bid:0,};
}
    //atleast 3 same suit  or 2 same suit with one J or 2 9
    if (suits.check_if_cards_has_three_same_suits()||suits.check_if_cards_has_two_same_suits())&&my_high_rank_cards.check_atleast_one_present('J')&&my_high_rank_cards.check_atleast_two_present('9')&&in_bid_state.defenderBid<17{
        return Bid{
            bid:in_bid_state.defenderBid+1,
        };
    }
    println!("Mah yeha chhu 152");
   //if this true bid more than challenger bid
    if (suits.check_if_cards_has_three_same_suits()||suits.check_if_cards_has_two_same_suits()) && (my_high_rank_cards.check_atleast_one_present('J')|| my_high_rank_cards.check_atleast_two_present('9')) &&in_bid_state.defenderBid<17{
        return Bid{
            bid:in_bid_state.defenderBid+1,
        };
    }
    println!("Mah yeha chhu 159");
    //if atleast 2 J and 2 same suits bids
    if my_high_rank_cards.check_atleast_two_present('J') && suits.check_if_cards_has_two_same_suits() && in_bid_state.defenderBid<18{
        return Bid{
            bid:in_bid_state.defenderBid+1,
        };
    }
    println!("Mah yeha chhu 166");
     //if atleast 1 J and 1 9 and two same suits bids
     if my_high_rank_cards.check_atleast_one_present('J')&&my_high_rank_cards.check_atleast_two_present('9')  && (suits.check_if_cards_has_two_same_suits()||suits.check_if_cards_has_three_same_suits() ) && in_bid_state.defenderBid<17{
        return Bid{
            bid:in_bid_state.defenderBid+1,
        };
    }
    println!("Mah yeha chhu 173");
    //if atleast 2 same suits and 2 J 1 9 1 T or 1
     if in_bid_state.defenderBid<17&&(suits.check_if_cards_has_two_same_suits()||suits.check_if_cards_has_three_same_suits())&&(my_high_rank_cards.check_atleast_two_present('J')&&my_high_rank_cards.check_atleast_one_present('9'))&&(my_high_rank_cards.check_atleast_one_present('1')||my_high_rank_cards.check_atleast_one_present('T')){
        return Bid{
            bid:in_bid_state.defenderBid+1,
        };
    }
}
}
    Bid{bid:0,}//pass bid
}


///bidding decisions logics
fn can_get_max_bid(my_high_rank_cards:&CountHighestRankCards,suits:&Trump)->bool{

    if suits.check_if_cards_has_three_same_suits(){
        return true;
    }
    // if i have atleast 1 1 J and 9 cards.. I should bet minimum
    //if cards have 3 same suit cards
    if suits.check_if_cards_has_two_same_suits()&&my_high_rank_cards.check_atleast_one_present('J'){
        return true;
    }

    if my_high_rank_cards.check_atleast_one_present('J')&& my_high_rank_cards.check_atleast_one_present('9'){
        return true;
    }
     //if cards has 2 same suit and atleast one 9 or J
     if suits.check_if_cards_has_two_same_suits() && (my_high_rank_cards.check_atleast_one_present('J')|| my_high_rank_cards.check_atleast_two_present('9')){
        return true;
    }
    false

}