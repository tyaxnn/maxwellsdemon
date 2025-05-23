use serde::{Serialize, Deserialize};
use bevy::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Resource)]
pub struct OldScores{
    pub top10 : [f32;10]
}

impl OldScores{
    pub fn top10q(&mut self,input : f32) {
        if input > self.top10[9]{
            self.top10[9] = input;

            self.top10.sort_by(|a, b| b.partial_cmp(a).unwrap());
        }
    }
    pub fn display(&self) -> String{

        let mut out = "".to_string();
        for i in 0..self.top10.len(){
            out = format!("{}{}い : {}\n",out,i + 1,self.top10[i]);
        }

        out
    }
}