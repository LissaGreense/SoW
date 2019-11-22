use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};
use specs::prelude::*;

enum Button{
    Square { height: i16, width: i16, color: i8 , text:str}
}

impl Component for Button{
    type Storage = DenseVecStorage<Self>;

}
