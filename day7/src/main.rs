use std::fs;
use std::collections::HashMap;
fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let mut all_dirs = HashMap::new();
    let mut stack = Vec::new();
    let mut counter = 0;
    for line in input.split("\n"){
        let chary: Vec<&str> = line.split(" ").collect();
        if chary[1] == "cd"{
            if chary[2]==".."{
                stack.pop();
            }
            else{
                all_dirs.insert(counter, 0);
                stack.push(counter);
                counter+=1;
            }
        }
        else if chary[0].chars().next().unwrap().is_numeric(){
            let addy = chary[0].parse::<usize>().unwrap();
            for dir in &stack{
                *all_dirs.get_mut(dir).unwrap()+=addy;
            }
        }
    }
    println!("part 1: {}", part_1(&all_dirs));
    println!("part 2: {}", part_2(&all_dirs));
}

fn part_1(all_dirs: &HashMap<i32, usize>) ->usize{
    
    let mut returny = 0;
    for (dir, size) in all_dirs.into_iter(){
        if size<=&100000{
            returny+=size;
        }
    }
    return returny;
}

fn part_2(all_dirs: &HashMap<i32, usize>) -> usize{
    let space = 30000000-(70000000-all_dirs[&0]);
    let mut best = 70000000;
    for (dir, size) in all_dirs.into_iter(){
        if *size>=space{
            if *size-space<best-space{
                best = *size;
            }
        }
    }
    best
}
