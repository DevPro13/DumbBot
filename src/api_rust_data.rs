use serde::{Serialize,Deserialize};
//#[serde(rename_all = "camelCase")]

// /hi responce
#derive[(Serialize,Debug)]
struct Hello{
    value:String,
}
// /bid payload
#derive[(Derialize,Debug)]
struct InBidState{
    defenderId:String,
    challengerId:String,
    defenderBid:u8,
    challengerBid:u8,
}
#derive[(Desrialize,Debug)]
struct InBid{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:u16,
    bidHistory:Vec<(String,u8)>,
    bidState:InBidState,
}
// /bid responce
#derive[(Serialize,Debug)]
struct Bid{
    bid:u8,
}
// /chooseTrump payload
#derive[(Deserialize,Debug)]
struct ChooseTrumpSuit{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:u16,
    cards:Vec<String>,
    bidHistory:Vec<(String,u8)>,
}
//choosetrump response
#derive[(Serialize,Debug)]
struct TrumpSuit{
    suit:String,
}
// plAY PAYLOAD
#derive[(Deserialize,Debug)]
struct Team{
    players:Vec<String>,
    bid:u8,
    won:u8,
}
#derive[(Desrialize,Debug)]
struct TrumpRevealed{
    hand:u8,
    playerId:String,
}
#derive[(Deserialize,Debug)]
struct Play<T,U>{//T bool or string, U--> bool or Object
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
struct ThrowCard{
    card:String,
}
// if you have the request trumo reveal
#derive[(Serialize,Debug)]
struct RevealTrump{
    revealTrump:bool,
}
// reveal trump and throw card at once
#derive[(Serialize,Debug)]
struct RevealTrumpAndThrowCard{
    revealTrump:bool,
    card:String,
}