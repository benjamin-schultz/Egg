use std::error::Error;
use std::fs;
use std::collections::HashMap;

const ANIMALS_FILE: &str = "./animals.txt";


pub fn run(name: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
    let animals = load_animals()?;

    let res = find_animals(&name, &animals);

    Ok(res)
}

// Load list of animals from file
fn load_animals() -> Result<String, Box<dyn Error>> {
    let result = fs::read_to_string(ANIMALS_FILE)?.to_lowercase();
    Ok(result)
}

// Main function to determine which animals match the names best
fn find_animals<'a>(names: &'a [String], animals: &str) -> Vec<String> {
    let mut chosen_animals = Vec::new();

    let frags = build_frags(&names);

    for frag in frags {
        pick_animals(&frag, &animals, &mut chosen_animals);
    }

    let ranked = order_animals(&mut chosen_animals);

    ranked.iter().map(|(a, _)| a.clone()).collect()
}

fn order_animals(set: &mut Vec<(String, usize)>) -> &Vec<(String, usize)> {
    set.sort_by(|a, b| b.1.cmp(&a.1));
    set
}

// Create a vector of all the names and their fragments
fn build_frags(names: &[String]) -> Vec<&str> {
    let mut result = Vec::new();

    let max_size = names.iter().max_by_key(|s| s.len()).unwrap().len();
    for i in (0..max_size + 1).rev() {
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
fn pick_animals(frag: &str, animals: &str, set: &mut Vec<(String, usize)>) {
    for animal in animals.lines() {
        if animal.contains(frag) {
            // Find any existing examples of this animal with different cases
            let existing = set.iter().position(|x| animal.eq_ignore_ascii_case(&x.0));
            let mut count = frag.len();
            // If the animal is already in here, capitalize the fragment we are looking at and replace
            if let Some(index) = existing {
                let mut cur = set[index].clone();
                let mut cur_name = cur.0;
                let cur_count = cur.1;
                set.remove(index);
                cur_name = cur_name.replacen(frag, &frag.to_uppercase(), 1);
                set.push((cur_name, count + cur_count));
            } else {
                // If the animal is new, capitalize the area of interest and insert
                let cur_name = animal.replacen(frag, &frag.to_uppercase(), 1);
                set.push((cur_name, count));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn name_is_animal() {
        let name = ["cattle".to_string()];
        let animals = load_animals().expect("load file error");
        let matches = find_animals(&name, &animals);

        println!("{}", &&name[0][..]);
        println!("{:?}", matches);

        assert!(matches.contains(&"CATTLE".to_string()));
    }

    #[test]
    fn animal_contains_name() {
        let name = ["cat".to_string()];
        let animals = load_animals().expect("load file error");
        let matches = find_animals(&name, &animals);

        println!("{}", &name[0]);
        println!("{:?}", matches);

        assert!(matches.contains(&"CAT".to_string()));
    }

    #[test]
    fn partial_name() {
        let name = ["cat".to_string()];
        let animals = load_animals().expect("load file error");
        let matches = find_animals(&name, &animals);

        println!("{:?}", name);
        println!("{:?}", matches);

        // Find all animals with a C
        let mut results = Vec::new();
        for animal in animals.lines() {
            if animal.contains('c') {
                results.push(animal);
            }
        }

        let mut matches_comp = Vec::new();
        for word in matches {
            matches_comp.push(word.to_lowercase());
        }
        matches_comp.sort();
        results.sort();

        assert_eq!(results, matches_comp);
    }

    #[test]
    fn multiple_partial_names() {
        let names = ["weird".to_string(), "batter".to_string()];
        let animals = load_animals().expect("load file error");
        let matches = find_animals(&names, &animals);

        println!("{:?}", names);
        println!("{:?}", matches);

        assert!(matches.contains(&"WomBAT".to_string()));
    }

    #[test]
    fn test_for_dups() {
        let names = ["test".to_string(), "something".to_string()];
        let animals = load_animals().expect("Load file error");
        let matches = find_animals(&names, &animals);

        assert!(has_unique_elements(matches));
    }

    fn has_unique_elements(iter: Vec<String>) -> bool {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x.to_lowercase()))
    }
}
