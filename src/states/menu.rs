extern crate amethyst;
use amethyst::prelude::*;

#[derive(Default)]
pub struct Menu;

impl SimpleState for Menu {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Loadnig...");
    }
}

