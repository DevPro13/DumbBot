use serde::{Serialize,Deserialize};

// /hi responce
#[derive(Serialize,Debug)]
pub struct Hello{
    pub value:String,
}
/*..................................................................................................... */
// /bid payload
#[derive(Deserialize,Debug)]
pub struct InBidState{
    defenderId:String,
    challengerId:String,
    defenderBid:u8,
    challengerBid:u8,
}
#[derive(Deserialize,Debug)]
pub struct InBid{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:u16,
    cards:Vec<String>,
    bidHistory:Vec<(String,u8)>,
    bidState:InBidState,
}
/*..................................................................................................... */
// /bid responce
#[derive(Serialize,Debug)]
pub struct Bid{
    bid:u8,
}
/*..................................................................................................... */
// /chooseTrump payload
#[derive(Deserialize,Debug)]
pub struct ChooseTrumpSuit{
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:u16,
    cards:Vec<String>,
    bidHistory:Vec<(String,u8)>,
}
/*..................................................................................................... */
//choosetrump response
#[derive(Serialize,Debug)]
pub struct TrumpSuit{
    suit:String,
}
/*..................................................................................................... */
// plAY PAYLOAD
#[derive(Debug,Deserialize)]
#[serde(untagged)]
pub enum TrumpSuitEnum {
 #[allow(non_camel_case_types)]
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
 #[allow(non_camel_case_types)]
    hand:u8,
    playerId:String,
}
#[derive(Debug,Deserialize)]
#[serde(untagged)]
pub enum TrumpRevealEnum {
 #[allow(non_camel_case_types)]
    RevealedBy(TrumpRevealed),
    trumpRevealed(bool),
}
 #[allow(non_camel_case_types)]
#[derive(Deserialize,Debug)]
pub struct Play{//T bool or string, U--> bool or Object
 //#[allow(non_camel_case_types)]
    playerId:String,
    playerIds:Vec<String>,
    timeRemaining:u16,
    teams:Vec<Team>,
    cards:Vec<String>,
    bidHistory:Vec<(String,u8)>,
    played:Vec<String>,
    handsHistory: Vec<(String,Vec<String>,String)>,
    trumpSuit:TrumpSuitEnum,
    trumpRevealed:TrumpRevealEnum,
}
/*..................................................................................................... */
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