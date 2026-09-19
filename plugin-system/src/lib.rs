// plugin in this system is any type that implements a specific trait.
// Each plugin will perform a
// specific task, and the system should manage a collection of these plugins,
// executing them in sequence.
// You’ll also address advanced issues like object safety and
// resolving potential conflicts between overlapping trait implementations.

pub trait Plugin {
    // 1. Finish the trait
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
    pub plugins: Vec<Box<dyn Plugin>>,
}

// 3. Implement the PluginManager
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(), // empty
        }
    }

    pub fn add_plugin(&mut self, plg: Box<dyn Plugin>) {
        let x: Option<&Box<dyn Plugin>> = self
            .plugins
            .iter()
            .find(|x| x.name() == plg.name());
        match x {
            Some(_plug) => panic!("Plugin with name '{}' already exists", plg.name()),
            None => self.plugins.push(plg),
        }
    }

    pub fn remove_plugin(&mut self, name_str: &str) -> Option<Box<dyn Plugin>>{
        // ? new for me is shorthand to return nothing if we error or miss values -> works with option and result
        let index: Option<usize> = self
            .plugins
            .iter()
            .position(|x| x.name() == name_str);
        match index {
            Some(val) => Some(self.plugins.remove(val)),
            None => {None}
        }
    }

    pub fn execute_all(&self) {
        for plug in &self.plugins {
            plug.execute();
        }
    }
}

// Example usage
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }
    fn execute(&self) {
        println!("Executing MyPlugin");
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
