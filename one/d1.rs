use std::fs;


fn main() {
    let contents= fs::read_to_string("./input1.txt")
    .expect("rip file");

    // println!("{contents}");
    // p1
    // let mut total = 50;
    // let mut sum = 0;

    // for line in contents.lines() {
    //     let first: char = line.chars().next().unwrap();
    //     let rest: i32 = line[1..].parse().unwrap();

    //     if first == 'L' {
    //         total -= rest;
    //     }else {
    //         total += rest;
    //     }
        

    //     total = (total % 100 + 100) % 100;
        
    //     // println!("{old_total} {total}");
        
    //     if total == 0{
    //         sum += 1
    //     }
    // }

    // p2

    let mut total = 50;
    let mut sum = 0;

    for line in contents.lines() {
        let first: char = line.chars().next().unwrap();
        let rest: i32 = line[1..].parse().unwrap();

        for _ in 0..rest {
            if first == 'L'{
                total = (total -1+100) % 100;
            }else{
                total = (total +1) % 100;
            }
            if total == 0{
                sum += 1
            }
        }
    }

    println!("{sum}")
}

