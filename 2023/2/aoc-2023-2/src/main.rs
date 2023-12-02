use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
struct Game {
    id: i32,
    red: i32,
    blue: i32, 
    green: i32,
    possible: bool,
}

impl Game {
    fn update_value_if_greater(&mut self, field_name: &str, val: i32) {
        match field_name {
            "green" => if val > self.green { self.green = val },
            "red" =>   if val > self.red { self.red= val},
            "blue" => if val > self.blue { self.blue = val},
            _ => ()
        };
    }
}
fn main() {
    let mut all_games: HashMap<i32, Game> = HashMap::new();
    let input = include_str!("../input.txt"); 
    let lines = input.lines().map(|line| line.trim());
    for line in lines {
        let g = get_game_from_line(line);
        all_games.insert(g.id, g);
    }

    let mut accum = 0;
    for (id, game) in all_games {
        if game.possible {
            accum = accum + id;
        }
    }
    println!("accum value={}", accum);
}

fn get_game_from_line(line: &str) -> Game {
    let mut g = Game{id: 0, red: 0, blue: 0, green: 0, possible: true};
    let v: Vec<_> = line.split(':').collect(); 
    let game_num_vec: Vec<&str> = v[0].split_whitespace().collect();
    let game_num = game_num_vec[1].parse().unwrap_or(0);
    g.id = game_num;
    let games: Vec<&str> = v[1].split(';').collect();
    for game in games {
        let vals: Vec<&str> = game.split(',').collect();
        for val in vals { 
            let v: Vec<&str> = val.split_whitespace().collect();
            let v_num: i32 = v[0].parse().unwrap_or(0); 
            g.update_value_if_greater(v[1], v_num);
        }
    }
    if g.red > 12 || g.green > 13 || g.blue > 14 {
        g.possible = false
    }
    g
}

#[cfg(test)]
mod tests {
    use super::get_game_from_line;
    use super::Game;
    #[test]
    fn test_getting_game_from_line() {
        let tests: Vec<(&str, Game)> = vec![
        ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game {id: 1, blue:6, green:2, red: 4, possible: true}),
        ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Game {id: 2, blue:4, green:3, red: 1, possible: true}),
        ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Game {id: 3, blue:6, green:13, red: 20, possible: false}),
        ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Game {id: 4, blue:15, green:3, red: 14, possible: false}),
        ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Game {id:5, blue:2, green:3, red: 6, possible: true}),
        ];            
        for test in tests {
            assert_eq!(get_game_from_line(test.0), test.1);
        }
    }

    fn test_fewest_cubes() {

    }
}