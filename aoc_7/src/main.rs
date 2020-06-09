extern crate regex;

use std::iter::repeat;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct InputRecord {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl InputRecord {
    fn new(name: String, weight: u32, children: Vec<String>) -> InputRecord {
        InputRecord {
            name,
            weight,
            children: children,
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<&Node>,
    parent: &Node,
}

impl Node {
    fn new(name: String, weight: u32) -> Node {
        Node {
            name,
            weight,
            children: vec![],
        }
    }
}


fn print_node(n : &Node, ind: usize) {
    let pad :String = repeat(' ').take(ind).collect();
    println!("{}{} w {}", pad,  n.name, n.weight);
    for c in n.children.iter() {
        print_node(c, ind + 2);
    }

    //println!("{}End of {}", pad, n.name);
}

fn main() {
    let mut nodes_map = HashMap::new();
    let mut input_records: Vec<InputRecord> = vec![];

    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let re = Regex::new(r"^\s*(\w+)\s*\((\d+)\)\s*(->)?\s*(.*)").unwrap();
    for line in reader.lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            println!("p1: {} p2: {} p3: {}", &cap[1], &cap[2], &cap[4]);
            input_records.push(
                InputRecord::new(
                    cap[1].to_string(),
                    //1,
                    cap[2].parse::<u32>().unwrap(),
                    cap[4].split(", ").map(|x| x.to_string()).collect()));

            nodes_map.insert(
                cap[1].to_string(),
                Node::new(
                    cap[1].to_string(),
                    cap[2].parse::<u32>().unwrap()));
        }
    }

    println!("Starting to build graph");
    println!("{:?}", nodes_map);
    for record in input_records {
        for name in record.children.iter().filter(|x| x.len() > 0) {
            println!("Looking for {} for {}", name, record.name);
            let to = nodes_map.get(name).unwrap();
            nodes_map.entry(record.name.clone()).and_modify(|e| { (*e).children.push(to); });
        }
    }
    println!("{:?}", nodes_map);
    println!("");

    for (_k,n) in nodes_map {
        print_node(&n, 0);
    }
}
