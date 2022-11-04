use std::{
    borrow::BorrowMut,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    Helmet(bool),              //is damaged?
    LeftThrusters(bool, i32),  //is damaged? How much power left?
    RightThrusters(bool, i32), //is damaged? How much power left?
    LeftRepulsor(bool, i32),   //is damaged? How much power left?
    RightRepulsor(bool, i32),  //is damaged? How much power left?
    ChestPiece(bool, i32),     //is damaged? How much power left?
    Missiles(i32),             //how many missiles left?
    ArcReactor(i32),           // How much power left?
    Wifi(bool),                // connected to wifi?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Armor {
    pub component: Component,
    pub version: i32,
}

// Part 2

// Students should fill in the Link type themselves. The Node and List types are given as is.
type Link = Option<Arc<RwLock<Node>>>;

struct Node {
    data: Armor,
    rest: Link,
}

#[derive(Clone)]
pub struct List {
    head_link: Link,
    size: usize,
}

impl List {
    pub fn new() -> Self {
     return List{head_link: None, size: 0}; 
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn peek(&self) -> Option<Armor> {
        match &self.head_link {
            None => None,
            Some(x) => {
                let locked_node = x.clone();
	        let item = locked_node.read().unwrap();
	        let armr = (*item).data.clone();
	        Some(armr)
   	    },
	}  
    }

    pub fn push(&mut self, component: Armor) {
        match &self.head_link {
            Some(next_node) => {
 	        let new_node = Node{data: component, rest: Some(next_node.clone())};
		self.head_link = Some(Arc::new(RwLock::new(new_node)));

	    },
            None => {
	        self.head_link = Some(Arc::new(RwLock::new(
		    Node{
	             data: component,
	             rest: None,
		    }
                )));
           },
       }
       self.size += 1;
   }
	  
    pub fn pop(&mut self) -> Option<Armor> {
      if self.head_link.is_none() {
         return None;
      }

      let new_head = self.head_link.clone();
      let new_clone = new_head.as_ref().unwrap().read().unwrap();
      self.head_link = new_clone.rest.clone();
      self.size -= 1;
      Some(new_clone.data)
    }
}
// Part 3

#[derive(Clone)]
pub struct Suit {
    pub armor: List,
    pub version: i32,
}

impl Suit {
    pub fn is_compatible(&self) -> bool {
        if self.armor.size < 1 {
	   return false
	}
	let mut armr_clone = self.armor.clone();
	while !(armr_clone.peek().is_none()){
	    if armr_clone.peek().unwrap().version != self.version {
	        return false;
	    }
	    armr_clone.pop();
	}
	return true
    }

    pub fn repair(&mut self) {
       let mut armr_clone = self.armor.clone();
       while !(armr_clone.peek().is_none()) {
       {
         let mut iron_man = armr_clone.head_link.as_ref().unwrap().try_write().unwrap();
	 match &mut iron_man.data.component {
	     Component::LeftThrusters(need_rep, status) =>
	     if *need_rep == true {
 	         *status = 100;
	         *need_rep = false;
	     }
	     Component::RightThrusters(need_rep, status) =>
	     if (*need_rep) == true {
	        *status = 100;
	        *need_rep = false;
             }
	     Component::LeftRepulsor(need_rep, status) =>
	     if (*need_rep) == true {
                *status = 100;
	        *need_rep = false;
	     }
	     Component::RightRepulsor(need_rep, status) =>
	     if (*need_rep) == true {
	        *status = 100;
	        *need_rep = false;
             }
	     Component::ChestPiece(need_rep, status) =>
	     if (*need_rep) == true {
	        *status = 100;
	        *need_rep = false
	     }
	     _ => ()  
	 }
       }
         armr_clone.pop();
       }
    }
}
