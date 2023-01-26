use core::fmt;
use std::collections::HashSet;
use std::error::Error;
use itertools::Itertools;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Debug)]
enum DataGram {
    Int(u32),
    List(Vec<DataGram>),
}

impl fmt::Display for DataGram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataGram::Int(i) => write!(f, "{}", i),
            DataGram::List(ds) => write!(f, "[{}]",  ds.iter().map(|d| format!("{}", d)).join(",")),
        }
    }
}

#[derive(Debug)]
struct PacketPair {
    index: usize,
    left: DataGram,
    right: DataGram,
}

impl PacketPair {
    fn new(index: usize, left: DataGram, right:DataGram) -> PacketPair {
        PacketPair { index, left, right, }
    }

    fn is_right_order(&self) -> bool {
        PacketPair::are_ordered(&self.left, &self.right).unwrap_or(false)
    }

    fn are_ordered(left: &DataGram, right: &DataGram) -> Option<bool> {
        use itertools::EitherOrBoth::*;
        match (left, right) {
            (DataGram::Int(l), DataGram::Int(r)) => if l < &r {
                Some(true)
            } else if l > &r {
                Some(false)
            } else {
                None
            },
            (DataGram::List(ls), DataGram::List(rs)) => {
                let mut comps = ls.iter().zip_longest(rs.iter()).map(|p| {
                    match p {
                        Both(l, r) => PacketPair::are_ordered(l, r),
                        Right(_) => Some(true),
                        Left(_) => Some(false),
                    }
                }).skip_while(|p| p.is_none());
                comps.next().flatten()
                
            }, 
            (DataGram::Int(i), DataGram::List(_)) => PacketPair::are_ordered(
                        &DataGram::List(vec![DataGram::Int(*i)]), right),
            (DataGram::List(_), DataGram::Int(i)) => PacketPair::are_ordered(left,
                &DataGram::List(vec![DataGram::Int(*i)])),

        }
    }
}

#[derive(Parser)]
#[grammar = "day13.pest"]
struct PacketParser;

impl DataGram {

    fn parse_packet(p: Pair<Rule>) -> Result<DataGram, String> {
        match p.as_rule() {
            Rule::packet => DataGram::parse_packet(p.into_inner().next().ok_or("empty packet")?),
            Rule::integer => Ok(DataGram::Int(p.as_str().parse::<u32>().map_err(|e| format!("{}", e))?)),
            Rule::list => {
                let items: Result<Vec<DataGram>, String> =    p.into_inner().map(|p| DataGram::parse_packet(p)).collect();
                items.map(|xs| DataGram::List(xs))
            },
            _ => Err(format!("unexpected input, expected: [packet | list | integer], got: {}", p.as_str()))
        }    
    }

    fn new(l: &str) -> Result<DataGram, String> {
        let mut p = PacketParser::parse(Rule::packet, l).map_err(|e| format!("{}", e))?;
        DataGram::parse_packet(p.next().ok_or("empty top level packet")?)
    
    }
}

pub(crate) fn result(ls: Vec<String>) -> Result<(), Box<dyn Error>> {
    let  lines: Result<Vec<PacketPair>, String> = ls.into_iter()
            .chunks(3)
            .into_iter()
            .enumerate()
            .map(|(i, ch)| {
                let pair:Result<Vec<DataGram>, String> =    
                        ch.take(2)
                        .map(|s| DataGram::new(&s))
                        .collect();
                pair.map(|p| {
                    let mut p = p.into_iter(); 
                    PacketPair::new(i+1, 
                        p.next().unwrap(), // todo maybe find a better way 
                        p.next().unwrap())
                })
            }).collect();

    let pairs = lines?;
    // println!("inputs: {:?}", pairs);
    
    let right_ordered = pairs.iter().filter(|&p| p.is_right_order());
    // println!("right_ordered: {:?}", right_ordered);
    let res: usize = right_ordered.map(|p| p.index).sum();
    
    println!("Part1 - result: {}", res);


    // for part 2, sort pairs with is_right_order turned into comparator
    // but first add 2 more items divider_packets
    // [[2]]
    // [[6]]
    let divider_packets = vec![
        DataGram::List(vec![DataGram::List(vec![DataGram::Int(2)])]),
        DataGram::List(vec![DataGram::List(vec![DataGram::Int(6)])]),
        ];
    //  first would need to flatten list of PacketPairs into list of DataGrams?
    let flattened_packets = pairs
        .into_iter()
        .flat_map(|p| vec![p.left, p.right])
        .chain(divider_packets)
        .sorted_by(|a,b| {
            match PacketPair::are_ordered(a, b) {
                Some(true) => std::cmp::Ordering::Less,
                Some(false) => std::cmp::Ordering::Greater,
                None => std::cmp::Ordering::Equal,
            }
        });
    // flattened_packets.for_each(|p| { 
    //     println!("{}", p);
    // });    
    let divider_packets = vec![
        DataGram::List(vec![DataGram::List(vec![DataGram::Int(2)])]),
        DataGram::List(vec![DataGram::List(vec![DataGram::Int(6)])]),
        ];
    let divider_set = HashSet::<_>::from_iter(divider_packets.iter().map(|p|  format!("{}", p)));
    let divider_indices:usize =    flattened_packets
        .enumerate()
        .filter(|(_, p)| divider_set.contains(&format!("{}", p)))
        .map(|(i, p)| i+1)
        .product();

    println!("Part2 - {}", divider_indices);
    Ok(())
}