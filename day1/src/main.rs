use std::fs;
use std::vec;
fn main() {
    let nums = fs::read_to_string("src/input.txt").unwrap();
    let mut foods: Vec<Vec<usize>> = Vec::new();
    for x in nums.split("\n\n"){
        for y in x.chars(){
        }
        foods.push(x.split("\n").map(|t| t.parse::<usize>().unwrap()).collect());
    }
    let mut max = [0,0,0];

    for x in foods{
        let mut c = 0;
        for y in x{
            c+=y;
        }
        if c>max[0]{
            max[2]=max[1];
            max[1]=max[0];
            max[0]=c;
        }
        else if c>max[1]{
            max[2]=max[1];
            max[1]=c;
        }
        else if c>max[2]{
            max[2]=c;
        }
    }
    println!("{}", max[0]+max[1]+max[2]);
}
