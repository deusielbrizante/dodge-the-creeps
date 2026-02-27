mod enemy;
mod hud;
mod main_scene;
mod player;

use godot::prelude::*;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
