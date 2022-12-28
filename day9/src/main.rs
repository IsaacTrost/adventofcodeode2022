
use std::fs;
use std::time::Instant;

struct Movey{
    direction: char,
    steps: i32,
}


fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("src/input").unwrap();
    let mut moves: Vec<Movey> = Vec::new();
    for x in input.split("\n"){
        let splitty: Vec<&str> = x.split(" ").collect();
        moves.push(Movey { direction: splitty[0].chars().next().unwrap(), steps: splitty[1].parse().unwrap() })
    }
    println!("Part 1: {}", part_1(&moves));
    println!("Part 2: {}", part_2(&moves));
    println!("Elapsed: {:.2?}", now.elapsed());
}

struct Tail{
    headpos: Loc,
    visited: Vec<Loc>,
    currentloc: Loc,
}


#[derive(Ord)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
struct Loc{
    x: i32,
    y: i32,
}
fn part_1(moves: &Vec<Movey>) -> i32{
    let mut tail = Tail {headpos: Loc {x: 0, y:0}, visited: Vec::new(), currentloc: Loc {x: 0, y:0}};
    tail.visited.push( Loc {x: 0, y:0});
    for set in moves{
        for x in 0..set.steps{
            if set.direction=='U'{
                tail.headpos.y+=1;
                if tail.headpos.y>1{
                    tail.currentloc.y+=1;
                    tail.headpos.y=1;
                    tail.currentloc.x+=tail.headpos.x;
                    tail.headpos.x=0;
                    match tail.visited.binary_search(&tail.currentloc){
                        Ok(_pos) => {},
                        Err(pos) => tail.visited.insert(pos, tail.currentloc)
                    }
                }
            }
            if set.direction=='D'{
                tail.headpos.y-=1;
                if tail.headpos.y< -1{
                    tail.currentloc.y-=1;
                    tail.headpos.y=-1;
                    tail.currentloc.x+=tail.headpos.x;
                    tail.headpos.x=0;
                    match tail.visited.binary_search(&tail.currentloc){
                        Ok(_pos) => {},
                        Err(pos) => tail.visited.insert(pos, tail.currentloc)
                    }
                }
            }
            if set.direction=='R'{
                tail.headpos.x+=1;
                if tail.headpos.x> 1{
                    tail.currentloc.x+=1;
                    tail.headpos.x=1;
                    tail.currentloc.y+=tail.headpos.y;
                    tail.headpos.y=0;
                    match tail.visited.binary_search(&tail.currentloc){
                        Ok(_pos) => {},
                        Err(pos) => tail.visited.insert(pos, tail.currentloc)
                    }
                }
            }
            if set.direction=='L'{
                tail.headpos.x-=1;
                if tail.headpos.x< -1{
                    tail.currentloc.x-=1;
                    tail.headpos.x=-1;
                    tail.currentloc.y+=tail.headpos.y;
                    tail.headpos.y=0;
                    match tail.visited.binary_search(&tail.currentloc){
                        Ok(_pos) => {},
                        Err(pos) => tail.visited.insert(pos, tail.currentloc)
                    }
                }
            }
        }
    }
    return tail.visited.len() as i32;
}

struct Longy{
    headpos: [Loc ;10],
    visited: Vec<Loc>,
}
fn part_2(moves: &Vec<Movey>) -> i32{
    let mut tail = Longy{
        headpos: [Loc {x: 0, y:0}; 10],
        visited: Vec::new(),
    };
    tail.visited.push(Loc {x: 0, y:0});
    for set in moves{
        println!("x: {}, y: {}", tail.headpos[8].x, tail.headpos[8].y);
        for x in 0..set.steps{
            if set.direction=='U'{
                tail.headpos[0].y+=1;
            }
            else if set.direction=='D'{
                tail.headpos[0].y-=1;
            }
            else if set.direction=='L'{
                tail.headpos[0].x-=1;
            }
            else if set.direction=='R'{
                tail.headpos[0].x+=1;
            }
            for y in 1..10{
                tail.headpos[y] = calc_movement(tail.headpos[y], &mut tail.headpos[y-1]);
            }
            match tail.visited.binary_search(&tail.headpos[9]){
                Ok(_pos) => {},
                Err(pos) => tail.visited.insert(pos, tail.headpos[9])
            }
            
        }
    }
                
    return tail.visited.len() as i32;
}

fn calc_movement(this: Loc, head: &mut Loc) -> Loc {
    let mut thisy = this;
    if head.y>1{
        head.y=1;
        thisy.y+=1;
        if head.x>0{
            head.x-=1;
            thisy.x+=1;
        }
        if head.x<0{
            head.x+=1;
            thisy.x-=1;
        }
    }
    else if head.y< -1{
        head.y=-1;
        thisy.y-=1;
        if head.x>0{
            head.x-=1;
            thisy.x+=1;
        }
        if head.x<0{
            head.x+=1;
            thisy.x-=1;
        }
    }
    else if head.x< -1{
        head.x=-1;
        thisy.x-=1;
        if head.y>0{
            head.y-=1;
            thisy.y+=1;
        }
        if head.y<0{
            head.y+=1;
            thisy.y-=1;
        }
    }
    else if head.x> 1{
        head.x=1;
        thisy.x+=1;
        if head.y>0{
            head.y-=1;
            thisy.y+=1;
        }
        if head.y<0{
            head.y+=1;
            thisy.y-=1;
        }
    }

    thisy
}