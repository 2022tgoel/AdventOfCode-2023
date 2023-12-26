const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

const INPUT: &str = "#...#...O....OO.#O.O........#.OO.#.O..O.O.#.#..O#..O...#.#......#..O.....O.###O..OO.#.....O.O.......
O.#..#..O.O...O..#OO#...#.#.OO....O.#.....O.#OOO.#O.#O#O.......O......O.OO.#.#.........O..#.....#...
....O..O..O#...#O#......O...##O#.O.....#O..##..O...#O..O.O..#.O..O..#.....OO....O.O.O...O......#.O.O
##O....O....#.OOO...O.#....O#.O#OO....O....O#...O..O#O.#..O.OO...O..O..#.#O..OO.O...O..#.#OO#.......
.O.O#OO..#...#..........O#.O..##.O..OO.O...#........O##.OOOO..#.#....O..O#O.#O.##..OO..O...#..O##...
OO...O##.....#...O.#O.....#.OOO.#.##...##...##.O.....##....O..O..#..#..OO...#..........OO..#.O#...O.
.OO#..#.O.O##...O....O...O....#.O.O..#.OO........O..O..O..###..O...O#..#..#......OO.O..#O....O##.#..
#O.O..O#...O..O.O.O...#.....O..O......O....OO.OO..O#.O#..#.O......#..O...#.O.##.#....O.OO.#...O...O#
.#.O.....O...O...O...O.#.........OO.....#................O....O..#..........##.O#.OO..O###..#.......
#O...#O..O#.......O...#..O...OO..#...O........#....O#..OO...O.O..#.OOO.....O..O.#...O.OO#.OO#.OO.O#.
......#.O...O...O.O...#...#O.O#....#.O..###..O....#.#.O.OO#....O#.....O...#.O......##......#O..#OO..
#OO.##...#..O....#...#...OO...........OO.O.OOOO...#...#O..#...#O#.O.O......O....O..#.O..O...#O#.O.O.
#...O.OO...#.#.OO...O#O....OO....#O....O.O#..#....O...##..O..O.#..#..#...........O...OO#.#..O.....O#
..#..O.O#.....O....O#..#........OO#.#...#O.O....O#....O#...O..#....OO#.O.#........#O....O.O...##O.#O
....#O.....O#...O.......O.......OO..#.....#...O##.O.......#O#....O...O..O.O.......#.....#..#.O.O.O..
.....O.....O.O.O#O.O##.....OOO#...#O.O#O.....#.O#..........O#O.....O.OOO..##.#O....O.O.....#.O.O#O.#
O#O....O.#.......O.O.OO.#..#...#O#..O...O.......O.#........#O#....O....O.#...#.O.#O...O.#.........O.
........O.O...O.#O....#..O....#O.....OO#..O.O..####..#...O..O...OOO.#OO..#....#...OO..O.....#O..O..#
O#...##...#..#O#..#O#..O#....OO..O#O#.....OOOOOO#.#.O.O..O.OO.O........O...#.#......OO#...#..#.#...#
..OO.O..OO.OO....#........#O.#...#O....#...O.OO...O#..O......O.###....#O......#.....####...#..#.#OO.
..O.O..O#....#.O.#.#.#.O#..O.......OO.........##.O.....#OOO..O#..#OO#.....O...OO.O.O...#...OO#.O.#..
O...#........###..##..O#.#O.O..O.#OO.OO...#....#O.O#O..OO...##....O......OO..#...#.......#...#......
..O..#.O#...OO#O.O#....#.....O#..O..O#..O.O..O.#O.O.#O.#OO..#........#.#O..#O....O..#...#OO#...#.O.O
..#O..O.....#....O.O...O.#O.#.O#.O......#........OO...O....#.O........O#.OO#..........O.O..OOOOO#.#.
O..##.##...........O##.#...#.O.#.OO..O.O...OO#.#..#...O..O....#..#..#...O###......O#.#O#..O.#....O.#
#.O.O.O.###.##......#.OO...OO.O..OOO.O.OOO.....##..O#...O......O.#O..#...#...OO..O#......O....##.O.#
#...#.O.O....O..#...#........##O#O#O..##O.#......O...OO#O...OOOO#O..OO##..#......O.#O.#.O#.OO...O..#
#..#........#O.#O.#...O...O....#OO...O....#...O..#......#..O..O...##........O..#.O..#.#OO..OO.O##.O.
..#...#...OO.#.O#O.O.##..#..##..O#...O#....#..O....OO.OOO..O#.#....O.......OO.#.O#.....O.#O.O.#.....
.......O...O..O...#.O..OO...O.....O.O#..#OO#.O...O..O#O..O..#.OO.......O..O....OOOO..O.#.O#O#..O.##.
.####...O.....#...#......#.O...........O......O.#O.....O.#......#.#...O...#......#..O#.....O.O..O..#
O....O...#OO##.O.O...O....O..O......O.O..O..O..O#O..O.O#O.#O.#..OO.....#.O.#O#..##.#.O.O...OO....#..
....#.O..O..#...#....O............O.O.#O.#.O...O....O.#.OO....O..O##...O##......O....O...OO........#
.#O...O..O.O.#O.......O###OO.O.O.O.....##O..OO...#.O..#OO..#...#...#.O.O###.....#O#..##.#O.....O....
..#.###.#..##......#..#.#OO..O......O..#OOO.#O...#....OO.##O...O..#.#O....#OO....O.#.......#....O...
#OO...#......#..#........#.OO.##....#...OOOO..O#O..O.#..#O........#O.OO#..........OO..OO....O..#O.#.
..##.##.O....#.##O..#...###..O##.#.#......O#O..###..##......O...#.....##.#...........O.O.O....O.#...
...O.#..#......#.OO..#..O##...O..O...OO##...OO.O#....O.O.#.O.....O..#..O..#...O..#O.....O.O..O.O#.##
O......###OOO#.#.....OO...O.....O.##.....OO..........OO#O#.O.#.#O....O..#O.#..........##O...#O....O.
..#...#...OO.OO#.....OO...O.O.O.##.##O##..O.#.O#..O.O.....OOOO....#....O.O.#..##OO.O#..#O.O.#...O...
.#.OO.##....O..O...O..#.##.O#OO..#.....OO.........O.OOO.#..O....O....OOO.#.OO..O..#...O#OO.OO##.#.O.
.#.OO.O#.O.O.....#O..O.O..O......O.#O...O..O...##.O#..#.O.....O...O....OO.OOO..OO.O...OO..#.#..OOO#.
.#..O.#.O..#.OOO....#O#O#............##.#.O...O....O.OO..#.O....O#O#.O.#...#....#..OO.O.O#.#.....O.O
O....O.O#....O..#.......O..#O.##.#...O..O.......O#.#.....#..#..O......O..O.....#............##OO....
O#.O...O.##O..OO..O.O.O.#O.O..##..O#.#..#O..O#O...#...OO..#.O.O....OO..O..#..O.......O.O#....O..#.O#
#.O..##......#..O#O..........##O#.#O..O#.#..#.O..##O.O.#.OO....O#..........O..#.##..#..O.#...#O.OO..
.O..#....OO...O.......#...#...O.##...O..O.O.........OO#O.OO.O..O...O.##.....##.....##O..#......O....
#..O.O..OO....O.O#.#O#........O..OO#.OO#O.O..O..#......O#...##.......#..##..O.#..#.O.O...#....#.O...
#..O.#...OO#.OO#.O.OO.O##.#....OO....#........#.O.O....O.#..O#..#....#.O#..O#..O.....OO..OOOO.....O.
.O..OO#.O#OO..#O..O.#......O..O..O.#O.#..#.#.O..O.O..O.....##.OO..#O.#............O#...#......O.OO.#
O......##OO....O...O......#...O..O.#...OO#O..O.OOO..O.....O...#.##..OOO.##...O.O....O#O....##...#...
..OO..#.O.O..#O.O...O...#...O.O#.##O....##...###O..O...O.O##..........###...O...O..#...#.#.....O....
#..OO....O..#O..OO...OO.............O.O..OOO.O.#.....O#O....#O.....#.O..O....O..O....O..O....O.#....
OO.O##..O...#O........#O...#OOOO.O..O....O...O..O..O#.O.O..##.##..O.#O...O#...#..O...O...OO.......O.
#..O#..O...O.#......#OO.OO.O.#...O...O#...#...O..##O#....#......#.....#..#O...#.O.O..............O#.
##..OO#..........#.O..OOOOO..#...#....#.#..O#O...O....#.O.#O#OO.#......#...O........O...O#....#....O
.....O...O#....O..O..OO.O..#...O.#..#..#...O#..#O....O.#.....#O....O...##....#..#......###.O.....O#.
#...O#..O..O#..OO....#.#...#O#..#.....#..#..................OO....O......OO.........O###O#....OO.O..
O#...O.O.O..##.....##....#....#......#...O.#...O..##...O..O...##..#....#..O..O#...##..O.O..OOO..#...
..#.......#.O..O..#O....#...#.....OOO#...O#...O#.#.#O.....O#O.#.O.#.O......O.#..O.O..O#......O.O..O#
...O.#.#..O.#..###..#...#.....O..O.....OO#..O..#.O..#....OOO...........#.......#..OO..OO#.#O....#O..
.O.O...#....#.#O..#..OO#..#....OO....#O..O#.#..#..O.#..O.#O#..OO........#O.#...OO....OO......#.#O...
OO.O.O#..#.#.#.O.OOO...#....O..O##...O.OO#.#.....O.....#.....O....#.........O...O..........OOO.O.OOO
......O...O...###....##.O.O..OOO..O.O......O.#.......#..O..........O#..#..#O.#.##.....###.#O#O#O..#.
O..#..#O.OO.......#.OO...#OO..OO.O#O.....#.#.O..OOO.OO.##O.O#...#.O......#..#O#O...O#..........O.O#.
OO#...O..#..O.#..O........O.....O.....O........O..#....###..O..O..#...O...#.#.##.......O..#.O#O#O...
.......O..O...O...O.OO#.O.#..OO............O.O.O.#..OO#....##O.....O#.....O##O.....O.#...###O.....#.
.....O....OO..O..#O.....O#..O..O.#.#O#.#O#....O.#O..O#.O#.OO......#.O#..O.O...O...#...#..O....#...#O
#...#....#..O..#.O..#.##O..#O..#..OOO.#..##.....OO.O.....OOO.#..O.#.##O#..O.#..#.#.#....#O#...##....
.OO.O..###....#O....O..#.O.#.......#....O###..O.#......O#....#........O...#O..#..#.........O......##
O#......O#.....#.O....#..#.O.#O.......OO#...#..#.#O.#.O.##......#.#.#..O#O.#...O......O..OOO..OO.###
#O.#.O...O.....O....#OO##...OO..#...OO.#O.OOO.O..#O....O............#....#....#...........O.OOO#.OO.
.O.O.#OO#O.O.OO#..O..#....O.#..O...O....O.#OO#O...##.......O...O.....#.O...#...O.##O...OOO.O........
.O#..O.......#..##..OO#.....O.#.OO#...#O.O..O......O..OO.#...#.#............O#O.OO.......O#........#
.OO#..O##...........O.OO.....#.##....O#.#.#O..O...O..#..#..O.......#......#.#...........###O##......
...#...#.....O........OO..#.#...O.#.....#.OO.....#.O.#OO......#OO..#.....#..O##..O..#...O.O.......O#
O...O...#O..#O#O.......#..#O....#OO....#.#O.....O....O##.#...O.......#.....#........#O.O...O...O....
..#.....OO.#.OO....O#O....O#O...O.O...#O.##....##O...#..##O#.OO#O..OOO.#.#......O.OO.O..O.O.OO.#O.#.
.......O.....##...O#.#O.#....O.....O........#...#.OO...#...OO........#OO#...#..OO..O...#....O#.#OO.O
..#..O.OO....O...#.......OO.O.##...#...O...#.O#.OO#....O...O..O###.O.###O..#....##.O.##....O..##...#
.......OOO.....OO..#..O..#O.OOO#O.....O.O#.....O....###..O#..OO..O....###O.OO...#..#.O....#.O.O..#OO
.#...#.........#.###..#...#.#...#O.#..O#O#OOOO#.#.O.....O.#....#..O......#OO....O..O.O.#.O......O..O
O.O....O....O..O......O...#.....O##....OOO.....#.OO.#..O....O..O.O#.O#.OO......#....#......OO......O
..#O.#O.....O...#O...##.O#..O...#..##.O.O.O....#.....#.O#O.....#............O....#.#...#....###...#.
.O..O...OO.....#...........##...O#.O##O........OO..O.###O.#..#..#.O..O.......O....#O.#.........#...#
..##.O....O...O.O.O#O...#O..#..#.#.O...#O.O.O.....#....#...........O...O....O....OO#O.#.O....#..O..O
.#O#....O.....O#O..O##O#..O..O.O.....O........#.#......O#..O..OO.OO#.....OO........O......O#...O.#..
....#.....O..#......O...##.O.O.#..#.....O.O..#.O#.#O...##....O.......#O..O.....#..#.....#.....#.....
.OO....OOO.OO....O...#.......O......#O.#O..O.O#O..............O.#......#.#.......#....#.O....#.#O.#.
OO...OO.#.O#OO#........##.O.....#..#O.#....O..O..O.O#..........O..O.O....#..O.......O....O....O.OO.O
O.#O.......O#.#...#........O...O.O....O#OO....O.O.O...#.#.O#...#O#O.....##..O#O...#O....O....O#OOO.#
.OO#..#..O..........##.O.#OO#..O.O.#...#....O..O#.O..#.#.....O.O..#..OO..##O#...O.O.O#........O.....
.O............#.O...#...#....###.O#.#.#....O.....OO...#O.#.O..O##...#O.#O..OO#..#.O#.#.....O#.......
#..O#..#.OOO#O...O.O..O..#OOO....#...........O##...OOO##O....O.O...O..O#O.........O.OO.....O....O...
.#....#OO..##O..#O##O.#O....#......O.OO..........##.OO#O...#.....O.O.....O#..OO.#........O..O.O..O..
....O.##...#.O.........O.OO....#....##O..#O.......#..O.##O#..#O..#.....O.O#O....#...O..O.OO.........
..###.O.....##.....OO.O..OO.O..#...O.....#....O.O...............O.##.#.O#O#.#...O..#.OOOOO....#.#.#.
O...O....O.#.#.O#.O#..#......O#.....OO.##....OO.....O.......O....O.O.....#...O..OO#..OO..O....#OOO.#
#.#.#.O..#.O...#....#..O....O......O..#..O.OO#O......O.......O.O.........O...O.......OO.##.......#.#
O#..........O...#O.###....#O..O.#......O....#.O.O...O...O.....O#..#.O#..O.#.O..OOO#.O...#..O.#O.....";

fn calc_load_col(col: usize, chars: &Vec<Vec<char>>) -> usize {
    let mut tot: usize = 0;
    let mut rocks: usize = 0;
    for row in (0..chars.len()).rev() {
        match chars[row][col] {
            'O' => {
                rocks += 1;
            }
            '#' => {
                println!("{} {}", rocks, tot);
                let max = chars.len() - row - 1;
                let min = chars.len() - row - rocks;
                if max >= min {
                    tot += ((max * (max + 1)) / 2) - (((min - 1) * min) / 2);
                }
                rocks = 0;
            }
            '.' => {
                continue;
            }
            _ => {
                panic!("Invalid character: {}", chars[row][col]);
            }
        }
    }
    let max = chars.len();
    let min = chars.len() - rocks + 1;
    if max >= min {
        tot += ((max * (max + 1)) / 2) - (((min - 1) * min) / 2);
    }
    return tot;
}

fn calc_laod(chars: &Vec<Vec<char>>) -> usize {
    let mut tot = 0;
    for col in 0..chars[0].len() {
        let val = calc_load_col(col, &chars);
        // println!("{}: {}", col, val);
        tot += val;
    }
    return tot;
}

fn main() {
    let chars: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    println!("Part 1: {}", calc_laod(&chars));
}