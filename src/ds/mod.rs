use serde_derive::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CharTupleStruct(pub char, pub char);

#[wasm_bindgen]
pub struct NumTupleStruct(pub i64, pub i64);

#[wasm_bindgen]
pub struct CharAndNumTuple(CharTupleStruct, NumTupleStruct);

#[wasm_bindgen]
#[derive(Serialize, Debug, Clone, Copy)]
pub struct FrequencyType(pub i64, pub i64);

#[wasm_bindgen]
#[derive(Serialize, Debug, Clone, Copy)]
pub enum NodeType {
    Character,
    Number,
    Nil,
}

#[wasm_bindgen]
#[derive(Serialize, Debug, Clone, Copy)]
pub enum ActionTypes {
    add,
    delete,
}

#[wasm_bindgen]
#[derive(Serialize, Debug, Clone, Copy)]
pub struct AddAction {
    pub action_type: ActionTypes,
    pub left_node: NodeType,
    pub right_node: NodeType,
    pub frequencies: FrequencyType,
    pub new_node_freq: i64,
}

#[wasm_bindgen]
impl AddAction {
    pub fn new(
        action_type: ActionTypes,
        left_node: NodeType,
        right_node: NodeType,
        frequencies: FrequencyType,
        new_node_freq: i64,
    ) -> AddAction {
        AddAction {
            action_type,
            left_node,
            right_node,
            frequencies,
            new_node_freq,
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum ValueTypes {
    Character(char),
    Number(i64),
}

#[derive(Serialize, Debug)]
pub struct Node {
    pub value: ValueTypes,
    pub freq: Option<i64>,
    pub right: Option<Box<Node>>,
    pub left: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: ValueTypes, freq: Option<i64>) -> Node {
        Node {
            value,
            freq,
            right: None,
            left: None,
        }
    }
    pub fn update_freq(&mut self, value: i64) {
        match &self.freq {
            Some(num) => self.freq = Some(num + value),
            None => panic!("Frequency Not Set"),
        }
    }
    pub fn get_freq(&self) -> i64 {
        match self.freq {
            Some(value) => value,
            None => panic!("Frequency Not Set"),
        }
    }
}
