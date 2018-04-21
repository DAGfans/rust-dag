
// Copyright 2018 The rust-dag Authors
// This file is part of the rust-dag library.
//
// The rust-dag library is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// The rust-dag library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with the rust-dag library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{Arc,RwLock};
use std::fmt;
use std::cmp::Ordering;

/// Structure providing fast access to block data.
///
pub struct Block{
    pub name: String,
    pub height: u64,
    pub size_of_past_set: u64,
    pub prev: HashMap<String, Arc<RwLock<Block>>>,
    pub next: HashMap<String, Arc<RwLock<Block>>>,
}

pub struct MaxMin{
    pub max: u64,
    pub min: u64,
}

impl fmt::Display for Block {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut formated_info = format!("name={},height={},size_of_past_set={},prev={{", self.name, self.height, self.size_of_past_set);

        for (key, _value) in &self.prev {

            let tmp = format!("{},", key);
            formated_info.push_str(&tmp);
        }

        if self.prev.len() > 0 {
            formated_info.pop();
        }
        formated_info.push_str("}");

        write!(f, "{}", formated_info)
    }
}

pub fn append_maps(target: &mut HashMap<String,Arc<RwLock<Block>>>, source: &HashMap<String,Arc<RwLock<Block>>>){

    for (key, value) in source {

        if let Entry::Vacant(v) = target.entry(key.clone()){
            v.insert(Arc::clone(value));
        }
    }
}

/// Remove from the list all the block predecessors and successors which is in the list, self included.
///
pub fn remove_past_future(block: &Block, list: &mut HashMap<String, Arc<RwLock<Block>>>){

    let exist = list.remove(&String::from(block.name.clone()));
    if exist.is_none() {
        return;
    }

    remove_successors(block, list);
    remove_predecessors(block, list);
}


/// Remove from the list all the block successors which is in the list, self not included.
///
fn remove_successors(block: &Block, list: &mut HashMap<String, Arc<RwLock<Block>>>){

    for (_key, value) in &block.next {

        let next = Arc::clone(value);
        let next = next.read().unwrap();

        let exist = list.remove(&String::from(next.name.clone()));
        if exist.is_some() {
            remove_successors(&next, list);
        }
    }
}

/// Remove from the list all the block predecessors which is in the list, self not included.
///
fn remove_predecessors(block: &Block, list: &mut HashMap<String, Arc<RwLock<Block>>>){

    for (_key, value) in &block.prev {

        let prev = Arc::clone(value);
        let prev = prev.read().unwrap();

        let exist = list.remove(&String::from(prev.name.clone()));
        if exist.is_some() {
            remove_predecessors(&prev, list);
        }
    }
}


pub fn sorted_keys_by_height(source: &HashMap<String,Arc<RwLock<Block>>>, reverse: bool) -> Vec<(String, u64)>{

    let mut keys_vec: Vec<(String, u64)> = Vec::new();

    for (_key, value) in source {
        let block = Arc::clone(value);
        let block = block.read().unwrap();

        keys_vec.push((String::from(block.name.clone()), block.height));
    }

    if reverse==true {
        keys_vec.sort_by(|a, b| {
            match a.1.cmp(&b.1).reverse() {
                Ordering::Equal => a.0.cmp(&b.0),
                other => other,
            }
        });
    }else{
        keys_vec.sort_by(|a, b| {
            match a.1.cmp(&b.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                other => other,
            }
        });
    }
    return keys_vec;
}


// Move from the list all the block successors which is in the list, self not included, to the target list.
//
//fn move_successors(block: &Block, list: &mut HashMap<String, Arc<RwLock<Block>>>, target: &mut HashMap<String,Arc<RwLock<Block>>>){
//
//    for (_key, value) in &block.next {
//
//        let next = Arc::clone(value);
//        let next = next.read().unwrap();
//
//        move_successors(&next, list, target);
//
//        let exist = list.remove(&next.name.clone());
//        if exist.is_some() {
//            target.entry(String::from(next.name.clone()))
//                .or_insert(Arc::clone(value));
//        }
//    }
//}