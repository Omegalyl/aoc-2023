use std::{fs::File, io::{BufRead, BufReader}, path::Path};

#[derive(Debug)]
struct Bag {
    red_cube: u64,
    green_cube: u64,
    blue_cube: u64,
}

impl Bag {
    fn new() -> Bag {
        Bag{
            red_cube: 0,
            green_cube: 0,
            blue_cube: 0 
        }
    }

    fn new_from_string(turn: String) -> Bag {
        let cubes = turn.split(",");
        let mut bag = Bag::new();
        for cube in cubes {
            let parts: Vec<&str> = cube.split_ascii_whitespace().collect();
            let cube_count = parts[0].trim().parse().expect("msg");
            let cube_color = parts[1].trim();
            if cube_color == "red" {
                bag.red_cube = cube_count
            }
            if cube_color == "green" {
                bag.green_cube = cube_count
            }
            if cube_color == "blue" {
                bag.blue_cube = cube_count
            }
        }
        return bag
    }


}
#[derive(Debug)]
struct Game {
    num: u64,
    turns: Vec<Bag>,
}

impl  Game {
    fn new(num: u64) -> Self {
        Game{
            num,
            turns: Vec::new()
        }
    }

    fn is_possible(&mut self, bag: &Bag) -> bool {
        for turn in &self.turns {
            if bag.green_cube < turn.green_cube {
                return false
            }
            if bag.red_cube < turn.red_cube {
                return false
            }
            if bag.blue_cube < turn.blue_cube {
                return false
            }
        }
        return  true;
    }

    fn new_from_string(num: u64, line: String) -> Game {
        let parts: Vec<&str> = line.split(":").collect();
        let rounds = parts[1].split(";");
        let mut game = Game::new(num);

        for round in rounds{
            let bag = Bag::new_from_string(round.to_string());
            game.turns.push(bag)
        }
        return game
    }
}

pub fn sum_of_ids() {
    let file_path = Path::new("./src/day2/input.txt");
    println!("{}", file_path.display());

    let fp = File::open(file_path).expect("Unable to open file");
    let fp = BufReader::new(fp);

    let mut games: Vec<Game> = Vec::new();
    let game_bag = Bag{red_cube: 12, green_cube: 13, blue_cube: 14 };

    let mut i = 1;
    for line in fp.lines() {
        games.push(Game::new_from_string(i, line.expect("msg")));
        i = i +1;
    }

    let mut num_total: u64= 0;
    for game in &mut games {
        if game.is_possible(&game_bag) {
            println!("Num: {}, Is Possible: {}", game.num.to_owned(), game.is_possible(&game_bag));
            // println!("{:#?}", game.turns);
            num_total += game.num
        }
    }
    println!("Toatl: {}", num_total);


}