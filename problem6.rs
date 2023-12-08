const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

const INPUT: &str = "Time:        49     78     79     80
Distance:   298   1185   1066   1181";

fn num_options(time: f64, distance: f64) -> f64 {
    let a: f64 = -1.0;
    let b: f64 = time;
    let c: f64 = -1.0 * distance;
    let discr: f64 = b * b - 4.0 * a * c;
    if discr >= 0.0 {
        let t1: f64 = (-b + discr.sqrt()) / (2.0 * a);
        let t2: f64 = (-b - discr.sqrt()) / (2.0 * a);
        return ((t2 - 0.00001).floor() - (t1 + 0.00001).ceil()) + 1.0;
        
    } else {
        return 0.0;
    }
}


fn main() {
    let first_row: &str = INPUT.lines().next().expect("No first row");
    let second_row: &str = INPUT.lines().nth(1).expect("No second row");
    let mut ans: f64 = 1.0;
    let mut distance_iter = second_row.split_whitespace();
    let mut time_concat = String::new();
    let mut distance_concat = String::new();
    let mut iter = 0;
    for time in first_row.split_whitespace() {
        let thing = distance_iter.next();
        if (iter == 0) {
            iter = 1;
            continue;
        }
        time_concat += time;
        distance_concat += thing.expect("No distance");
        // let time = time.parse::<f64>();
        // let distance = distance_iter.next().expect("No distance").parse::<f64>();
        // match time {
        //     Ok(_) => {
        //         let time: f64 = time.unwrap();
        //         let distance: f64 = distance.unwrap();
        //         // solve the quadratic equation
        //         ans *= num_options(time, distance);
        //     }
        //     Err(_) => {
        //         continue;
        //     }
        // }
    }
    println!("{} {}", time_concat, distance_concat);
    ans = num_options(time_concat.parse::<f64>().unwrap(), distance_concat.parse::<f64>().unwrap());
    println!("{}", ans);
}