use super::*;

use std::collections::*;

type Input = String;
type Parsed = String;

pub fn part1(input: &str) -> usize {
    let mut allergen_to_ingredients = HashMap::new();
    let mut all_ingredients = HashSet::new();
    let mut ingredient_count = HashMap::new();

    for line in input.lines() {
        let mut pieces = line.strip_suffix(")").unwrap().split(" (contains ");
        let ingredients = pieces.next().unwrap().split(" ").collect::<HashSet<_>>();
        let allergens = pieces.next().unwrap().split(", ").collect::<HashSet<_>>();

        all_ingredients = &all_ingredients | &ingredients;
        ingredients.iter().for_each(|ingredient| {
            let counter = ingredient_count.entry(ingredient.to_owned()).or_insert(0);
            *counter += 1;
        });

        // already calculates all the subset of ingredients that are likely to be allergenic
        for allergen in allergens {
            let entry = allergen_to_ingredients
                .entry(allergen)
                .or_insert(ingredients.clone());
            *entry = &*entry & &ingredients;
        }
    }

    let mut not_allergic = all_ingredients.clone();
    for ingredients in allergen_to_ingredients.values() {
        not_allergic = &not_allergic - ingredients;
    }

    not_allergic
        .iter()
        .map(|k| ingredient_count.get(k).unwrap())
        .sum()
}

pub fn part2(input: &str) -> String {
    let mut allergen_to_ingredients = HashMap::new();
    let mut all_ingredients = HashSet::new();
    let mut ingredient_count = HashMap::new();

    for line in input.lines() {
        let mut pieces = line.strip_suffix(")").unwrap().split(" (contains ");
        let ingredients = pieces.next().unwrap().split(" ").collect::<HashSet<_>>();
        let allergens = pieces.next().unwrap().split(", ").collect::<HashSet<_>>();

        all_ingredients = &all_ingredients | &ingredients;
        ingredients.iter().for_each(|ingredient| {
            let counter = ingredient_count.entry(ingredient.to_owned()).or_insert(0);
            *counter += 1;
        });

        // already calculates all the subset of ingredients that are likely to be allergenic
        for allergen in allergens {
            let entry = allergen_to_ingredients
                .entry(allergen)
                .or_insert(ingredients.clone());
            *entry = &*entry & &ingredients;
        }
    }

    println!("allergen_to_ingredients = {:?}", allergen_to_ingredients);

    let mut ingredient_to_allergen = HashMap::new();
    let allergens = allergen_to_ingredients.keys().copied().collect::<Vec<_>>();
    let allergens_num = allergens.len();

    while ingredient_to_allergen.keys().len() < allergens_num {
        println!(
            "ingredient to allergen found so far: {:?}",
            ingredient_to_allergen.keys().collect::<Vec<_>>()
        );

        // for each allergen
        for allergen in &allergens {
            // get the ingredients sets in which this allergen appears
            let candidates_for_allergen = allergen_to_ingredients.get(allergen).unwrap();

            // if the intersection is of size 1, then we've found a match
            if candidates_for_allergen.len() == 1 {
                // take out the ingredient now from all of the lists of candidates
                let ingredient_to_remove = candidates_for_allergen.iter().next().unwrap().clone();
                ingredient_to_allergen.insert(ingredient_to_remove, allergen);
                for (_, v) in allergen_to_ingredients.iter_mut() {
                    v.remove(ingredient_to_remove);
                }
            }
        }
    }

    ingredient_to_allergen
        .iter()
        .sorted_by_key(|&(&k, &&v)| v)
        .map(|(&k, _)| k)
        .collect::<Vec<&str>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day21() {
        let test_input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(part1(test_input), 5);
        assert_eq!(part2(test_input), "mxmxvkd,sqjhc,fvjkl".to_owned());
    }

    #[test]
    fn day21() {
        let input = get_input(2020, 21).unwrap();
        assert_eq!(part1(&&input), 2230);
        assert_eq!(
            part2(&input),
            "qqskn,ccvnlbp,tcm,jnqcd,qjqb,xjqd,xhzr,cjxv".to_owned()
        );
    }
}
