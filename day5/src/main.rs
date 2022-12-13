use std::fs;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let crate_input: String = input.split("\n\n").collect::<Vec<&str>>()[0].to_string();
    let mut crates: Vec<Vec<char>> = Vec::new();
    for x in 0..9{
        crates.push(Vec::new());
    }
    for line in crate_input.split("\n"){
        let chary: Vec<char> = line.chars().collect();
        if chary[1]=='1'{
            continue;
        }
        for x in 0..9{
            if chary[x*4+1]!=' '{
                crates[x].insert(0, chary[x*4+1]);
            }
        }
    }
    let move_input: String = input.split("\n\n").collect::<Vec<&str>>()[1].to_string();
    let mut moves: Vec<(usize, usize, usize)> = Vec::new();
    for line in move_input.split("\n"){
        let splitt: Vec<&str> = line.split(" ").collect();
        
        moves.push((splitt[1].parse::<usize>().unwrap(), splitt[3].parse::<usize>().unwrap(), splitt[5].parse::<usize>().unwrap()));
        
    }
    println!("part 1: {}", part_1(&crates, &moves));
    println!("part 2: {}", part_2(&crates, &moves));
}


fn part_1(crates: &Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    let mut m_crates = crates.clone();
    for movey in moves{
        for y in 0..movey.0{
            let crateo = m_crates[movey.1-1].pop().unwrap();
            m_crates[movey.2-1].push(crateo);
        }
    }
    let mut returny: String = "".to_string();
    for stack in m_crates{
        returny.push(*stack.last().unwrap());
    } 
    return returny;
}


fn part_2(crates: &Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    let mut m_crates = crates.clone();
    for movey in moves{

        let lenny = m_crates[movey.1-1].len()-movey.0;
        let mut stacky: Vec<char> = m_crates[movey.1-1].split_off(lenny);
        m_crates[movey.2-1].append(&mut stacky);
    }

    let mut returny: String = "".to_string();
    for stack in m_crates{
        returny.push(*stack.last().unwrap());
    } 
    return returny;
}
