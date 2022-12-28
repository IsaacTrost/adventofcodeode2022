use std::fs;
use std::collections::HashMap;
fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let forest: Vec<Vec<u32>> = input.split("\n").map(|line| line.chars().map(|charo| charo.to_digit(10).unwrap()).collect()).collect();
    println!("part 1: {}", part_1(&forest));
    println!("part 2: {}", part_2(&forest));
}

fn part_1(forest: &Vec<Vec<u32>>) -> i32{
    let mut seen_trees = HashMap::new();
    let mut answer = 0;
    let mut currentpos=(0,0);
    for row in forest{
        let mut height: i32 = -1;
        currentpos.1=0;
        for tree in row{
            
            if *tree as i32>height{
                height=*tree as i32;
                if !seen_trees.contains_key(&currentpos){
                    answer+=1;
                    seen_trees.insert(currentpos, true);
                }
                if height==9{
                    break;
                }
            }
            currentpos.1+=1;
        }
        height=-1;
        currentpos.1=99;
        for tree in row.into_iter().rev(){
            
            currentpos.1-=1;
            if *tree as i32>height{
                height=*tree as i32;
                if !seen_trees.contains_key(&currentpos){
                    answer+=1;
                    seen_trees.insert(currentpos, true);
                }
                if height==9{
                    break;
                }
            }
            
        }
        currentpos.0+=1;
    }
    for col in 0..99{
        let mut height: i32 = -1;
        for tree in 0..99{
            currentpos=(tree, col);
            if forest[tree][col] as i32>height{
                height=forest[tree][col] as i32;
                if !seen_trees.contains_key(&currentpos){
                    answer+=1;
                    seen_trees.insert(currentpos, true);
                }
                if height==9{
                    break;
                }
            }
        }
        height=-1;
        for tree in (0..99).rev(){
            currentpos=(tree, col);
            if forest[tree][col] as i32>height{
                height=forest[tree][col] as i32;
                if !seen_trees.contains_key(&currentpos){
                    answer+=1;
                    seen_trees.insert(currentpos, true);
                }
                if height==9{
                    break;
                }
            }
        }
    }
    return answer;
}

fn part_2(forest: &Vec<Vec<u32>>) -> i32{
    let mut highest = 0;
    for row in 1..forest.len() as i32-1{
        println!("{}", row);
        for col in 1..forest[1].len() as i32-1{
            let height = forest[row as usize][col as usize];
            let mut current = 1;
            for x in [(0 as i32,1 as i32),(0,-1),(1,0),(-1,0)]{
                let mut dist = 0;
                for y in 1..forest.len() as i32{
                    if row+ x.0*y< 0 || row + x.0*y > forest.len() as i32-1 || col as i32 + x.1*y <0 || col as i32 + x.1*y>forest[0].len() as i32-1{
                        break;
                    }
                    dist+=1;
                    if forest[(row + x.0*y) as usize][(col + x.1*y) as usize] >= height{
                        break;
                    }
                }
                current*=dist;
            }
            if current>highest{
                highest=current;
            }
        }
    }
    return highest;

    }
