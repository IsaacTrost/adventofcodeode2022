use std::fs;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let mut start = (20,0);
    let mut end = (20,37);
    let mut graph: Vec<Vec<i32>> = input.split("\n").map(|x| x.chars().map(|y|(y as i32) - 97).collect()).collect();
    for x in 0..graph.len(){
        for y in 0..graph[0].len(){
            if graph[x][y]==83-97{
                start = (x,y);
                graph[x][y]=0;
            }
            else if graph[x][y]==69-97{
                end = (x,y);
                graph[x][y]=25;
            }
        }
    }
    println!("puzzle 1: {}", part_1(&graph, start, end));
    println!("puzzle 2: {}", part_2(graph, end));


}

fn part_1(graph: &Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> i32{
    let mut new_graph: Vec<Vec<(usize, usize)>>= graph.into_iter().map(|line| line.into_iter().map(|it| (*it as usize, std::i32::MAX as usize)).collect()).collect(); 
    new_graph[start.0][start.1].1=0;
    scan(&mut new_graph, start);
    return new_graph[end.0][end.1].1 as i32
}

fn part_2(graph: Vec<Vec<i32>>, end: (usize, usize)) -> i32{
    let mut small = std::i32::MAX;
    let mut counter = 0;
    for row in 0..graph.len(){
        for col in 0..graph[0].len(){
            if graph[row][col]==0{
                let y = part_1(&graph.clone(), (row, col), end);
                println!("{counter}, {small} {row},{col}, {y}");
                if y<small{
                    small=y;
                }
                counter+=1;
            }
        }
    }
    small
}

fn scan(mut graph: &mut Vec<Vec<(usize, usize)>>, pos: (usize, usize)) -> &mut Vec<Vec<(usize, usize)>>{
    let dirs:[(i32, i32); 4] = [(0,-1), (0,1), (-1,0), (1,0)];
    for x in dirs{
        if pos.0 as i32 + x.0>=0 && pos.0 as i32 + x.0<graph.len() as i32 && pos.1 as i32 + x.1 >=0 && pos.1 as i32 + x.1 <graph[0].len() as i32{
            if graph[(pos.0 as i32 + x.0) as usize][(pos.1 as i32 + x.1) as usize].0 <= graph[pos.0][pos.1].0+1 && graph[(pos.0 as i32 + x.0) as usize][(pos.1 as i32 + x.1) as usize].1 > graph[pos.0][pos.1].1+1{
                graph[(pos.0 as i32 + x.0) as usize][(pos.1 as i32 + x.1) as usize].1 = graph[pos.0][pos.1].1+1;
                graph = scan(graph, ((pos.0 as i32 + x.0) as usize, (pos.1 as i32 + x.1) as usize));
            }
        }
    }
    graph
}