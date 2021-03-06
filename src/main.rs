// Copyright (c) 2021 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.
pub mod main_pipeline;
pub mod renderer;
pub mod utils;
pub mod window;

use crate::window::PoritzCraftWindow;

pub fn main() {
    PoritzCraftWindow::new().run();
}
