use super::*;

use std::collections::*;

type Input = String;
type Parsed = String;

pub fn part1(input: &str) -> usize {
    let mut allergen_to_ingredients = HashMap::new();
    let mut all_ingredients = HashSet::new();
    let mut foods = vec![];
    for line in input.lines() {
        let mut pieces = line.strip_suffix(")").unwrap().split(" (contains ");
        let ingredients_joined = pieces.next().unwrap();
        let allergens_joined = pieces.next().unwrap();

        let ingredients = ingredients_joined.split(" ").collect::<HashSet<_>>();
        all_ingredients = &all_ingredients | &ingredients;
        let allergens = allergens_joined.split(", ").collect::<HashSet<_>>();
        foods.push(ingredients.clone());

        for allergen in allergens {
            allergen_to_ingredients
                .entry(allergen)
                .or_insert(Vec::new())
                .push(ingredients.clone())
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
            let candidate_ingredient_sets = allergen_to_ingredients.get(allergen).unwrap();

            // take the intersection among all of those sets
            let set = candidate_ingredient_sets[0].clone();
            let candidates_for_allergen = candidate_ingredient_sets[1..]
                .iter()
                .fold(set, |acc, x| &acc & x);

            // if the intersection is of size 1, then we've found a match
            if candidates_for_allergen.len() == 1 {
                // take out the ingredient now from all of the lists of candidates
                let ingredient_to_remove = candidates_for_allergen.iter().next().unwrap().clone();
                ingredient_to_allergen.insert(ingredient_to_remove, allergen);
                for (_, v) in allergen_to_ingredients.iter_mut() {
                    for set in v {
                        set.remove(ingredient_to_remove);
                    }
                }
            }
        }
    }

    println!("all_ingredients {:?}", all_ingredients);
    let mut not_allergic = all_ingredients.clone();
    for (found, _) in ingredient_to_allergen.iter() {
        not_allergic.remove(found);
    }
    println!("not allergic {:?}", not_allergic);
    let mut answer = 0;
    for ingredient in not_allergic {
        for food in &foods {
            if food.contains(&ingredient) {
                answer += 1;
            }
        }
    }

    let sorted_by_allergens = ingredient_to_allergen
        .iter()
        .sorted_by_key(|&(&k, &&v)| v)
        .map(|(&k, _)| k)
        .collect::<Vec<&str>>()
        .join(",");

    println!("SOL: {}", sorted_by_allergens);

    answer
}

pub fn part2(input: &Parsed) -> usize {
    5
}

pub fn parse(s: &Input) -> &Parsed {
    s
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
        // assert_eq!(part2(&parse(&test_input)), 5);
    }

    #[test]
    fn day21() {
        let input = get_input(2020, 21).unwrap();
        assert_eq!(part1(&parse(&input)), 2230);
        // assert_eq!(part2(&parse(&input)), 5);
    }
}
