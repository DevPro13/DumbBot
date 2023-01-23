mod choosetrump;
mod api_rust_data;
mod bid;
use self::bid::get_bid;
use self::choosetrump::get_trump_suit;
mod hi;
use self::hi::Hello;
mod play;
mod knowledge;
mod mcts_algorithm;
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
                            Play,//for play payload
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
    println!("{:?}",payload);
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
    let trump_suit =get_trump_suit(&ChooseTrumpPayload.cards);//get object of trumpResponce
    // Serialize it to a JSON string.
    let body = serde_json::to_string(&trump_suit).unwrap();
    Ok(format!("{}",body))
}
#[post("/play")]
async fn play_card(payload: web::Json<Play>) -> Result<String> {
    println!("{:?}\n\n",payload);
    let web::Json(Play)=payload;
    let play_card_body=play::play_game::play_card(&Play);
    println!("thrown responce = {}",play_card_body);
    Ok(play_card_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello World! I am running!!!!......");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                .allow_any_origin() //      to allow input from any origin i.e. sandbox to pc, or server to docker the instance
                .allowed_methods(vec!["GET", "POST"]) //      to allow only two method used, "get" and "post" method of request
                .allow_any_header() //      to allow any header information sent with the method, it doesn't matter
                .max_age(300), //           cache time set 5 minutes for frequent update
        )
            .service(hi_req)
            .service(bid_req)
            .service(trump_req)
            .service(play_card)
    })
    .bind(("0.0.0.0",8001))?
    .run()
    .await
}
