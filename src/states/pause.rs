extern crate amethyst;
use amethyst::prelude::*;

#[derive(Default)]
pub struct Pause;

impl SimpleState for Pause {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Loadnig...");
    }
}

