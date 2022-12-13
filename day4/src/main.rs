use std::fs;


struct Elf_pair{
    elf1: (i32, i32),
    elf2: (i32, i32),
}
fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let mut elfs: Vec<Elf_pair> = Vec::new();
    for x in input.split("\n"){
        let elfy: Vec<&str> = x.split(",").collect();
        let elf1: Vec<&str> = elfy[0].split("-").collect();
        let elf2: Vec<&str> = elfy[1].split("-").collect();

        elfs.push(Elf_pair{
            elf1: (elf1[0].parse().unwrap(),elf1[1].parse().unwrap()),
            elf2: (elf2[0].parse().unwrap(),elf2[1].parse().unwrap()),
        });
    }
    println!("part 1: {}", part1(&elfs));
    println!("part 2: {}", part2(&elfs));
}

fn part1(elfs: &Vec<Elf_pair>) -> i32{
    let mut total = 0;
    for pair in elfs{
        if (pair.elf1.0<=pair.elf2.0 && pair.elf1.1>=pair.elf2.1) || (pair.elf1.0>=pair.elf2.0 && pair.elf1.1<=pair.elf2.1){
            total +=1;
        }
    }
    return total;
}

fn part2(elfs: &Vec<Elf_pair>) -> i32{
    let mut total = 0;
    for pair in elfs{
        if (pair.elf1.0>=pair.elf2.0 && pair.elf1.0<=pair.elf2.1) || (pair.elf1.1>=pair.elf2.0 && pair.elf1.1<=pair.elf2.1) || (pair.elf2.1>=pair.elf1.0 && pair.elf2.1<=pair.elf1.1){
            total +=1;
        }
    }
    return total;
}
