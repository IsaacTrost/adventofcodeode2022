use std::fs;

fn main() {
    let boxes = fs::read_to_string("src/input").unwrap();
    let mut total = 0;
    for c in boxes.split("\n"){
        let x: Vec<char> = c.chars().collect();
        let len = x.len();
        let d = total;
        for y in &x[0..len/2]{
            if total>d{
                break;
            }
            for u in &x[len/2..len]{
                if y==u{
                    total+=calc_weight(*u);
                    break;
                }
            }
        }        
    }
    println!("{}", total);

    let boxes: Vec<&str> = boxes.split("\n").collect();
    total = 0;
    for x in 0..boxes.len()/3{
        let l = total;
        let line1: Vec<char> = boxes[x*3].chars().collect();
        let line2: Vec<char> = boxes[x*3+1].chars().collect();
        let line3: Vec<char> = boxes[x*3+2].chars().collect();
        for y in line1{
            if total>l{
                break;
            }
            for u in &line2{
                if total>l{
                    break;
                }
                if &y==u{
                    for i in &line3{
                        if &y==i{
                            total+=calc_weight(y);
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{}", total);
}

fn calc_weight(ch: char) -> i32{
    if ch.is_uppercase(){
        return (ch as i32) -64+26;
    }
    return (ch as i32)-96;
}