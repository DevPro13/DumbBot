use serde_json::Value;
use crate::get_bid;
use crate::BidPayload;
use crate::Bid;
#[cfg(test)]
mod unit_test{
    #[test]
    pub fn test_bid(bid_payload:&BidPayload,expected_bid:u8){
        let obtained_bid:Bid=get_bid(&bid_payload);
        println!("{}",format!(r#"{{"obtained_bid":{}, "expected":{expected_bid}}}"#,obtained_bid.bid));
        assert_eq!(obtained_bid.bid,expected_bid);   
    }
}
use unit_test::test_bid;
pub fn perform_bid_test(){
    let test1=r#"{
        "playerId": "A2",
        "playerIds": [
          "A1",
          "B1",
          "A2",
          "B2"
        ],
        "cards": [
          "JS",
          "TS",
          "KH",
          "9C"
        ],
        "timeRemaining": 1500,
        "bidHistory": [
          [
            "A1",
            16
          ],
          [
            "B1",
            0
          ]
        ],
        "bidState": {
          "defenderId": "A1",
          "challengerId": "B1",
          "defenderBid": 16,
          "challengerBid": 17
        }
      }"#;
    let test2=r#""#;
    let test3=r#""#;
    let test4=r#""#;
    let test5=r#""#;
    let test6=r#""#;
    let test7=r#""#;
    let test8=r#""#;
    let test9=r#""#;
    let test10=r#""#;

}