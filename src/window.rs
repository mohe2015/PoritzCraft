// Copyright (c) 2021 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use crate::{renderer::PoritzCraftRenderer, utils::state_is_pressed};

use nalgebra::{Isometry3, Matrix4, Rotation3, Vector3, Translation3, UnitQuaternion};
use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct PoritzCraftWindow {}

impl PoritzCraftWindow {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let event_loop = EventLoop::new();

        let mut renderer = PoritzCraftRenderer::new(&event_loop);

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                renderer.main_pipeline.recreate_swapchain = true;
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(key_code) = input.virtual_keycode {
                    match key_code {
                        VirtualKeyCode::W => {
                            let test = Isometry3::<f32> {
                                rotation: UnitQuaternion::<f32>::identity(),
                                translation: Translation3::new(-250.0, -250.0, -250.0),
                            };

                            renderer.main_pipeline.view_matrix = renderer.main_pipeline.view_matrix;
                        }
                        _ => (),
                    }
                }
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: _,
                        button: _,
                        ..
                    },
                ..
            } => {}
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position: _, .. },
                ..
            } => {}
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                println!("{delta:?}");

                let rotvec = Vector3::new(delta.1 as f32 * -0.05f32, delta.0 as f32 * 0.05f32, 0.0);
                let rot = Matrix4::new_rotation(rotvec);

                renderer.main_pipeline.view_matrix = rot * renderer.main_pipeline.view_matrix;
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta: _, .. },
                ..
            } => {}
            Event::RedrawEventsCleared => {
                renderer.main_pipeline.render();
            }
            _ => (),
        });
    }
}
