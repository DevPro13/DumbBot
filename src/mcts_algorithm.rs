use std::cell::RefCell;
use std::rc::Rc;
use crate::play::GameDetails;
const c:f64=(2 as f64).sqrt();
#[derive(PartialEq,Debug)]
struct MCTSMCTSTwentyNineGameTreeNode {
    pub state:Vec<String>,//initial state
    pub children: Vec<Rc<RefCell<MCTSTwentyNineGameTreeNode>>>,//its child nodes
    pub parent: Option<Rc<RefCell<MCTSTwentyNineGameTreeNode>>>,//this node parent node
    pub position:isize,//current position to throw the card
    pub nxt_move:usize,//
    pub plays:u32,//how many time this node played/visited
    pub wins:u32,//how many times it wins when it was played
  }
  impl MCTSTwentyNineGameTreeNode{
    fn get_positions(&self,played:&Vec<String>)->Vec<usize>{
      let mut pos:Vec<usize>=Vec::new();
      for i in played.len()..4{
        pos.push(i)
      }
      pos
    }
    pub fn init(&mut self,state:&Vec<String>,pos:isize)->Rc<RefCell<MCTSMCTSTwentyNineGameTreeNode>>{
        MCTSMCTSTwentyNineGameTreeNode{
          state:state.to_vec(),
          children:vec![],
          parent:None,
          position:pos,
          nxt_move:pos+1,
          plays:0,
          wins:0,
        }
    }
    pub fn add_child_node(&mut self,new_node:Rc<RefCell<MCTSMCTSTwentyNineGameTreeNode>>){
      self.children.borrow_mut().push(new_node);
  }
 
     fn select_node(&self)->Rc<RefCell<MCTSMCTSTwentyNineGameTreeNode>>{
            let mut select_node=Rc::clone(&self);
            let selected_uct_val=std::f64::INFINITY;
            for child in self.children.iter(){
                let mut child_uct_val:f64=child.calc_uct_val();
                if child_uct_val>selected_uct_val{
                    select_node=Rc::clone(&child);
                    child_uct_val=selected_uct_val
                }
            }
            Rc::clone(&select_node)
    }
    fn expand_tree(&mut self,cards:&Vec<String>){
        for i in cards.iter(){
          let mut new_state:Vec<String>=self.state;
          new_state.push(i);
          let child=Rc::new(RefCell::new(MCTSMCTSTwentyNineGameTreeNode::init(new_state,self.position+1)));
          child.borrow_mut().parent=Rc::clone(&self);
          self.add_child_node(Rc::clone(&child));
        }
    }
    fn calc_uct_val(&self)->f64{
        if self.plays==0{
            return std::f64::INFINITY;
        }
        let exploitation:f64=self.wins as f64/self.plays as f64;
        let exploration:f64=c*((self.parent.plays as f64).ln()/self.plays as f64).sqrt();
        exploitation+exploration
    }
    fn simulate_node(&self){

    }
  }