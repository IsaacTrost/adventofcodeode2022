use std::fs;

#[derive(PartialEq)]
enum command{
    noop,
    addx,
}

struct CommandV{
    kind: command,
    value: i32,
}
fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let commands: Vec<CommandV> = input.split("\n").map(|x| if x.split(" ").next().unwrap()=="noop"{
        CommandV{
            kind: command::noop,
            value: 0,
        }
    }else{
        CommandV{
            kind: command::addx,
            value: x.split(" ").nth(1).unwrap().parse().unwrap(),
        }
    }).collect();
    println!("part 1: {}", part_1(&commands));
    part_2(&commands);
}

fn part_1(commands: &Vec<CommandV>) -> i32 {
    let mut cycle = 1;
    let mut total = 0;
    let mut reg = 1;
    let mut commands = commands.iter();
    let mut mid = false;
    let mut delay = 0;
    while cycle<=220{
        println!("reg: {}, cycle: {}", reg, cycle);
        if cycle==20 || (cycle-20)%40==0{
            total+=cycle*reg;
            println!("\n{cycle}, {reg}\n")
        }
        if mid{
            mid = false;
            reg+=delay;
            cycle+=1;
            continue;
        }
        let y = commands.next().unwrap();
        if y.kind == command::addx{
            delay=y.value;
            mid=true;
        }
        cycle+=1;
    }
   total
}

fn part_2(commands: &Vec<CommandV>) -> (){
    let mut commands = commands.iter();
    let mut cycle = 0;
    let mut reg = 1;
    let mut mid = false;
    let mut delay = 0;
    while cycle<=240{
        let mut pixel = '.';
        if cycle%40<=reg+1 && cycle%40>=reg-1{
            pixel = '#';
        }
        print!("{pixel} ");
        if (cycle+1)%40==0{
            print!("\n")
        }
        if mid{
            mid = false;
            reg+=delay;
            cycle+=1;
            continue;
        }
        let y = commands.next().unwrap();
        if y.kind == command::addx{
            delay=y.value;
            mid=true;
        }
        cycle+=1;
    }
}