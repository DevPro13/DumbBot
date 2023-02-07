use std::collections::HashMap;
//use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use rand::prelude::{thread_rng,Rng};//to generate ramdom result
use crate::play::GameDetails;
use crate::knowledge::{Knowledge,HandsInformation,MyCARDS};
#[derive(PartialEq,Debug,Clone)]
pub struct MCTSTwentyNineGameTreeNode {
    pub state:Option<Vec<String>>,//initial state
    pub children: Vec<Rc<RefCell<MCTSTwentyNineGameTreeNode>>>,//its child nodes
    pub parent:Option<Rc<RefCell<MCTSTwentyNineGameTreeNode>>>,//this node parent node
    pub plays:i32,//how many time this node played/visited
    pub wins:i32,//how many times it wins when it was played
  }
impl MCTSTwentyNineGameTreeNode{
    pub fn init()->MCTSTwentyNineGameTreeNode{
        MCTSTwentyNineGameTreeNode{
          state:None,
          children:vec![],
          parent:None,
          plays:0,
          wins:0,
        }
    }
    fn add_child_node(&mut self,new_node:Rc<RefCell<MCTSTwentyNineGameTreeNode>>){
      self.children.push(new_node);
  }
 
  pub fn select_node(&self)->Rc<RefCell<MCTSTwentyNineGameTreeNode>>{
        let mut selected_uct_val=std::f64::NEG_INFINITY;
        let mut selected_node=Rc::new(RefCell::new(MCTSTwentyNineGameTreeNode::init()));
        for child in self.children.iter(){
            //let mut child=child.borrow();
            //let child_uct_val=child.borrow().calc_uct_val();
            if child.as_ref().borrow().calc_uct_val()>selected_uct_val{
                selected_node=Rc::clone(&child);
                selected_uct_val=child.as_ref().borrow().calc_uct_val();
            }
        }
        //selected_node.as_ref().borrow().display();
        Rc::clone(&selected_node)
}
  pub fn expand_tree(&mut self,cards:&Vec<String>,parent_node:Rc<RefCell<MCTSTwentyNineGameTreeNode>>){
      for i in cards.iter(){
        let mut new_state:Vec<String>=self.state.as_ref().unwrap().to_owned();
        new_state.push(i.clone());
        let child=Rc::new(RefCell::new(MCTSTwentyNineGameTreeNode::init()));
        //let mut child_ref=child.borrow();
        child.borrow_mut().state=Some(new_state);
        child.borrow_mut().parent=Some(Rc::clone(&parent_node));
        self.add_child_node(Rc::clone(&child));
      }
  }
  fn calc_uct_val(&self)->f64{
    let c:f64=(2 as f64).sqrt();
    if self.plays==0{
        return std::f64::INFINITY;
    }
    let exploitation:f64=(self.wins/self.plays) as f64;
    let exploration:f64=c*((self.parent.as_ref().unwrap().as_ref().borrow().plays as f64).ln()/self.plays as f64).sqrt();
    exploitation+exploration
}
pub fn backpropagate(&mut self,winner_id:u8,myid:u8){
      self.plays+=1;
      self.parent.as_ref().unwrap().as_ref().borrow_mut().plays+=1;
      if winner_id==myid||(myid+2)%4==winner_id{
        self.wins+=1;//if we are winning
      }
      else{
        self.wins-=1;//if we are losing
      }
}
    pub fn rollout(&self,knowledge:&Knowledge,mycards:&MyCARDS,gamedetails:&mut GameDetails,handsinfo:&HandsInformation)->u8{
      let current_hand_suit:char=gamedetails.this_hand_suit;
      let mut current_state:Vec<String>=self.state.as_ref().unwrap().to_owned();
      //println!("Mah yeha chhu current state babu..{:?}",current_state);
      let mut this_hand_cards:Vec<String>=knowledge.get_opp_cards_of_this_suit(current_state[0].as_bytes()[1] as char,&mycards);
      let mut other_cards:Vec<String>=knowledge.get_opponent_cards_except_this_suit_cards(current_state[0].as_bytes()[1] as char,&mycards);
      while current_state.len()<4{
        if other_cards.len() as u8==0{
          break;
        }
        let mut rng = thread_rng();//for random number generation
         //...........................SECOND PLAY................
          //println!("Mah yeha chhu current state babu..{:?}\n",current_state);
           //println!("Mah yeha chhu.this hand cards.{:?}\n",this_hand_cards);
           //println!("Mah yeha chhu..other cards{:?}\n",other_cards);
        if current_state.len() as u8==1{//player second turn
                        gamedetails.this_hand_suit=current_state[0].as_bytes()[1] as char;
                        let mut ranout_suits=handsinfo.player_ran_out_of_these_suits((gamedetails.playerid+1)%4);
                        let leading_card=self.check_who_wins(&current_state, &gamedetails).1;
                        if handsinfo.probability_that_this_player_ran_out_of_this_suit((gamedetails.playerid+1)%4,gamedetails.this_hand_suit,gamedetails.playerid){
                          ranout_suits.push(gamedetails.this_hand_suit)
                        }
                        //println!("Mah yeha chhu.ran out suits of this plaeyr 1{:?}\n",ranout_suits);
                        let mut other_cards_for_this_player=knowledge.get_cards_except_these_suits(&other_cards, &ranout_suits);
                        //println!("Mah yeha chhu..other cards of this player {:?}\n",other_cards_for_this_player);
                        if other_cards_for_this_player.len() as u8==0{
                          other_cards_for_this_player=other_cards.to_owned();
                        }
                        if this_hand_cards.len() as u8!=0{
                          //if player has this hand card
                            //probably this player may run out of this hand suit cards
                            if handsinfo.probability_that_this_player_ran_out_of_this_suit((gamedetails.playerid+1)%4,gamedetails.this_hand_suit,gamedetails.playerid){
                                  let idx=rng.gen_range(0..other_cards_for_this_player.len() as u8);
                                  let indx=other_cards.iter().position(|r|r==&other_cards_for_this_player[idx as usize]).unwrap();
                                  current_state.push(other_cards_for_this_player.remove(idx as usize));
                                  other_cards.remove(indx);
                                  continue;
                            }
                            else{
                            //theres is this hand cards left
                            let idx=self.get_card(&leading_card,&this_hand_cards);
                            current_state.push(this_hand_cards.remove(idx as usize));
                            continue;
                            }
                        }
                        else{
                          //this player is ran out of this hand suit cards.. so lets throw some random cards..
                          let idx=rng.gen_range(0..other_cards_for_this_player.len() as u8);
                          let indx=other_cards.iter().position(|r|r==&other_cards_for_this_player[idx as usize]).unwrap();
                          current_state.push(other_cards_for_this_player.remove(idx as usize));
                          other_cards.remove(indx);
                          continue;
                          }
        }
        //...........................THIRD PLAY................
        let leading_card=self.check_who_wins(&current_state, &gamedetails).1;
        if current_state.len() as u8==2{//player third turn
                          //if player has this hand card
                          let mut ranout_suits=handsinfo.player_ran_out_of_these_suits((gamedetails.playerid+2)%4);
                          if handsinfo.probability_that_this_player_ran_out_of_this_suit((gamedetails.playerid+2)%4,gamedetails.this_hand_suit,gamedetails.playerid){
                            ranout_suits.push(gamedetails.this_hand_suit)
                          }
                          //println!("Mah yeha chhu.ran out suits of this plaeyr 2{:?}\n",ranout_suits);
                          let mut other_cards_for_this_player=knowledge.get_cards_except_these_suits(&other_cards, &ranout_suits);
                        //println!("Mah yeha chhu..other cards of this player{:?}\n",other_cards_for_this_player);
                        if other_cards_for_this_player.len() as u8==0{
                          other_cards_for_this_player=other_cards.to_owned();
                        }
                          if this_hand_cards.len() as u8!=0{
                            //probably this player may run out of this hand suit cards
                            if handsinfo.probability_that_this_player_ran_out_of_this_suit((gamedetails.playerid+1)%4,gamedetails.this_hand_suit,gamedetails.playerid){
                                  let idx=rng.gen_range(0..other_cards_for_this_player.len() as u8);
                                  let indx=other_cards.iter().position(|r|r==&other_cards_for_this_player[idx as usize]).unwrap();
                                  current_state.push(other_cards_for_this_player.remove(idx as usize));
                                  other_cards.remove(indx);
                                  continue;
                            }
                            else{
                            //theres is this hand cards left
                            let idx=self.get_card(&leading_card,&this_hand_cards);
                            current_state.push(this_hand_cards.remove(idx as usize));
                            continue;
                            }
                          }
                        else{
                          //this player is ran out of this hand suit cards.. so lets throw some random cards..
                          let idx=rng.gen_range(0..other_cards_for_this_player.len() as u8);
                          let indx=other_cards.iter().position(|r|r==&other_cards_for_this_player[idx as usize]).unwrap();
                          current_state.push(other_cards_for_this_player.remove(idx as usize));
                          other_cards.remove(indx);
                          continue;
                        }
        }
         //...........................4TH PLAY................
        if current_state.len() as u8==3{//player 4TH turn
                           //if player has this hand card
                           let mut ranout_suits=handsinfo.player_ran_out_of_these_suits((gamedetails.playerid+1)%4);
                           if handsinfo.probability_that_this_player_ran_out_of_this_suit((gamedetails.playerid+3)%4,gamedetails.this_hand_suit,gamedetails.playerid){
                             ranout_suits.push(gamedetails.this_hand_suit)
                           }
                           //println!("Mah yeha chhu.ran out suits of this plaeyr 3{:?}\n",ranout_suits);
                           let mut other_cards_for_this_player=knowledge.get_cards_except_these_suits(&other_cards, &ranout_suits);
                           //println!("Mah yeha chhu..other cards of this player{:?}\n",other_cards_for_this_player);
                           if other_cards_for_this_player.len() as u8==0{
                            other_cards_for_this_player=other_cards.to_owned();
                          }
                           if this_hand_cards.len() as u8!=0{
                            //probably this player may run out of this hand suit cards
                            if handsinfo.probability_that_this_player_ran_out_of_this_suit((gamedetails.playerid+3)%4,gamedetails.this_hand_suit,gamedetails.playerid){
                                  let idx=rng.gen_range(0..other_cards_for_this_player.len() as u8);
                                  let indx=other_cards.iter().position(|r|r==&other_cards_for_this_player[idx as usize]).unwrap();
                                current_state.push(other_cards_for_this_player.remove(idx as usize));
                                other_cards.remove(indx);
                                continue;
                            }
                            else{
                            //theres is this hand cards left
                            let idx=self.get_card(&leading_card,&this_hand_cards);
                            current_state.push(this_hand_cards.remove(idx as usize));
                            continue;
                            }
                          }
                        else{
                          //this player is ran out of this hand suit cards.. so lets throw some random cards..
                          let idx=rng.gen_range(0..other_cards_for_this_player.len() as u8);
                          let indx=other_cards.iter().position(|r|r==&other_cards_for_this_player[idx as usize]).unwrap();
                          current_state.push(other_cards_for_this_player.remove(idx as usize));
                          other_cards.remove(indx);
                          continue;
                        }

        }
      }
      println!("{:?}",current_state);
        let winner:(u8,String)=self.check_who_wins(&current_state,&gamedetails);//returns the winner
        println!("{:?}",winner);
        gamedetails.this_hand_suit=current_hand_suit;//update previous this hand suit
        winner.0
    }
    fn display(&self){
      println!("self.state={:?}\n self.plays={}\n self.wins={}\n",self.state.as_ref().unwrap(),self.plays,self.wins);
    }
    fn get_card(&self,leading_card:&String,cards:&Vec<String>)->usize{
      let mut rng = thread_rng();//for random number generation
        let card_map_to_rank_point:HashMap<char,(u8,u8)>=HashMap::from([
          //each suit cards ranks and points
          ('J',(1,3)),
          ('9',(2,2)),
          ('1',(3,1)),
          ('T',(4,1)),
          ('K',(5,0)),
          ('Q',(6,0)),
          ('8',(7,0)),
          ('7',(8,0))
]);
          let mut card_to_throw:String=leading_card.to_string();
          for card in cards.iter(){
            if card_map_to_rank_point[&(card_to_throw.as_bytes()[0] as char)]>card_map_to_rank_point[&(card.as_bytes()[0] as char)]{
                card_to_throw=card.to_string();
                break;
            }
          }
          if card_to_throw==*leading_card{
            return rng.gen_range(0..cards.len() as u8) as usize;
          }
          cards.iter().position(|r|r==&card_to_throw).unwrap() as usize

    }
    pub fn best_score_node(&self)->Rc<RefCell<MCTSTwentyNineGameTreeNode>>{
      let mut temp_node=Rc::clone(&self.children[0]);
      if temp_node.as_ref().borrow().wins==0{
        temp_node.as_ref().borrow_mut().wins=1;
      }
      for i in self.children.iter(){
        if i.as_ref().borrow().wins==0{
          i.as_ref().borrow_mut().wins=-1;
        }
        i.as_ref().borrow().display();
        if (i.as_ref().borrow().plays/i.as_ref().borrow().wins)>(temp_node.as_ref().borrow().plays/temp_node.as_ref().borrow().wins){
          temp_node=Rc::clone(i);
        }
      }
      Rc::clone(&temp_node)
    }
    pub fn worst_score_node(&self)->Rc<RefCell<MCTSTwentyNineGameTreeNode>>{
      let mut temp_node=Rc::clone(&self.children[0]);
      if temp_node.as_ref().borrow().wins==0{
        temp_node.as_ref().borrow_mut().wins=1;
      }
      for i in self.children.iter(){
        i.as_ref().borrow().display();
        if i.as_ref().borrow().wins==0{
          i.as_ref().borrow_mut().wins=-1;
        }
        i.as_ref().borrow().display();
        if (i.as_ref().borrow().plays/i.as_ref().borrow().wins)<(temp_node.as_ref().borrow().plays/temp_node.as_ref().borrow().wins){
          temp_node=Rc::clone(i);
        }
      }
      Rc::clone(&temp_node)
    }
    fn check_who_wins(&self,played:&Vec<String>,gamedetails:&GameDetails)->(u8,String){
      //if winner team is yours.. maximize the point. else minimize
      //return possible winner and thrown card
          //println!("gamedetails {:?}",gamedetails);
          let mut possible_winner:u8=gamedetails.last_hand_winner;
          let mut thrown_by:u8=gamedetails.last_hand_winner;
          let mut winner_rank_point:(u8,u8)=gamedetails.card_map_to_rank_point[&(played[0].as_bytes()[0] as char)];
          let mut winning_suit:char=gamedetails.this_hand_suit;//if trump card. change it to trump_suit
          let mut thrown_card:&String=&played[0];
          for i in played[1..played.len()].iter(){
              thrown_by= (thrown_by+1)%4;
              let played_suit=i.as_bytes()[1] as char;
              let rank_point:(u8,u8)=gamedetails.card_map_to_rank_point[&(i.as_bytes()[0] as char)];
              if winning_suit==played_suit && rank_point.0<winner_rank_point.0{
                  winner_rank_point=rank_point;
                  possible_winner=thrown_by;
                  thrown_card=i;

              }
              else if played_suit==gamedetails.trump_suit{
                  if winning_suit!=gamedetails.trump_suit{
                      winning_suit=gamedetails.trump_suit;
                      winner_rank_point=rank_point;
                      possible_winner=thrown_by; 
                      thrown_card=i;

                  }
                  else if rank_point.0<winner_rank_point.0 {
                      winner_rank_point=rank_point;
                      possible_winner=thrown_by;
                      thrown_card=i;

                  } 
              }   
          }
          (possible_winner,thrown_card.to_string())
  }
  pub fn get_best_score_card(&self,idx:u8)->String{
        self.state.as_ref().unwrap()[idx as usize].clone()
  }
}