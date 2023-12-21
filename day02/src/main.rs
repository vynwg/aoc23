const R: u16 = 12;
const G: u16 = 13;
const B: u16 = 14;

#[derive(Debug)]
struct Ball {
    r: u16,
    g: u16,
    b: u16
}

#[derive(Debug)]
struct Game {
    id: u16,
    plays: Vec<Ball>
}

fn parse(line: &str) -> Game {
    let (game, info) = line.split_once(":").unwrap();
    let (_, id) = game.split_once(" ").unwrap();
    let id = id.parse::<u16>().unwrap();
    let plays: Vec<_> = info
        .split(";")
        .map(|s| s.trim())
        .map(|s| s
            .split(", ")
            .collect::<Vec<_>>()
        )
        .map(|c| {
            let r: Vec<_> = c.iter()
                .find(|&&x| x.contains("red"))
                .unwrap_or(&"0")
                .split(" ")
                .collect();
            let r: u16 = r[0]
                .parse()
                .unwrap();

            let g: Vec<_> = c.iter()
                .find(|&&x| x.contains("green"))
                .unwrap_or(&"0")
                .split(" ")
                .collect();
            let g: u16 = g[0]
                .parse()
                .unwrap();

            let b: Vec<_> = c.iter()
                .find(|&&x| x.contains("blue"))
                .unwrap_or(&"0")
                .split(" ")
                .collect();
            let b: u16 = b[0]
                .parse()
                .unwrap();

            Ball { r, g, b }
        })
        .collect();

    Game { id, plays }
}

fn main() {
    let games = std::fs::read_to_string("input.txt").unwrap();
    let games: Vec<Game> = games
        .lines()
        .map(parse)
        .collect();

    let possible_games: Vec<&Game> = games
        .iter()
        .filter(|game| {
            game.plays
                .iter()
                .all(|b| b.r <= R && b.g <= G && b.b <= B)
        })
        .collect();
    let ids: Vec<u16> = possible_games
        .iter()
        .map(|game| game.id)
        .collect();

    println!("Sum of the IDs of possible games: {:?}", ids.iter().sum::<u16>());


    let least_balls: Vec<_> = games
        .iter()
        .map(|game|
            game.plays
                .iter()
                .fold(Ball { r: 0, g: 0, b: 0 }, |mut acc, n| {
                    acc.r = std::cmp::max(n.r, acc.r);
                    acc.g = std::cmp::max(n.g, acc.g);
                    acc.b = std::cmp::max(n.b, acc.b);
                    acc
                })
        )
        .collect();
    let game_powers: Vec<u16> = least_balls
        .iter()
        .map(|ball| ball.r*ball.g*ball.b)
        .collect();

    println!("Sum of the power of games: {:?}", game_powers.iter().sum::<u16>());
}
