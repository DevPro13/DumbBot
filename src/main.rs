mod choosetrump;
mod api_rust_data;
mod bid;
use self::bid::get_bid;
use self::choosetrump::get_trump_suit;
mod hi;
use self::hi::Hello;
//use std::time::Instant;
use actix_cors::Cors;
use actix_web::{get, 
                post,
                web::{self, Payload}, 
                App, 
                HttpResponse, 
                HttpServer, 
                Responder, 
                Result,
                http::header::ContentType,
            };
use self::api_rust_data::{
                            InBid as BidPayload,//bid payload
                            Bid,//bid responce
                            ChooseTrumpSuit as ChooseTrumpPayload,
                            TrumpSuit,//choose trump responce
                            Play,//play payload
                            //play responses
                            RevealTrump,
                            ThrowCard,
};
#[get("/hi")]
async fn hi_req() ->impl Responder {
    let hello:Hello =Hello::responce_hi(&mut Hello::default());
     // Serialize it to a JSON string.
    let body = serde_json::to_string(&hello).unwrap();
    // Create response and set content type
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[post("/bid")]
async fn bid_req(payload: web::Json<BidPayload>) -> Result<String> {
    //println!("{:?}",payload);
    let web::Json(BidPayload)=payload;
    //println!("{:?}",bid_payload);
    let obtained_bid:Bid=get_bid(&BidPayload);//get object of Bidresponce
     // Serialize it to a JSON string.
     let body = serde_json::to_string(&obtained_bid).unwrap();
     Ok(format!("{}",body))
 }

#[post("/chooseTrump")]
async fn trump_req(payload: web::Json<ChooseTrumpPayload>) -> Result<String> {
    println!("{:?}",payload);
    let web::Json(ChooseTrumpPayload)=payload;
    let trump_suiit =get_trump_suit(&ChooseTrumpPayload.cards);//get object of trumpResponce
    // Serialize it to a JSON string.
    let body = serde_json::to_string(&trump_suiit).unwrap();
    //let body=r#"{"suit"}:"H""#;
    Ok(format!("{}",body))
}
#[post("/play")]
async fn play_card(payload: web::Json<Play>) -> Result<String> {
    println!("{:?}",payload);
    //let web::Json(Play:play_payload)=payload;
    // let start = Instant::now();
    //et player_move = play::get_move(&payload);
    // let duration = start.elapsed();
    // println!("Get move took {:?}", duration);
    //let play_card=play::play_game(&payload.Play);//get object of playresponce
    // Serialize it to a JSON string.
    //let body = serde_json::to_string(&play_card).unwrap();
    let body=r#"{"suit"}:"JH""#;
    Ok(format!("{}",body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hi_req)
            .service(bid_req)
            .service(trump_req)
            .service(play_card)
    })
    .bind(("127.0.0.1",7878))?
    .run()
    .await
}

/*
Json(InBid { playerId: "A2", playerIds: ["A1", "B1", "A2", "B2"], timeRemaining: 1000, cards: ["JS", "TS", "KH", "9C"], bidHistory: [("A1", 16), ("B1", 0)], bidState: InBidState { defenderId: "A1", challengerId: "B1", defenderBid: 16, challengerBid: 17 } })
"{\"bid\":18}"
Json(InBid { playerId: "A2", playerIds: ["A1", "B1", "A2", "B2"], timeRemaining: 1000, cards: ["JS", "TS", "KH", "9C"], bidHistory: [("A1", 16), ("B1", 0)], bidState: InBidState { defenderId: "A1", challengerId: "B1", defenderBid: 16, challengerBid: 17 } })
"{\"bid\":18}"



Json(InBid { playerId: "A2", playerIds: ["A1", "B1", "A2", "B2"], timeRemaining: 1000, cards: ["JS", "TS", "KH", "9C"], bidHistory: [("A1", 16), ("B1", 0)], bidState: InBidState { defenderId: "A1", challengerId: "B1", defenderBid: 16, challengerBid: 17 } })
Json(ChooseTrumpSuit { playerId: "A2", playerIds: ["A1", "B1", "A2", "B2"], timeRemaining: 1000, cards: ["JS", "TS", "KH", "9C"], bidHistory: [("A1", 16), ("B1", 0)] })
Json(Play { playerId: "A2", playerIds: ["A1", "B1", "A2", "B2"], timeRemaining: 1500, teams: [Team { players: ["A1", "A2"], bid: 17, won: 0 }, Team { players: ["B1", "B2"], bid: 17, won: 4 }], cards: ["JS", "TS", "KH", "9C", "JD", "7D", "8D"], bidHistory: [("A1", 16), ("B1", 17), ("A1", 17), ("B1", 0), ("A2", 0), ("B2", 0)], played: ["9S", "1S", "8S"], handsHistory: [("A1", ["7H", "1H", "8H", "JH"], "B2")], trumpSuit: SuitShown(false), trumpRevealed: trumpRevealed(false) })


*/