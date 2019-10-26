extern crate amethyst;
use amethyst::prelude::*;

#[derive(Default)]
pub struct Load;

impl SimpleState for Load {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
		println!("Loadnig...");
	}
}