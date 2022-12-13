use std::fs;

fn main() {
   let datastream:Vec<char> = fs::read_to_string("src/input").unwrap().chars().collect();
    println!("part 1: {}", part_1(&datastream));
    println!("part 2: {}", part_2(&datastream));
}

fn part_1(datastream: &Vec<char>) -> usize{
    let mut marker: usize = 4;
    while true{
        let t = check_sameness(&datastream[marker-4..marker]);
        if t==0{
            return marker;
        }
        else{
            marker+=t;
        }
    }
    return 0;
}

fn part_2(datastream: &Vec<char>) -> usize{
    let mut marker: usize = 14;
    while true{
        let t = check_sameness14(&datastream[marker-14..marker]);
        if t==0{
            return marker;
        }
        else{
            marker+=t;
        }
    }
    return 0;
}


fn check_sameness(subby: &[char])-> usize{
    for x in 0..3{
        for y in x+1..4{
            if subby[x]==subby[y]{
                return x+1;
            }
        }
    }
    return 0;
}

fn check_sameness14(subby: &[char])-> usize{
    for x in 0..13{
        for y in x+1..14{
            if subby[x]==subby[y]{
                return x+1;
            }
        }
    }
    return 0;
}