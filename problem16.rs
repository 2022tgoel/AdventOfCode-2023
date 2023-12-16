fn parse(string: String) -> Vec<Vec<u32>> {
    let dim1: usize = string.lines().collect::<Vec<_>>().len();
    let dim2: usize = string.lines().next().unwrap().len();
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for i in 0..dim1 {
        grid.push(vec![0; dim2]);
    }
    for (i, line) in string.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => grid[i][j] = 4,
                '|' => grid[i][j] = 1,
                '-' => grid[i][j] = 0,
                '\\' => grid[i][j] = 2,
                '/' => grid[i][j] = 3,
                _ => panic!("Invalid character"),
            }
        }
    }
    return grid;
}

static DR: &'static [i32] = &[1, -1, 0, 0];
static DC: &'static [i32] = &[0, 0, 1, -1];

fn valid(r: usize, c: usize, dir: usize, grid: &Vec<Vec<u32>>) -> bool {
    let nr = r as i32 + DR[dir];
    let nc = c as i32 + DC[dir];
    // println!("{} {} {} {} {}", r, c, dir, nr, nc);
    nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

fn dfs(r: usize, c: usize, dir: usize, grid: &Vec<Vec<u32>>, energized: &mut Vec<Vec<Vec<bool>>>) {
    if energized[r][c][dir] {
        return;
    }
    energized[r][c][dir] = true;
    if grid[r][c] == 1 { 
        if dir >= 2 {
            if valid(r, c, 0, grid) {
                dfs(add(r, DR[0]), add(c, DC[0]), 0, grid, energized)
            }
            if valid(r, c, 1, grid) {
                dfs(add(r, DR[1]), add(c, DC[1]), 1, grid, energized)
            }
        } else {
            if valid(r, c, dir, grid) {
                dfs(add(r, DR[dir]), add(c, DC[dir]), dir, grid, energized)
            }
        }
    }
    if grid[r][c] == 0 {
        if dir < 2 {
            if valid(r, c, 2, grid) {
                dfs(add(r, DR[2]), add(c, DC[2]), 2, grid, energized)
            }
            if valid(r, c, 3, grid) {
                dfs(add(r, DR[3]), add(c, DC[3]), 3, grid, energized)
            }
        } else {
            if valid(r, c, dir, grid) {
                dfs(add(r, DR[dir]), add(c, DC[dir]), dir, grid, energized)
            }
        }
    }
    if grid[r][c] >= 2 {
        let mut newdir = 0;
        match grid[r][c] {
            2 => {
                newdir = (dir + 2) % 4;
            }
            3 => {
                newdir = ((dir + 2) % 4) ^ 1;
            }
            4 => {
                newdir = dir;
            }
            _ => panic!(),
        }
        if valid(r, c, newdir, grid) {
            dfs(add(r, DR[newdir]), add(c, DC[newdir]), newdir, grid, energized)
        }
    }
}

fn solve(r: usize, c: usize, dir: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let mut energized: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; grid[0].len()]; grid.len()];
    dfs(r, c, dir, &grid, &mut energized);
    let mut tot = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if energized[i][j][0] || energized[i][j][1] || energized[i][j][2] || energized[i][j][3] {
                tot += 1;
            }
        }
    }
    return tot;
}

fn solve_all(grid: &Vec<Vec<u32>>) -> u32 {
    let mut tot = 0;
    for c in 0..grid[0].len() {
        tot = tot.max(solve(0, c, 0, grid));
        tot = tot.max(solve(add(grid.len(), - 1), c, 1, grid));
    }
    for r in 0..grid.len() {
        tot = tot.max(solve(r, 0, 2, grid));
        tot = tot.max(solve(r, add(grid[0].len(), - 1), 3, grid));
    }
    return tot;
}

use std::env;
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("problem16_input.txt").expect("Should have been able to read the file");
    let grid = parse(contents);

    println!("{}", solve_all(&grid));
}