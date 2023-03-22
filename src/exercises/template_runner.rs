use std::sync::mpsc::Receiver;
use glfw::{Action, Context, Key};
use crate::runner::Runner;

// todo: change struct name
pub struct CreatingAWindow;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// todo: change struct name
impl Runner for CreatingAWindow {
    fn chapter(&self) -> i32 { todo!(chapter) }
    fn section(&self) -> i32 { todo!("section") }
    fn name(&self) -> &'static str {
        todo!("name")
        todo!("add to lister.rs")
    }

    fn run(&self) {
        todo!("Run");
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    todo!("Process Events");
}