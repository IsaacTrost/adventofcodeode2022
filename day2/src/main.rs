use std::fs;

fn main() {
    let nums = fs::read_to_string("src/input.txt").unwrap();
    let mut moves: Vec<(i32, i32)> = Vec::new();
    for x in nums.split("\n"){
        let c: Vec<char>=x.chars().collect();
        moves.push(((c[0] as i32)-64, (c[2] as i32)-87));
    }
    for x in 0..moves.len(){
        if moves[x].1==1{
            moves[x].1=(moves[x].0-1);
            if moves[x].1==0{
                moves[x].1=3;
            }
        }
        else if moves[x].1==2{
            moves[x].1=moves[x].0;
        }
        else{
            moves[x].1=(moves[x].0+1);
            if moves[x].1==4{
                moves[x].1-=3
            }
        }
    }
    let mut total = 0;
    for x in moves{
        total+=x.1;
        if x.1==x.0{
            total+=3;
        }
        else if x.1-x.0==1 || x.1-x.0==-2{
            total+=6;
            println!("win");
        }
    }
    print!("{}", total);
}
