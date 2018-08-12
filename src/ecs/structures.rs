use super::*;
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::input::mouse;
use ggez::{Context, GameResult};
use nalgebra as na;

use assets::{Assets, DrawableHandle};
use gui;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Trees;
