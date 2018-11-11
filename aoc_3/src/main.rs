use std::cmp;
use std::iter;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq,Eq,Hash)]
#[derive(Clone)]
struct Point {
    horizontal: i32,
    vertical: i32,
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Point {
    fn new(h: i32, v: i32) -> Point {
        Point{horizontal: h, vertical:v}
    }
}

fn part1(dest: i32) -> Point {
    if dest == 1 {
        return Point::new(0,0);
    }
    let mut cur = 2;
    let mut cur_point = Point::new(1, 0);
    let mut edge = 3;

    let mut layer_end = cur + 4 * (edge - 1) - 1;

    //println!("edge {} layer_end {}", edge, layer_end);
    // Find layer
    while dest > layer_end {
        cur_point.horizontal += 1;
        cur_point.vertical += 1;
        cur = layer_end + 1;
        edge += 2;
        layer_end = cur + 4 * (edge - 1) - 1;
        //println!("edge {} layer_end {}", edge, layer_end);
    }

    // Find in layer
    let moves = cmp::min(dest - cur, edge -2);
    cur += moves;
    cur_point.vertical -= moves;

    let moves = cmp::min(dest - cur, edge -1);
    cur += moves;
    cur_point.horizontal -= moves;

    let moves = cmp::min(dest - cur, edge -1);
    cur += moves;
    cur_point.vertical += moves;

    let moves = cmp::min(dest - cur, edge -1);
    cur_point.horizontal += moves;

    cur_point
}

fn calc_val(grid: &mut HashMap<Point, i32>, pos: &Point) -> i32 {
    (-1..2).flat_map(|x| iter::repeat(x).take(3))
        .zip((-1..2).cycle())
        .filter(|(x,y)| *x != 0 || *y != 0)
        .map(|(x,y)| *grid.entry(Point::new(pos.horizontal + x, pos.vertical + y)).or_insert(0))
        .sum()
}

fn step(pos: &mut Point, edge: &mut i32, dir:&mut Direction) {
    let new_dir;
    match dir {
        Direction::Up(x) => { if *x > 0 { pos.vertical += 1; new_dir = Direction::Up(*x-1); } else { pos.horizontal -= 1; new_dir = Direction::Left(*edge-2);} },
        Direction::Left(x) => { if *x > 0 { pos.horizontal -= 1; new_dir = Direction::Left(*x-1); } else { pos.vertical -= 1; new_dir = Direction::Down(*edge-2);} },
        Direction::Down(x) => { if *x > 0 { pos.vertical -= 1; new_dir = Direction::Down(*x-1); } else { pos.horizontal += 1; new_dir = Direction::Right(*edge - 1);} },
        Direction::Right(x) => { if *x > 0 { pos.horizontal += 1; new_dir = Direction::Right(*x-1); } else { pos.vertical += 1; *edge += 2; new_dir = Direction::Up(*edge-3);} },
    }

    *dir = new_dir;
}

fn part2(dest: i32) -> i32 {
    let mut grid = HashMap::new();
    let mut cur_point = Point::new(1,0);
    let mut cur_val;
    let mut edge = 3;
    let mut dir = Direction::Up(1);

    grid.insert(Point::new(0,0), 1);

    loop {
        cur_val = calc_val(&mut grid, &cur_point);
        grid.insert(cur_point.clone(), cur_val);
        println!("DBG: {:?} : {}, {:?}", cur_point, cur_val, dir);

        if cur_val > dest { return cur_val; }

        step(&mut cur_point, &mut edge, &mut dir);
    }
}

fn main() {
    let input = 361527;
    let p1_solution = part1(input);
    println!("Part 1 solution point: {:?} total: {} ", p1_solution, p1_solution.horizontal.abs() + p1_solution.vertical.abs());

    println!("Part 2 solution {} ", part2(input));
}
