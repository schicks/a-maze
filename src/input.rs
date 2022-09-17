use std::ops::{BitAnd, BitOr};

use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

#[derive(PartialEq, Debug)]
pub enum GameInput {
    UpRight,
    Up,
    UpLeft,
    DownLeft,
    Down,
    DownRight,
    Interact,
}
/*
* GGRS needs to represent input as "just data" (particularly, it need to implement bytemuck::pod).
* We want to be working with binary representations as little as possible,
* so we're going to implement a bunch of utilities on `GameInput` to handle the conversions.
*/

// derives and repr cargo culted from https://github.com/johanhelsing/matchbox/blob/main/matchbox_demo/src/box_game.rs#L34
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct InputSet(u8);

// These are isolated input representations. It's important that they only have one positive bit.
// 00000000
const INPUT_EMPTY: InputSet = InputSet(0);
// 00000001
const INPUT_UP_RIGHT: InputSet = InputSet(1 << 0);
// 00000010
const INPUT_UP: InputSet = InputSet(1 << 1);
// 00000100
const INPUT_UP_LEFT: InputSet = InputSet(1 << 2);
// 00001000
const INPUT_DOWN_LEFT: InputSet = InputSet(1 << 3);
// 00010000
const INPUT_DOWN: InputSet = InputSet(1 << 4);
// 00100000
const INPUT_DOWN_RIGHT: InputSet = InputSet(1 << 5);
// 01000000
const INPUT_INTERACT: InputSet = InputSet(1 << 6);

impl From<GameInput> for InputSet {
    // this lets us get isolated representations easily, without exposing the constants outside this module.
    fn from(input: GameInput) -> InputSet {
        match input {
            GameInput::UpRight => INPUT_UP_RIGHT,
            GameInput::Up => INPUT_UP,
            GameInput::UpLeft => INPUT_UP_LEFT,
            GameInput::DownLeft => INPUT_DOWN_LEFT,
            GameInput::Down => INPUT_DOWN,
            GameInput::DownRight => INPUT_DOWN_RIGHT,
            GameInput::Interact => INPUT_INTERACT,
        }
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

impl InputSet {
    pub fn contains(self, input: GameInput) -> bool {
        /*
        * Checks whether set contains the given input.
        */
        self & InputSet::from(input) != INPUT_EMPTY
    }
}

const KEYMAP: [(KeyCode, InputSet); 7] = [
    (KeyCode::Q, INPUT_UP_LEFT),
    (KeyCode::W, INPUT_UP),
    (KeyCode::E, INPUT_UP_RIGHT),
    (KeyCode::A, INPUT_DOWN_LEFT),
    (KeyCode::S, INPUT_DOWN),
    (KeyCode::D, INPUT_DOWN_RIGHT),
    (KeyCode::Return, INPUT_INTERACT),
];

fn input(_: In<PlayerHandle>, keys: Res<Input<KeyCode>>) -> Vec<InputSet> {
    let folded = KEYMAP.iter().fold(INPUT_EMPTY, |acc, a| {
        let (key, i) = a;
        if keys.pressed(*key) {
            acc | *i
        } else {
            acc
        }
    });
    vec![folded]
}
