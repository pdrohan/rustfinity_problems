use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
        let animals = registry
            .entry(section.to_string())
            .or_default();

        if !animals.contains(&animal.to_string()) {
            animals.push(animal.to_string());
        }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    let mut animals = registry.get(section).cloned().unwrap_or_default();
    animals.sort();
    animals
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut all_animals: Vec<String> = Vec::new();


    for (_key, value) in registry.iter() {
        all_animals.extend(value.clone());
    }
    all_animals.sort();
    return all_animals
}