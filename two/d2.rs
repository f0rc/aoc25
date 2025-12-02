use std::fs;


fn main() {
    let contents= fs::read_to_string("./in")
    .expect("rip file");

    // let x = "0101";

    // let y: i32 = x.parse().unwrap_or_default();
    // println!("{}", y);


    // let parts: Vec<&str> = contents.split(",").collect();

    // // println!("{}", parts.len());

    // let mut sum: i128 = 0;

    // for part in parts {
    //     let part = part.trim();


    //     let range: Vec<&str> = part.split('-').collect();
    //     let start: i128 = range[0].parse().expect("bad1");
    //     let mut end : i128 = range[1].parse().expect("bad");
    //     end+=1;

    //     // println!("{start} {end}");
        

    //     for i in start..end {
    //         let s = i.to_string();
    //         if s.len() % 2 == 1 {
    //             continue;
    //         }

    //         // println!("{}", i);

    //         let s_len = s.len().div_euclid(2);
    //         let s1 = &s[0..s_len];
    //         let s2 = &s[s_len..];
            
    //         if s1 == s2 {
                
    //             sum += i;
    //         }
    //     }
    // }

    // println!("{sum}")



    // not 34558395177
    // p1: 34826702005 ( errored on parse unrap or default gg)



    // p2: 
    let parts: Vec<&str> = contents.split(",").collect();

    let mut sum: i128 = 0;

    for part in parts {
        let part = part.trim();


        let range: Vec<&str> = part.split('-').collect();
        let start: i128 = range[0].parse().expect("bad1");
        let mut end : i128 = range[1].parse().expect("bad");
        end+=1;

        for i in start..end {
            let s = i.to_string();
            let s_len = s.len();

            for p_len in 1..=s_len/2 {
                if s_len % p_len != 0 {
                    continue;
                }

                let p = &s[0..p_len];
                let new_s = p.repeat(s_len / p_len);

                if new_s == s {
                    sum += i;
                    break;
                }
            }
        }

        // p2: 43287141963
    }

    println!("{sum}")


}