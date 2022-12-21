use serde::{Serialize,Deserialize};
// /bid payload
#[derive(Deserialize,Debug)]
pub struct InBidState{
    pub defenderId:String,
    pub challengerId:String,
    pub defenderBid:u8,
    pub challengerBid:u8,
}
#[derive(Deserialize,Debug)]
pub struct InBid{
    pub playerId:String,
    pub playerIds:Vec<String>,
    pub timeRemaining:i32,
    pub cards:Vec<String>,
    pub bidHistory:Vec<(String,u8)>,
    pub bidState:InBidState,
}
// /bid responce
#[derive(Serialize,Debug)]
pub struct Bid{
    pub bid:u8,
}
// /chooseTrump payload
#[derive(Deserialize,Debug)]
pub struct ChooseTrumpSuit{
    pub playerId:String,
    pub playerIds:Vec<String>,
    pub timeRemaining:i32,
    pub cards:Vec<String>,
    pub bidHistory:Vec<(String,u8)>,
}
//choosetrump response
#[derive(Serialize,Debug)]
pub struct TrumpSuit{
    pub suit:String,
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
    pub players:Vec<String>,
    pub bid:u8,
    pub won:u8,
}
#[derive(Deserialize,Debug)]
pub struct TrumpRevealed{
    pub hand:u8,
    pub playerId:String,
}
#[derive(Debug,Deserialize)]
#[serde(untagged)]
pub enum TrumpRevealEnum {
    RevealedBy(TrumpRevealed),
    trumpRevealed(bool),
}
#[derive(Deserialize,Debug)]
pub struct Play{
    pub playerId:String,
    pub playerIds:Vec<String>,
    pub timeRemaining:i32,
    pub teams:Vec<Team>,
    pub cards:Vec<String>,
    pub bidHistory:Vec<(String,u8)>,
    pub played:Vec<String>,
    pub handsHistory: Vec<(String,Vec<String>,String)>,
    pub trumpSuit:TrumpSuitEnum,
    pub trumpRevealed:TrumpRevealEnum,
}
// play responce
#[derive(Serialize,Debug)]
pub struct ThrowCard{
    pub card:String,
}
// if you have the request trumo reveal
#[derive(Serialize,Debug)]
pub struct RevealTrump{
    pub revealTrump:bool,
}
// reveal trump and throw card at once
#[derive(Serialize,Debug)]
pub struct RevealTrumpAndThrowCard{
    pub revealTrump:bool,
    pub card:String,
}