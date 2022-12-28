use std::fs;


enum Packet{
    numby {num: i32},
    listy {list: Vec<Packet>}
}
fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let mut compars: Vec<Vec<Packet>> = Vec::new();
    for compar in input.split("\n\n"){
        let mut pair: Vec<Packet> = Vec::new();
        for packet in compar.split("\n"){
            println!("{packet}");
            let mut new_packet = Packet::listy{list: Vec::new()};
            create_listy(&mut new_packet, packet.chars().collect())
        }
    }
}

fn create_listy(packet: &mut Packet, input: Vec<char>) -> (){
    if input[0] != '['{
        panic!();
    }
    let mut county = 1;
    let mut skippy = 0;
    for x in &input[1..input.len()]{
        if skippy>0{
            skippy-=1;
            county+=1;
            continue;
        }
        println!("{x}");
        if x==&','{
            county+=1;
            continue;
        }
        
        if x==&'['{
            let mut new_packet = Packet::listy { list: Vec::new() };
            create_listy(&mut new_packet, input[county..find_close(&input, county)].to_vec());
            skippy = find_close(&input, county)-county+1;
            if let Packet::listy{list} = packet {
                list.push(new_packet);
            }
            

        }
        else {
            if let Packet::listy{list} = packet {
                list.push(Packet::numby { num: x.to_digit(10).unwrap() as i32 });
            }
        }
        county+=1;
    }
}

fn find_close(input: &Vec<char>, start: usize) -> usize{
    if input[start] != '['{
        panic!();
    }
    let mut counter = 1;
    for x in start+1..input.len(){
        if input[x]=='['{
            counter+=1;
        }
        else if input[x]==']'{
            counter-=1;
        }
        if counter == 0{
            return x
        }
    } 
    panic!();
}
