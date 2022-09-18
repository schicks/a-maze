use std::ops::{BitAnd, BitOr};

use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

/*
* GGRS needs to represent input as "just data" (particularly, it need to implement bytemuck::pod).
* We want to be working with binary representations as little as possible,
* so we're going to implement a bunch of utilities on `GameInput` to handle the conversions.
*/

// derives and repr cargo culted from https://github.com/johanhelsing/matchbox/blob/main/matchbox_demo/src/box_game.rs#L34
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct InputSet(u8);

impl InputSet {
    // These are isolated input representations. It's important that they only have one positive bit.
    // This also means that we are limited to 8 inputs for a u8, 16 for u16, etc.
    // 00000000
    pub const EMPTY: InputSet = InputSet(0);
    // 00000001
    pub const UP_RIGHT: InputSet = InputSet(1 << 0);
    // 00000010
    pub const UP: InputSet = InputSet(1 << 1);
    // 00000100
    pub const UP_LEFT: InputSet = InputSet(1 << 2);
    // 00001000
    pub const DOWN_LEFT: InputSet = InputSet(1 << 3);
    // 00010000
    pub const DOWN: InputSet = InputSet(1 << 4);
    // 00100000
    pub const DOWN_RIGHT: InputSet = InputSet(1 << 5);
    // 01000000
    pub const INTERACT: InputSet = InputSet(1 << 6);

    pub fn contains(self, input: InputSet) -> bool {
        /*
         * Checks whether `self` contains all the elements of `input`.
         */
        self & input == input
    }
}

// OR for our InputSet represents set unions.
impl BitOr for InputSet {
    type Output = InputSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

// AND for our InputSet represents set intersections.
impl BitAnd for InputSet {
    type Output = InputSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

// mapping between isolated representations and the keys for creating them.
const KEYMAP: [(KeyCode, InputSet); 7] = [
    (KeyCode::Q, InputSet::UP_LEFT),
    (KeyCode::W, InputSet::UP),
    (KeyCode::E, InputSet::UP_RIGHT),
    (KeyCode::A, InputSet::DOWN_LEFT),
    (KeyCode::S, InputSet::DOWN),
    (KeyCode::D, InputSet::DOWN_RIGHT),
    (KeyCode::Return, InputSet::INTERACT),
];

pub fn input(_: In<PlayerHandle>, keys: Res<Input<KeyCode>>) -> InputSet {
    KEYMAP.iter().fold(InputSet::EMPTY, |acc, a| {
        let (key, i) = a;
        if keys.pressed(*key) {
            acc | *i
        } else {
            acc
        }
    })
}
