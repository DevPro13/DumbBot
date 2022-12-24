use std::collections::HashMap;
use super::choosetrump::Trump;
#[derive(Clone,Default)]
pub struct CountHighestRankCards{
    cards:HashMap<String,u8>,
}


use super::api_rust_data::{InBid,Bid,InBidState};
impl CountHighestRankCards{
    pub fn init_count(&mut self)->CountHighestRankCards{
        CountHighestRankCards{
            cards:HashMap::from([
                ("J".to_string(),0),
                ("9".to_string(),0),
                ("T".to_string(),0),
                ("1".to_string(),0),
            ]),
        }
    }
    fn count_highest_rank_cards(&mut self,cards:&Vec<String>){
        for  i in cards{
            let k=i.as_bytes()[0] as char;
            if k=='J' || k=='9'||k=='T'||k=='1'{
                self.cards.insert(k.to_string(),self.cards[&k.to_string()]+1);
            }
        }
    }
    fn check_atleast_one_present(&self,card:String)->bool{
        if self.cards[&card]>0{
            return true;
        }
        false
    }
    fn return_total_cards_of_given_rank(&self,card:String)->u8{
        self.cards[&card]
    }
}

pub fn get_bid(bid_payload:&InBid)->Bid{
    let InBid{cards,bidState:in_bid_state,..}=bid_payload;
    let mut suits=Trump::init_trump_count(&mut Trump::default());
    suits.countsuits(&cards);
    let mut my_high_rank_cards=CountHighestRankCards::init_count(&mut CountHighestRankCards::default());
    my_high_rank_cards.count_highest_rank_cards(&cards);
    //bidding decision starts here
    if bid_payload.bidHistory.len()==0 && can_get_max_bid(&my_high_rank_cards,&suits){
        return Bid{bid:16,};//pass minimum bid
    }

    //yedi def and chalenger is betn team.. one of them must pass to minimize max bid.
    let index = bid_payload.playerIds.iter().position(|playerid| *playerid == in_bid_state.defenderId).unwrap();
    if bid_payload.playerIds[(index+2)%4]==in_bid_state.challengerId{
        return Bid{bid:0,};//return because i don't want to increase the bid
    }

    if can_get_max_bid(&my_high_rank_cards,&suits){
    if bid_payload.playerId==in_bid_state.defenderId{
        if in_bid_state.challengerBid>=18{// i don't wanna go any further
            return Bid{bid:0,};
        }
       //if this true bid more than challenger bid
        if (suits.check_if_cards_has_three_same_suits()||suits.check_if_cards_has_two_same_suits()) && my_high_rank_cards.check_atleast_one_present("J".to_string())&& my_high_rank_cards.check_atleast_one_present("9".to_string()){
            return Bid{
                bid:in_bid_state.challengerBid+1,
            };
        }
        //else bid equal
        if suits.check_if_cards_has_two_same_suits() && my_high_rank_cards.check_atleast_one_present("J".to_string()){
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
        if in_bid_state.defenderBid>=18{// i don't wanna go any further
        return Bid{bid:0,};
    }
   //if this true bid more than challenger bid
    if (suits.check_if_cards_has_three_same_suits()||suits.check_if_cards_has_two_same_suits()) && my_high_rank_cards.check_atleast_one_present("J".to_string())&& my_high_rank_cards.check_atleast_one_present("9".to_string()){
        return Bid{
            bid:in_bid_state.challengerBid+1,
        };
    }
}
}
    Bid{bid:0,}//pass bid
}


///bidding decisions logics
fn can_get_max_bid(my_high_rank_cards:&CountHighestRankCards,suits:&Trump)->bool{
    //if i have atleast 1 1 J and 9 cards.. I should bet minimum
    if my_high_rank_cards.check_atleast_one_present("J".to_string())&& my_high_rank_cards.check_atleast_one_present("9".to_string()){
        return true;
    }
    //if more than 2 9s, bet min
    if my_high_rank_cards.return_total_cards_of_given_rank("J".to_string())>1 || my_high_rank_cards.return_total_cards_of_given_rank("9".to_string())>1{
        return true;
    }
//if cards have 3 same suit cards
    if suits.check_if_cards_has_three_same_suits(){
        return true;
    }
    //if cards has 2 same suit and atleast one 9 and
    if suits.check_if_cards_has_two_same_suits() && my_high_rank_cards.check_atleast_one_present("J".to_string())&& my_high_rank_cards.check_atleast_one_present("9".to_string()){
        return true;
    }
    false

}


/*Json(InBid { playerId: "You-0", playerIds: ["You-0", "Opponent-0", "You-1", "Opponent-1"], timeRemaining: 1500, cards: ["7S", "1C", "JD", "8C"], bidHistory: [], bidState: InBidState { defenderId: "You-0", challengerId: "Opponent-0", defenderBid: 0, challengerBid: 0 } })
Json(InBid { playerId: "Opponent-0", playerIds: ["You-0", "Opponent-0", "You-1", "Opponent-1"], timeRemaining: 1500, cards: ["1S", "1H", "KD", "KS"], bidHistory: [("You-0", 16)], bidState: InBidState { defenderId: "You-0", challengerId: "Opponent-0", defenderBid: 16, challengerBid: 0 } })
Json(InBid { playerId: "You-1", playerIds: ["You-0", "Opponent-0", "You-1", "Opponent-1"], timeRemaining: 1500, cards: ["QH", "7H", "QD", "9C"], bidHistory: [("You-0", 16), ("Opponent-0", 0)], bidState: InBidState { defenderId: "You-0", challengerId: "You-1", defenderBid: 16, challengerBid: 0 } })
Json(InBid { playerId: "Opponent-1", playerIds: ["You-0", "Opponent-0", "You-1", "Opponent-1"], timeRemaining: 1500, cards: ["JS", "TD", "JH", "KH"], bidHistory: [("You-0", 16), ("Opponent-0", 0), ("You-1", 0)], bidState: InBidState { defenderId: "You-0", challengerId: "Opponent-1", defenderBid: 16, challengerBid: 0 } })
 */