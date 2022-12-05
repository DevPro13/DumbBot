use std::collections::HashMap;
mod api_rust_data;
mod choosetrump;
mod algorithm;
use super::algorithm::CountHighestRankCards;
use super::choosetrump::Trump;
use super::api_rust_data::{InBid,Bid,InBidState};
pub impl CountHighestRankCards{
    fn init_count(&mut self)->CountHighestRankCards{
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
}

pub fn get_bid(bid_payload:&InBid)->Bid{
    let InBid{cards,InBidState,..}=InBid;
    let mut count_suits=Trump::init_trump_count(&mut moduleinrust::Trump::default());
    count_suits.countsuits(&cards);
    let counted_suits:HashMap<String,u8>=count_suits.ret_counted_suits();//return counted suits from your cards in hashmap from... eg  "H":2, "S":1

    let mut count_rank_cards=CountHighestRankCards::init_count(&mut moduleinrust::CountHighestRankCards::default());
    let high_rank_cards:&HashMap<String,u8>=count_rank_cards.get_highet_rank_cards(&cards);
    

    //bidding decision starts here
    if bid_payload.bidHistory.len()==0 && can_get_min_bid(&counted_suits,&high_rank_cards){
        return Bid{bid:16,};//pass minimum bid
    }


    


    Bid{bid:0,}//pass bid
}


///bidding decisions logics
fn can_get_min_bid(suits:&)->bool{

}
fn can_challenge_the_defender_bid()->bool{

}

/*
Bid strategy

if 1st your turn,
    bid min 16 or pass
if defender bid 16:
    bid 17 or else pass

*/