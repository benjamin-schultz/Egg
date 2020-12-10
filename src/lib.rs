use std::error::Error;

pub fn run(name: &[String], animals: &str) -> Result<Vec<(String, usize)>, Box<dyn Error>> {
    let res = find_animals(&name, &animals);

    Ok(res)
}

// Main function to determine which animals match the names best
fn find_animals(names: &[String], animals: &str) -> Vec<(String, usize)> {
    let mut chosen_animals = Vec::new();

    for name in names {
        let frags = build_frag_single_word(name);
        let mut offset_vec = Vec::new();
        for frag in frags {
            pick_animals(&frag, &animals, &mut chosen_animals, 5, &mut offset_vec);
        }
    }

    order_animals(&mut chosen_animals).clone()
}

fn order_animals(set: &mut Vec<(String, usize)>) -> &Vec<(String, usize)> {
    set.sort_by(|a, b| b.1.cmp(&a.1));
    set
}

fn build_frag_single_word(name: &String) -> Vec<&str> {
    let mut result = Vec::new();
    for i in (1..name.len()+1).rev() {
        result.push(&name[..i]);
    }
    result
}

// Reads through the list of animals and extracts any animals that match the fragments
// A hashset is used to remove duplicates
fn pick_animals<'a>(frag: &str, animals: &'a str, set: &mut Vec<(String, usize)>, offset: usize, offset_vec: & mut Vec<&'a str>) {

    for animal in animals.lines() {
        if animal.contains(frag) {
            // Find any existing examples of this animal with different cases
            let existing = set.iter().position(|x| animal.eq_ignore_ascii_case(&x.0));
            let mut count = frag.len();
            if !offset_vec.contains(&animal) {
                count = frag.len() + offset;
                offset_vec.push(&animal);
            }

            // If the animal is already in here, capitalize the fragment we are looking at and replace
            if let Some(index) = existing {
                let cur = set[index].clone();
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
