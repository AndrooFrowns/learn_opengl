use std::fmt::{Display, Formatter};
use std::ops::Deref;
use learn_opengl::*;

/// Keeps track of the available runs
pub struct Lister {
    /// List of available runs
    exercises: Vec<Box<dyn runner::Runner>>,
}

/// Possible ways to identify the run
pub enum RunID {
    Numeric { chapter: i32, section: i32 },
    Named(String),
}

/// Error for when the run is not found
pub struct RunIDNotFound;

impl Lister {
    /// Creates a new list of the available runs for launching
    pub fn new() -> Self {
        let exercises: Vec<Box<dyn runner::Runner>> = vec![
            Box::new(exercises::creating_a_window::CreatingAWindow),
            Box::new(exercises::hello_window::HelloWindow),
            Box::new(exercises::hello_triangle::HelloTriangle),
            Box::new(exercises::shader::Shader),
            Box::new(exercises::textures::Textures),
        ];

        Lister { exercises }
    }

    /// launches a run based on the id
    ///
    /// # Arguments
    ///
    /// * `id` the identification of which run to launch
    pub fn launch(&self, id: RunID) -> Result<(), RunIDNotFound> {
        match id {
            RunID::Numeric { chapter, section } => {
                let matching_element = self.exercises.iter().find(|element| element.chapter() == chapter && element.section() == section);
                match matching_element {
                    Some(element) => {
                        element.run();
                        Ok(())
                    }
                    None => Err(RunIDNotFound)
                }
            }
            RunID::Named(name) => {
                let matching_element = self.exercises.iter().find(|element| element.name() == name);
                match matching_element {
                    Some(element) => {
                        element.run();
                        Ok(())
                    }
                    None => Err(RunIDNotFound)
                }
            }
        }
    }
}

impl Display for Lister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.exercises.iter().map(|b| describe_runner(b.deref())).collect::<Vec<String>>().join("\n"))
    }
}

fn describe_runner(runner: &dyn runner::Runner) -> String {
    let chapter = runner.chapter();
    let section = runner.section();
    let name = runner.name();
    format!("Ch: {chapter}, S: {section}, Name: \"{name}\"")
}
