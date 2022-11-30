use serde::{Serialize,Deserialize};
//#[serde(rename_all = "camelCase")]

// /hi responce
#derive[(Serialize,Debug)]
pub struct Hello{
    value:String,
}
// /bid payload
#derive[(Derialize,Debug)]
pub struct InBidState{
    defenderId:String,
    challengerId:String,
    defenderBid:u8,
    challengerBid:u8,
}
#derive[(Desrialize,Debug)]
pub struct InBid{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:u16,
    bidHistory:Vec<(String,u8)>,
    bidState:InBidState,
}
// /bid responce
#derive[(Serialize,Debug)]
pub struct Bid{
    bid:u8,
}
// /chooseTrump payload
#derive[(Deserialize,Debug)]
pub struct ChooseTrumpSuit{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:u16,
    cards:Vec<String>,
    bidHistory:Vec<(String,u8)>,
}
//choosetrump response
#derive[(Serialize,Debug)]
pub struct TrumpSuit{
    suit:String,
}
// plAY PAYLOAD
#derive[(Deserialize,Debug)]
pub struct Team{
    players:Vec<String>,
    bid:u8,
    won:u8,
}
#derive[(Desrialize,Debug)]
pub struct TrumpRevealed{
    hand:u8,
    playerId:String,
}
#derive[(Deserialize,Debug)]
pub struct Play<T,U>{//T bool or string, U--> bool or Object
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:u16,
    teams:Vec<Team>,
    cards:Vec<String>,
    bidHistory:Vec<(String,u8)>,
    played:Vec<String>,
    handsHistory: Vec<(String,Vec<String>,String)>,
    trumpSuit:T,
    trumpRevealed:U,
}
// play responce
#derive[(Serialize,Debug)]
pub struct ThrowCard{
    card:String,
}
// if you have the request trumo reveal
#derive[(Serialize,Debug)]
pub struct RevealTrump{
    revealTrump:bool,
}
// reveal trump and throw card at once
#derive[(Serialize,Debug)]
pub struct RevealTrumpAndThrowCard{
    revealTrump:bool,
    card:String,
}