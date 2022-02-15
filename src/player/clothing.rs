use serde::{Deserialize, Serialize};

use crate::utils::color::Color;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Clothing {
    pub hat: u16,
    pub face: u16,
    pub hand: u16,
    pub back: u16,
    pub hair: u16,
    pub ances: u16,
    pub shirt: u16,
    pub pants: u16,
    pub shoes: u16,
    pub chest: u16,

    pub eye_color: Color,
    pub skin_color: Color,
    pub hair_color: Color,
    pub eyelens_color: Color,
}

impl Default for Clothing {
    fn default() -> Self {
        Self {
            hat: 0,
            face: 0,
            hand: 0,
            back: 0,
            hair: 0,
            ances: 0,
            shirt: 0,
            pants: 0,
            shoes: 0,
            chest: 0,

            eye_color: Color::default(),
            skin_color: Color::default(),
            hair_color: Color::default(),
            eyelens_color: Color::default(),
        }
    }
}
