use std::collections::HashMap;
mod api_rust_data;
mod choosetrump;
#[derive(Clone,Default)]
pub struct CountHighestRankCards{
    card:HashMap<String,u8>,
}
use super::choosetrump::Trump;
use super::api_rust_data::{InBid,Bid,InBidState};
impl CountHighestRankCards{
    pub fn init_count(&mut self)->CountHighestRankCards{
        CountHighestRankCards{
            card:HashMap::from([
                "J".to_string():0,
                "9".to_string():0,
                "T".to_string():0,
                "1".to_string():0,
            ]);
        }
    }
    fn get_highest_rank_cards(&mut self,cards:&Vec<String>)&Vec<String,u8>{
        for  i in cards{
            let k=i.as_bytes()[0] as char;
            if k=='J' || k=='9'||k=='T'||k=='1'{
                self.card.insert(k.to_string(),self.card[&k.to_string()]+1);
            }
        }
        self.card
    }
    fn check_atleast_one_present(&self,card:String)->bool{
        if self.cards[card]>0{
            return true;
        }
        false
    }
    fn return_total_cards_of_given_rank(&self,card:String)->u8{
        self.cards[card]
    }
}

pub fn get_bid(bid_payload:&InBid)->Bid{
    let InBid{cards,InBidState,..}=InBid;
    let mut count_suits=Trump::init_trump_count(&mut moduleinrust::Trump::default());
    count_suits.countsuits(&cards);
    let counted_suits:HashMap<String,u8>=count_suits.ret_counted_suits();//return counted suits from your cards in hashmap from... eg  "H":2, "S":1

    let mut count_rank_cards=CountHighestRankCards::init_count(&mut moduleinrust::CountHighestRankCards::default());
    let high_rank_cards:&HashMap<String,u8>=count_rank_cards.get_highet_rank_cards(&cards);
    

    //bidding decision starts here
    if bid_payload.bidHistory.len()==0 && can_get_max_bid(&count_rank_cards,&count_suits){
        return Bid{bid:16,};//pass minimum bid
    }

    //yedi def and chalenger is betn team.. one of them must pass to minimize max bid.
    let index = bid_payload.players.iter().position(|playerid| *playerid == bid_payload.defenderId).unwrap();
    if bid_payload.players[(index+2)%4]==bid_payload.challengerId{
        return Bid{bid:0,};//return because i don't want to increase the bid
    }

    if can_get_max_bid(&count_rank_cards,&count_suits){
    if bid_payload.playerId==InBidState.defenderId{
        if InBidState.challengerBid>=18{// i don't wanna go any further
            return Bid{bid:0,};
        }
       //if this true bid more than challenger bid
        if (suits.check_if_cards_has_three_same_suits()||suits.check_if_cards_has_two_same_suits()) && mycards.check_atleast_one_present("J".to_string())&& mycards.check_atleast_one_present("9".to_string()){
            return Bid{
                bid:InBidState.challengerBid+1,
            };
        }
        //else bid equal
        if suits.check_if_cards_has_two_same_suits() && mycards.check_atleast_one_present("J".to_string()){
            return Bid{
                bid:InBidState.challengerBid,
            };
        }
        //pass
        return Bid{
            bid:0,
        };
    }
    if bid_payload.playerId==InBidState.challengerId{
        //bid more or pass
        if InBidState.defenderBid>=18{// i don't wanna go any further
        return Bid{bid:0,};
    }
   //if this true bid more than challenger bid
    if (suits.check_if_cards_has_three_same_suits()||suits.check_if_cards_has_two_same_suits()) && mycards.check_atleast_one_present("J".to_string())&& mycards.check_atleast_one_present("9".to_string()){
        return Bid{
            bid:InBidState.challengerBid+1,
        };
    }
}
}
    Bid{bid:0,}//pass bid
}


///bidding decisions logics
fn can_get_max_bid(mycards:&CountHighestRankCards,suits:&Trump)->bool{
    //if i have atleast 1 1 J and 9 cards.. I should bet minimum
    if mycards.check_atleast_one_present("J".to_string())&& mycards.check_atleast_one_present("9".to_string()){
        return true;
    }
    //if more than 2 9s, bet min
    if mycards.return_total_cards_of_given_rank("J".to_string())>1 || mycards.return_total_cards_of_given_rank("9".to_string())>1{
        return true;
    }
//if cards have 3 same suit cards
    if suits.check_if_cards_has_three_same_suits(){
        return true;
    }
    //if cards has 2 same suit and atleast one 9 and
    if suits.check_if_cards_has_two_same_suits() && mycards.check_atleast_one_present("J".to_string())&& mycards.check_atleast_one_present("9".to_string()){
        return true;
    }
    false

}
