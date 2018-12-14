fn main() {
    let input = 765071;

    shared::report_result_with_measurement("Part 1", || {
        format!("{} recipes", recipes_after(input))
    });
    shared::report_result_with_measurement("Part 2", || {
        format!("{} recipes", num_recipes_until(&input.to_string()))
    });
}

fn num_recipes_until(recipe_search: &str) -> usize {
    let recipe_search_digits = recipe_search
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut reversed_recipe_search_digits = recipe_search_digits.clone();
    reversed_recipe_search_digits.reverse();

    let mut a = 0;
    let mut b = 1;
    let mut recipes: Vec<usize> = vec![3, 7];
    loop {
        let current_a_recipe = recipes[a];
        let current_b_recipe = recipes[b];
        {
            let mut new_recipes: Vec<usize> =
                shared::get_digits(current_a_recipe + current_b_recipe);

            if recipes.len() > recipe_search_digits.len() {
                'search: for from_new in 0..new_recipes.len() {
                    for (index, search_digit) in reversed_recipe_search_digits.iter().enumerate() {
                        let reference_digit = if index <= from_new {
                            new_recipes[from_new - index]
                        } else {
                            recipes[recipes.len() - index + from_new]
                        };
                        if reference_digit != *search_digit {
                            continue 'search;
                        }
                    }
                    let recipes_before = recipes.len() - recipe_search_digits.len() + from_new + 1;
                    return recipes_before;
                }
            }

            recipes.append(&mut new_recipes);
        }
        a = (a + 1 + current_a_recipe) % recipes.len();
        b = (b + 1 + current_b_recipe) % recipes.len();
    }
}

#[test]
fn test_num_recipes_until() {
    assert_eq!(num_recipes_until("51589"), 9);
    assert_eq!(num_recipes_until("01245"), 5);
    assert_eq!(num_recipes_until("92510"), 18);
    assert_eq!(num_recipes_until("59414"), 2018);
}

fn recipes_after(n: usize) -> String {
    let mut a = 0;
    let mut b = 1;
    let mut recipes: Vec<usize> = vec![3, 7];
    for _ in 0..(n - 2 + 10) {
        let current_a_recipe = recipes[a];
        let current_b_recipe = recipes[b];
        {
            let mut new_recipes: Vec<usize> =
                shared::get_digits(current_a_recipe + current_b_recipe);
            recipes.append(&mut new_recipes)
        }
        a = (a + 1 + current_a_recipe) % recipes.len();
        b = (b + 1 + current_b_recipe) % recipes.len();
    }

    recipes[n..]
        .iter()
        .take(10)
        .map(|x| x.to_string())
        .collect::<String>()
}

#[test]
fn test_recipes_after() {
    assert_eq!(recipes_after(9), "5158916779".to_string());
    assert_eq!(recipes_after(5), "0124515891".to_string());
    assert_eq!(recipes_after(18), "9251071085".to_string());
    assert_eq!(recipes_after(2018), "5941429882".to_string());
}
