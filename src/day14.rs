use std::{collections::HashMap, str::FromStr, fmt};

use adventools::prelude::*;

pub struct D {}
impl DayParsed for D {}

impl Day for D {
    fn number(&self) -> u8 {
        14
    }

    fn part01(&self) -> Result<()> {
        let recipes = self.input_as::<Recipe>()?;
        println!("{}", find_ore_for_fuel(&recipes));
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let recipes = self.input_as::<Recipe>()?;
        println!("{}", find_fuel_for_ore(&recipes));
        Ok(())
    }
}

type IntType = i64;

struct Recipe {
    inputs: HashMap<String, IntType>,
    outputs: HashMap<String, IntType>
}

impl FromStr for Recipe {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=>");
        let inputs = recipe_map(split.next().unwrap());
        let outputs = recipe_map(split.next().unwrap());
        Ok(Recipe { inputs, outputs })
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Recipe<inputs={:?}, outputs={:?}>", self.inputs, self.outputs))
    }
}

fn recipe_entry(e: &str) -> (String, IntType) {
    let mut c_t = e.trim().split(" ");
    let count = c_t.next().unwrap().parse::<IntType>().unwrap();
    let token = c_t.next().unwrap().to_string();
    (token, count)
}

fn recipe_map(side: &str) -> HashMap<String, IntType> {
    side.split(",").map(recipe_entry).collect()
}


fn recipes_by_output(recipes: &Vec<Recipe>) -> HashMap<String, &Recipe> {
    let mut by_output: HashMap<String, &Recipe> = HashMap::new();
    for r in recipes {
        for k in r.outputs.keys() {
            by_output.insert(k.to_string(), r);
        }
    }
    by_output
}

fn apply_recipe(r: &Recipe, processed: &mut HashMap<String, IntType>, count: IntType) {
    // println!("applying {} x{}", r, count);
    for (k, v) in r.inputs.iter() {
        *processed.entry(k.to_string()).or_insert(0) += v * count;
    }
    for (k, v) in r.outputs.iter() {
        *processed.entry(k.to_string()).or_insert(0) -= v * count;
    }
}


fn find_ore_for_fuel(recipes: &Vec<Recipe>) -> IntType {
    let by_output = recipes_by_output(recipes);
    let ore = "ORE".to_string();
    let mut processed: HashMap<String, IntType> = HashMap::new();
    processed.insert("FUEL".to_string(), 1);
    loop {
        let needed: HashMap<_, _> = processed
            .iter()
            .filter(|(_k, &v)| v > 0)
            .collect();
        if needed.len() == 1 && needed.contains_key(&ore) {
            // don't need anything else
            return *needed[&ore];
        }
        let missing = needed.keys().filter(|k| k != &&&"ORE").next().unwrap();
        let r = by_output[*missing];
        apply_recipe(r, &mut processed, 1);
    }
}

fn ceil_div(q: IntType, d: IntType) -> IntType {
    if q % d == 0 {
        return q / d;
    } else {
        return (q / d) + 1;
    }
}

fn find_fuel_for_ore(recipes: &Vec<Recipe>) -> IntType {
    const STORED_ORE: IntType = 1000000000000;
    let by_output = recipes_by_output(recipes);
    // let's make an estimate!
    let est = STORED_ORE / find_ore_for_fuel(recipes);

    let mut fuel_counter: IntType = est;
    let mut processed: HashMap<String, IntType> = HashMap::new();
    // println!("est {}", est);
    let ore = "ORE".to_string();

    processed.insert(ore.to_string(), -STORED_ORE);
    processed.insert("FUEL".to_string(), est);

    loop {
        // println!("{} {} {:?}", fuel_counter, processed[&ore], processed);
        loop {
            let needed: HashMap<_, _> = processed
                .iter()
                .filter(|(k, &v)| k != &"ORE" && v > 0)
                .collect();
            if needed.is_empty() {
                break;
            }
            let ing = *needed.keys().next().unwrap();
            let recipe = by_output[&ing.to_string()];
            let iters = ceil_div(*needed[ing], recipe.outputs[ing]);
            apply_recipe(recipe, &mut processed, iters);
        }
        if processed[&ore] > 0 {
            break;
        }
        let ore_avail = -processed[&ore];
        let ore_consumed = STORED_ORE - ore_avail;
        let next_est = (ore_avail / (ore_consumed / fuel_counter) * 7 / 10).max(1);

        fuel_counter += next_est;
        *processed.get_mut("FUEL").unwrap() += next_est;
    }
    fuel_counter - 1
}



#[cfg(test)]
mod tests {
    use super::*;

    fn test_inputs() -> Vec<(IntType, IntType, Vec<String>)> {
        vec![(31, 0, split_str(r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL")),
            (165, 0, split_str(r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL")),
            (13312, 82892753, split_str(r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT")),
            (180697, 5586022, split_str(r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF")),
            (2210736, 460664, split_str(r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX")),
        ]
    }
    #[test]
    fn test_p1() {
        for (expected, expected_fuel, lines) in test_inputs() {
            let recipes = parse_lines::<Recipe>(&lines).unwrap();
            assert_eq!(find_ore_for_fuel(&recipes), expected);
            if expected_fuel > 0 {
                assert_eq!(find_fuel_for_ore(&recipes), expected_fuel);

            }
        }

    }
}
