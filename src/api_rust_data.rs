use serde::{Serialize,Deserialize};
// /hi responce
#[derive(Serialize,Debug)]
pub struct Hello{
    value:String,
}
// /bid payload
#[derive(Derialize,Debug)]
pub struct InBidState{
    defenderId:String,
    challengerId:String,
    defenderBid:u8,
    challengerBid:u8,
}
#[derive(Desrialize,Debug)]
pub struct InBid{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:i32,
    bidHistory:Vec<(String,u8)>,
    bidState:InBidState,
}
// /bid responce
#[derive(Serialize,Debug)]
pub struct Bid{
    bid:u8,
}
// /chooseTrump payload
#[derive(Deserialize,Debug)]
pub struct ChooseTrumpSuit{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:i32,
    cards:Vec<String>,
    bidHistory:Vec<(String,u8)>,
}
//choosetrump response
#[derive(Serialize,Debug)]
pub struct TrumpSuit{
    suit:String,
}
// plAY PAYLOAD
#[derive(Debug,Deserialize)]
#[serde(untagged)]
pub enum TrumpSuitEnum {
    Suit(String),
    SuitShown(bool),
}
#[derive(Deserialize,Debug)]
pub struct Team{
    players:Vec<String>,
    bid:u8,
    won:u8,
}
#[derive(Deserialize,Debug)]
pub struct TrumpRevealed{
    hand:u8,
    playerId:String,
}
#[derive(Debug,Deserialize)]
#[serde(untagged)]
pub enum TrumpRevealEnum {
    RevealedBy(TrumpRevealed),
    trumpRevealed(bool),
}
#[derive(Deserialize,Debug)]
pub struct Play{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:i32,
    teams:Vec<Team>,
    cards:Vec<String>,
    bidHistory:Vec<(String,u8)>,
    played:Vec<String>,
    handsHistory: Vec<(String,Vec<String>,String)>,
    trumpSuit:TrumpSuitEnum,
    trumpRevealed:TrumpRevealEnum,
}
// play responce
#[derive(Serialize,Debug)]
pub struct ThrowCard{
    card:String,
}
// if you have the request trumo reveal
#[derive(Serialize,Debug)]
pub struct RevealTrump{
    revealTrump:bool,
}
// reveal trump and throw card at once
#[derive(Serialize,Debug)]
pub struct RevealTrumpAndThrowCard{
    revealTrump:bool,
    card:String,
}