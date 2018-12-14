fn main() {
    let input = 765071;

    println!("Part 1: {}", recipes_after(input));
}

fn recipes_after(n: usize) -> String {
    let mut a = 0;
    let mut b = 1;
    let mut recipes: Vec<usize> = vec![3, 7];
    for _ in 0..(n - 2 + 10) {
        let current_a_recipe = recipes[a];
        let current_b_recipe = recipes[b];
        {
            let mut new_recipes: Vec<usize> = (current_a_recipe + current_b_recipe)
                .to_string()
                .chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
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
