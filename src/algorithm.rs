use std::collections::{
    HashMap,
    HashSet
};
mod algorithm{
const points=HashMap::from([
        "J":3,
        "9":2,
        "T":1,
        "1":1,
        "K":0,
        "Q":0,
        "8":0,
        "7":0,
    ]);
//SET OF ALL CARDS
const cards=HashSet::from([
    "JS","JD","JH","JC",
    "9S","9D","9H","9C",
    "1S","1D","1H","1C",
    "TS","TD","TH","TC",
    "KS","KD","KH","KC",
    "QS","QD","QH","QC",
    "8S","8D","8H","8C",
    "7S","7D","7H","7C",
]);
const rank::HashMap::from([
    "J":8,
    "9":7,
    "T":6,
    "1":5,
    "K":4,
    "Q":3,
    "8":2,
    "7":1,
]);


fn give_sum_of_points(board)->u8{

}
fn minimax(is_max_team:bool,board){//is_,max represent miximizing player or not..and board is playboard.. 
    if board.len()==4{
        return give_sum_of_points(board);
    }
    if is_max_team{
        

    }
    else{
        //min team
    }

}
    
}
mod bid{

}
mod get_trump{
    
}