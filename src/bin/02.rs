use itertools::Itertools;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn from(s: &str) -> Self {
        let first_split = s.split(": ").collect_vec();
        let id_parse = first_split[0].replace("Game ", "").parse::<u32>();
        let game_id = match id_parse {
            Ok(n) => n,
            Err(e) => {
                println!(
                    "error parsing {}: {}",
                    first_split[0].replace("Game ", ""),
                    e
                );
                0
            }
        };

        let cube_sets: Vec<CubeSet> = first_split[1]
            .split("; ")
            .into_iter()
            .map(|s| {
                let mut cube_set = CubeSet::new();

                let cubes: Vec<Cube> = s
                    .split(", ")
                    .into_iter()
                    .map(|c| Cube::from(c))
                    .collect_vec();

                for cube in cubes {
                    match cube.color {
                        Color::Red => cube_set.red = Some(cube),
                        Color::Blue => cube_set.blue = Some(cube),
                        Color::Green => cube_set.green = Some(cube),
                        Color::NoColor => println!("NoColor: {:?}", cube),
                    };
                }
                cube_set
            })
            .collect();

        Self {
            id: game_id,
            sets: cube_sets,
        }
    }

    fn is_possible(&self) -> bool {
        for set in self.sets.iter() {
            if !set.is_possible() {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct CubeSet {
    red: Option<Cube>,
    green: Option<Cube>,
    blue: Option<Cube>,
}

impl CubeSet {
    fn new() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
        }
    }

    fn is_possible(&self) -> bool {
        (self.red.as_ref().is_some_and(|x| x.count <= 12) || self.red.is_none())
            && (self.green.as_ref().is_some_and(|x| x.count <= 13) || self.green.is_none())
            && (self.blue.as_ref().is_some_and(|x| x.count <= 14) || self.blue.is_none())
    }
}
#[derive(Debug)]
struct Cube {
    color: Color,
    count: u32,
}

impl Cube {
    fn from(s: &str) -> Self {
        let splt: Vec<&str> = s.split_whitespace().collect();
        let num = splt[0].parse::<u32>();

        Self {
            color: Color::from(splt[1]),
            count: match num {
                Ok(n) => n,
                Err(n) => {
                    println!("error parsing {}: {}", splt[0], n);
                    0
                }
            },
        }
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    NoColor,
}

impl Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "green" => Self::Green,
            _ => Self::NoColor,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().into_iter().map(|s| Game::from(s)).collect();

    let possible_games: Vec<u32> = games
        .iter()
        .filter_map(|g| if g.is_possible() { Some(g.id) } else { None })
        .collect_vec();

    Some(possible_games.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
