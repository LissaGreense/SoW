extern crate amethyst;
use amethyst::prelude::*;

#[derive(Default)]
pub struct Gameplay;

impl SimpleState for Gameplay {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Loadnig...");
    }
}

