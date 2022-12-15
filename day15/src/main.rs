use std::fs;

struct Sensor{
    location: Loc,
    nearest: Loc,
}

struct Loc{
    y: i32,
    x: i32,
}

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in input.split("\n"){
        println!("{}", line);
        let splitty: Vec<&str> = line.split(" ").collect();
        let sensorloc = Loc{
            x: splitty[2].replace(&['=','x', ',', 'y', ':'][..], "").parse().unwrap(),
            y: splitty[3].replace(&['=','x', ',', 'y', ':'][..], "").parse().unwrap(),

        };
        let beakonloc = Loc{
            x: splitty[8].replace(&['=','x', ',', 'y', ':'][..], "").parse().unwrap(),
            y: splitty[9].replace(&['=','x', ',', 'y', ':'][..], "").parse().unwrap(),

        };
        sensors.push(
            Sensor { location: sensorloc, nearest: beakonloc }
        )
    }
    println!("Part 1: {}", part_1(&sensors));
    println!("Part 2: {}", part_2(&sensors));
}

fn part_1(sensors: &Vec<Sensor>) -> i32{
    let mut forbid: Vec<(i32, i32)> = Vec::new();
    for sensor in sensors{
        let dist = (sensor.location.x-sensor.nearest.x).abs() + (sensor.location.y-sensor.nearest.y).abs();
        let from2mil = (sensor.location.y-2000000).abs();
        if dist>=from2mil{
            let leftover = dist-from2mil;
            forbid.push((sensor.location.x-leftover,sensor.location.x+leftover));
        } 
    }
    let mut countedrange: Vec<(i32, i32)> = Vec::new();
    let mut thingy: Vec<i32> = Vec::new();
    for x in &forbid{
        let mut newo = *x;
        thingy = Vec::new();
        let mut counter = 0;
        for y in &countedrange{
            //println!("{:?}", y);
            if y.0<=x.0 && y.1>= x.1{
                newo = (100,-100)
            }
            else if x.0<=y.0 && x.1>= y.1{
                println!("deletin shit");
                thingy.push(counter);
            }
            else if x.0<=y.0 && x.1>= y.0{
                newo.1= y.0-1;
            }
            else if y.0<=x.0 && y.1>= x.0{
                newo.0=y.1+1;
            }
            counter+=1;
        }
        if !thingy.is_empty(){
            
            for u in thingy.into_iter().rev(){
                countedrange.remove(u as usize);
            }
        }
        if newo == (100,-100){
            continue;
        }
        if newo.1>=newo.0{
        countedrange.push(newo);
        }
        println!("{:?},\n{:?}", x, countedrange);

        
    }
    let mut answer = 0;
    for x in countedrange{
        answer+=x.1-x.0+1;
    }
    return answer;
        
    }

    

fn part_2(sensors: &Vec<Sensor>) -> i64{
    for p in 0..=4000000{
        if p%500000==0{
            println!("{}",p);
        }
        let mut forbid: Vec<(i32, i32)> = Vec::new();
        for sensor in sensors{
            let dist = (sensor.location.x-sensor.nearest.x).abs() + (sensor.location.y-sensor.nearest.y).abs();
            let from2mil = (sensor.location.y-p).abs();
            if dist>=from2mil{
                let leftover = dist-from2mil;
                forbid.push((sensor.location.x-leftover,sensor.location.x+leftover));
            } 
        }
        let mut countedrange: Vec<(i32, i32)> = Vec::new();
        let mut thingy: Vec<i32> = Vec::new();
        for x in &forbid{
            let mut newo = *x;
            thingy = Vec::new();
            let mut counter = 0;
            for y in &countedrange{
                //println!("{:?}", y);
                if y.0<=x.0 && y.1>= x.1{
                    newo = (100,-100)
                }
                else if x.0<=y.0 && x.1>= y.1{
                    thingy.push(counter);
                }
                else if x.0<=y.0 && x.1>= y.0{
                    newo.1= y.0-1;
                }
                else if y.0<=x.0 && y.1>= x.0{
                    newo.0=y.1+1;
                }
                counter+=1;
            }
            if !thingy.is_empty(){
                
                for u in thingy.into_iter().rev(){
                    countedrange.remove(u as usize);
                }
            }
            if newo == (100,-100){
                continue;
            }
            if newo.1>=newo.0{
            countedrange.push(newo);
            }   
        }
        countedrange.sort();
        for h in 0..countedrange.len(){
            if h==countedrange.len()-1{
                break;
            }
            if countedrange[h].1+1!=countedrange[h+1].0{
                return (countedrange[h].1+1) as i64*4000000+p as i64;
            }
        }
    }
    0
}