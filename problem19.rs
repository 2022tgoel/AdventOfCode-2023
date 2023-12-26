use std::fs;
use std::collections::HashMap;

#[derive(Clone)]
struct Constraint {
    x_min: i32,
    x_max: i32,
    m_min: i32,
    m_max: i32,
    a_min: i32,
    a_max: i32,
    s_min: i32,
    s_max: i32
}

impl Constraint {
    fn new() -> Constraint {
        Constraint {
            x_min: 0,
            x_max: 4001,
            m_min: 0,
            m_max: 4001,
            a_min: 0,
            a_max: 4001,
            s_min: 0,
            s_max: 4001
        }
    }
}


struct Point {
    x: i32,
    m: i32,
    a: i32,
    s: i32
}

struct Path {
    label: String,
    var: String, 
    comp: String,
    bound: i32
}

fn parse(input: &str) -> (HashMap<&str, Vec<Path>>, Vec<Point>) {
    let mut map = HashMap::new();
    let mut it = input.lines().peekable();
    while let Some(line) = it.next() {
        if line == "" {
            break;
        }
        let label = line.split("{").nth(0).unwrap();
        let components = line.split("{").nth(1).unwrap().split("}").nth(0).unwrap();
        let mut paths = Vec::new();
        let mut it = components.split(",").peekable();
        while let Some(path) = it.next() {
            if it.peek().is_none() {
                paths.push(Path { label: String::from(path), var: String::from(""), 
                    comp: String::from(""), bound: 0 });
            } else {
                let constr = path.split(":").nth(0).unwrap();
                let label = path.split(":").nth(1).unwrap();
                paths.push(Path { label: String::from(label), var: constr[0..1].to_string(), 
                    comp: constr[1..2].to_string(), bound: constr[2..].parse::<i32>().unwrap() });
            }
        }
        map.insert(label, paths);
    }
    let mut points = Vec::new();
    while let Some(line) = it.next() {
        let ptext = &line[1..line.len()-1];
        let mut p = Point { x: 0, m: 0, a: 0, s: 0 };
        for var in ptext.split(",") {
            let name = var.split("=").nth(0).unwrap();
            let val = var.split("=").nth(1).unwrap().parse::<i32>().unwrap();
            match name {
                "x" => p.x = val,
                "m" => p.m = val,
                "a" => p.a = val,
                "s" => p.s = val,
                _ => panic!("Bad point")
            }
        }
        points.push(p);
        if it.peek().is_none() {
            break;
        }
    }
    return (map, points);
}

fn check(var1: i32, var2: i32, comp: String) -> bool {
    match comp.as_str() {
        "<" => return var1 < var2,
        ">" => return var1 > var2,
        _ => panic!()
    }
}

fn follow(label: String, map: &HashMap<&str, Vec<Path>>, point: Point) -> bool {
    if label.as_str() == "R" {
        return false;
    } else if label.as_str() == "A" {
        return true;
    }
    for path in map.get(label.as_str()).unwrap() {
        match path.var.as_str() {
            "x" => {
                if check(point.x, path.bound, path.comp.clone()) {
                    return follow(path.label.clone(), map, point);
                }
            }
            "m" => {
                if check(point.m, path.bound, path.comp.clone()) {
                    return follow(path.label.clone(), map, point);
                } 
            }
            "a" => {
                if check(point.a, path.bound, path.comp.clone()) {
                    return follow(path.label.clone(), map, point);
                }   
            }
            "s" => {
                if check(point.s, path.bound, path.comp.clone()) {
                    return follow(path.label.clone(), map, point);
                }                
            }
            _ => {
               return follow(path.label.clone(), map, point); 
            }
        }
    }
    panic!("should not reach this point");
}

fn size(constr: &Constraint) -> i64 {
    return (constr.x_max - constr.x_min - 1) as i64 * (constr.m_max - constr.m_min - 1) as i64 * (constr.a_max - constr.a_min - 1) as i64 * ((constr.s_max - constr.s_min - 1) as i64);
}
use std::cmp;

fn modify_first(path: &Path, lower: &mut i32, upper: &mut i32) -> () {
    match path.comp.as_str() {
        "<" => {
            *upper = cmp::min(*upper, path.bound);
        }
        ">" => {
            *lower = cmp::max(*lower, path.bound);
        }
        _ => panic!("bad constraint")
    }
}

fn modify_last(path: &Path, lower: &mut i32, upper: &mut i32) -> () {
    match path.comp.as_str() {
        "<" => {
            *lower = cmp::max(*lower, path.bound - 1);
        }
        ">" => {
            *upper = cmp::min(*upper, path.bound + 1);
        }
        _ => panic!("bad constraint")
    }
}
fn part2(label: String, map: &HashMap<&str, Vec<Path>>, mut constr: Constraint, ans: &mut i64) -> () {
    // println!("{} {}", label, size(&constr));
    if size(&constr) <= 0 {
        return;
    }
    if label == "A" {
        *ans += size(&constr);
        return;
    } else if label == "R" {
        return;
    }
    for path in map.get(label.as_str()).unwrap() {
        // println!("  {} {} {} {} {} {} {} {}", path.label, path.var, path.comp, path.bound, constr.x_min, constr.x_max, constr.m_min, constr.m_max);
        // add the constraint, recur, remove the contraint
        match path.var.as_str() {
            "x" => {
                let mut new_constr = constr.clone();
                modify_first(path, &mut new_constr.x_min, &mut new_constr.x_max);
                part2(path.label.clone(), map, new_constr, ans);
                modify_last(path, &mut constr.x_min, &mut constr.x_max);
            }
            "m" => {
                let mut new_constr = constr.clone();
                modify_first(path, &mut new_constr.m_min, &mut new_constr.m_max);
                part2(path.label.clone(), map, new_constr, ans);
                modify_last(path, &mut constr.m_min, &mut constr.m_max);       
            }
            "a" => {
                let mut new_constr = constr.clone();
                modify_first(path, &mut new_constr.a_min, &mut new_constr.a_max);
                part2(path.label.clone(), map, new_constr, ans);
                modify_last(path, &mut constr.a_min, &mut constr.a_max);              
            }
            "s" => {
                let mut new_constr = constr.clone();
                modify_first(path, &mut new_constr.s_min, &mut new_constr.s_max);
                part2(path.label.clone(), map, new_constr, ans);
                modify_last(path, &mut constr.s_min, &mut constr.s_max);                           
            }
            _ => {
                part2(path.label.clone(), map, constr.clone(), ans);
            }
        }
    }
}


// fn contraints_to_accept() -> Constraint {

// }

fn main() {
    let contents: String = fs::read_to_string("problem19_input.txt").expect("Should have been able to read the file");
    let (input, points) = parse(&contents);
    // for (label, paths) in input {
    //     print!("{}: ", label);
    //     for path in paths {
    //         print!("{} {} {} goes to {}, ", path.var, path.comp, path.bound, path.label);
    //     }
    //     println!("");
    // }
    // part 1 code 
    let mut ans = 0;
    for p in points {
        let rating = p.x + p.m + p.a + p.s;
        if follow(String::from("in"), &input, p) {
            ans += rating;
        }
    }
    println!("Part 1: {}", ans);
    // part 2 code
    let mut ans: i64 = 0;
    part2(String::from("in"), &input, Constraint::new(), &mut ans);
    println!("Part 2: {}", ans);
}