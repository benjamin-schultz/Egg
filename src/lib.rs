use std::error::Error;
use std::fs;
use std::collections::HashSet;

const ANIMALS_FILE: &str = "./animals.txt";

pub fn run(name: &[String]) -> Result<(), Box<dyn Error>> {
    let animals = load_animals()?;

    find_animals(&name, &animals);

    Ok(())
}

// Load list of animals from file
fn load_animals() -> Result<String, Box<dyn Error>> {
    let result = fs::read_to_string(ANIMALS_FILE)?.to_lowercase();
    Ok(result)
}

// Main function to determine which animals match the names best
fn find_animals<'a>(names: &[String], animals: &'a str) -> Vec<&'a str> {
    let mut chosen_animals = HashSet::new();

    let frags = build_frags(&names);

    for frag in frags {
        let nextset = pick_animals(&frag, &animals);
        chosen_animals.extend(nextset);
    }

    chosen_animals.into_iter().collect::<Vec<&str>>()
}

// Create a vector of all the names and their fragments
fn build_frags(names: &[String]) -> Vec<&str> {
    let mut result = Vec::new();

    let max_size = names.iter().max_by_key(|s| s.len()).unwrap().len();
    for i in (0..max_size+1).rev() {
        for name in names {
            if i <= name.len() && i != 0 {
                result.push(&name[..i])
            }
        }
    }

    result
}

// Reads through the list of animals and extracts any animals that match the fragments
// A hashset is used to remove duplicates
fn pick_animals<'a>(frag: &str, animals: &'a str) -> HashSet<&'a str> {
    let mut chosen_animals = HashSet::new();

    for animal in animals.lines() {
        if animal.contains(frag) {
            chosen_animals.insert(animal);
        }
    }

    chosen_animals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_animal() {
        let name = ["cattle".to_string()];
        let animals = load_animals().expect("load file error");
        let matches = find_animals(&name, &animals);

        println!("{}", &&name[0][..]);
        println!("{:?}", matches);

        assert!(matches.contains(&&name[0][..]));
    }

    #[test]
    fn animal_contains_name() {
        let name = ["cat".to_string()];
        let animals = load_animals().expect("load file error");
        let matches = find_animals(&name, &animals);

        println!("{}", &name[0]);
        println!("{:?}", matches);

        assert!(matches.contains(&&name[0][..]));
    }

    #[test]
    fn partial_name() {
        let name = ["cat".to_string()];
        let animals = load_animals().expect("load file error");
        let mut matches = find_animals(&name, &animals);

        println!("{:?}", name);
        println!("{:?}", matches);

        // Find all animals with a C
        let mut results = Vec::new();
        for animal in animals.lines() {
            if animal.contains('c') {
                results.push(animal);
            }
        }

        matches.sort();
        results.sort();

        assert_eq!(results, matches);
    }

    #[test]
    fn multiple_partial_names() {
        let names = ["weird".to_string(), "batter".to_string()];
        let animals = load_animals().expect("load file error");
        let matches = find_animals(&names, &animals);

        println!("{:?}", names);
        println!("{:?}", matches);

        assert!(matches.contains(&"wombat"));
    }
}
