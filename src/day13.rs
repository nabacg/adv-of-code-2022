use std::error::Error;
use itertools::Itertools;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Debug)]
enum DataGram {
    Int(u32),
    List(Vec<DataGram>),
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
    println!("inputs: {:?}", pairs);
    
    let right_ordered = pairs.into_iter().filter(|p| p.is_right_order());
    println!("right_ordered: {:?}", right_ordered);
    let res: usize = right_ordered.map(|p| p.index).sum();
    
    println!("result: {}", res);

    Ok(())
}