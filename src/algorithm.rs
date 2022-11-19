use std::collections::HashMap;
use std::collections::HashSet;

mod algorithm{
const cards_point=HashMap::from([
    ("JS",3),("JD",3),("JH",3),("JC",3),
    ("9S",2),("9D",2),("9H",2),("9C",2),
    ("1S",1),("1D",1),("1H",1),("1C",1),
    ("TS",1),("TD",1),("TH",1),("TC",1),
    ("KS",0),("KD",0),("KH",0),("KC",0),
    ("QS",0),("QD",0),("QH",0),("QC",0),
    ("8S",0),("8D",0),("8H",0),("8C",0),
    ("7S",0),("7D",0),("7H",0),("7C",0),
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