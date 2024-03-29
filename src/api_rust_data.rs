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
    pub timeRemaining:f64,
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
    pub timeRemaining:f64,
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
    Suit(char),
    SuitShown(bool),
}
#[derive(Deserialize,Debug)]
pub struct Team{
    pub players:Vec<String>,
    pub bid:u8,
    pub won:u8,
}
#[derive(Deserialize,Debug,Default,Clone)]
pub struct TrumpRevealedBy{
    pub hand:u8,
    pub playerId:String,
}
#[derive(Debug,Deserialize)]
#[serde(untagged)]
pub enum TrumpRevealEnum {
    RevealedBy(TrumpRevealedBy),
    trumpRevealed(bool),
}
#[derive(Deserialize,Debug)]
pub struct Play{
    pub playerId:String,
    pub playerIds:Vec<String>,
    pub timeRemaining:f64,
    pub teams:Vec<Team>,
    pub cards:Vec<String>,
    pub bidHistory:Vec<(String,u8)>,
    pub played:Vec<String>,
    pub handsHistory: Vec<(String,Vec<String>,String)>,
    pub trumpSuit:TrumpSuitEnum,
    pub trumpRevealed:TrumpRevealEnum,
}