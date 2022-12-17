#[macro_use] extern crate scan_fmt;

use std::{fs, env, collections::{BinaryHeap, HashSet}, cmp::{Ordering, Reverse}};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let input_contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

    // println!("DEBUG input: '{:#?}'", &input_contents);
    println!("DEBUG parse input: '{:#?}'",  parse(&input_contents));
    println!("solution1: '{}'", solution1(parse(&input_contents.clone())));
    // println!("solution2: '{}'", solution2(parse(&input_contents)));

    // let inp = "Valve EE has flow rate=3; tunnels lead to valves FF, DD\n";
    // let (a,b,c): (String, u32, String) = scan_fmt!(inp, r"Valve {} has flow rate={d}; tunnels lead to valves {/.*/}", String, u32, String).unwrap();
    // println!("DEBUG inp: '{:#?}' '{:#?}' '{:#?}'", a, b, c);

    // let Valves(valves) = parse(&input_contents);
    // let sp = shortest_path(Valves(valves.clone()), valves[0].clone(), valves[valves.len()-1-2].clone());
    // println!("shortest_path: '{}'", sp);
}

#[derive(Debug)]
struct Graph {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<String>,
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rate.cmp(&other.rate)
    }
}

impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// type Valves = Vec<Valve>;
#[derive(Debug)]
struct Valves(Vec<Valve>);

fn parse(input: &String) -> Valves {
    let valves: Vec<Valve> = input
        .lines()
        .map(
            |l| {
                let (v,r,_,_,_,e): (String, u32, String, String, String, String) = scan_fmt!(l, r"Valve {} has flow rate={d}; {} {} to {} {/.*/}", String, u32, String, String, String, String)
                .unwrap();
                let ve: Vec<String> = e
                    .split(", ")
                    .map(|el| el.to_string())
                    .collect()
                ;
                return Valve{name: v, rate: r, tunnels: ve};
            }
        )
        .collect()
        ;

    println!("DEBUG parse: '{:#?}'", valves);

    return Valves(valves);
}

fn solution1(Valves(valves): Valves) -> u32 {
    let mut closed_valves = valves.clone();

    let vsc = valves.clone();
    let mut current_valve = vsc
        .iter().filter(|v| v.name == "AA".to_string()).next().unwrap();

    let mut pressure: u32 = 0;
    let mut pressure_over_time: u32 = 0;

    let mut remaining_minutes = 30;
    while remaining_minutes > 0 {
        let TunnelScores(mut tunnel_scores) = tunnel_scores(Valves(valves.clone()), current_valve.clone());
        tunnel_scores.sort();
        tunnel_scores.reverse();

        // println!("DEBUG solution1 for: '{:#?}'", tunnel_scores);
        // break;

        pressure_over_time += pressure;

        for score in tunnel_scores {
            // can't open in remaining time
            if score.hops >= remaining_minutes {
                continue;
            }

            //open
            if current_valve.name == score.name {
                pressure += current_valve.rate;
                closed_valves = closed_valves.into_iter()
                    .filter(|v| v.name != current_valve.name)
                    .collect()
                ;

                break;
            }

            //move
            current_valve = valves
                .iter()
                .filter(|v| v.name == score.name)
                .next().unwrap()
            ;
            pressure_over_time += pressure * (score.hops-1);

            remaining_minutes -= score.hops-1;

            break;
        }

        remaining_minutes -= 1;
    }

    return pressure_over_time;
}

#[derive(Debug, PartialEq, Eq)]
struct Waypoint {
    steps: u32,
    valve: Valve,
}
impl Ord for Waypoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for Waypoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(vss: Valves, start: Valve, end: Valve) -> u32 {
    let valves = vss.0;

    let mut prio_queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    prio_queue.push(Reverse(
        Waypoint { steps: 0, valve: start.clone() }
    ));

    if start.name == end.name {
        return 0;
    }

    while let Some(Reverse(shortest_neighbor)) = prio_queue.pop() {
        // print!("DEBUG - '{:#?}'\n", smallest_steps_wayp);
        let candidates = shortest_neighbor.valve.tunnels;

        if candidates.contains(&end.name) {
            return shortest_neighbor.steps+1;
        }

        for candidate_pos in candidates {
            if visited.insert(candidate_pos.clone()) {
                let vn = valves
                    .iter()
                    .filter(|v| v.name == candidate_pos)
                    .next().unwrap();
                prio_queue.push(Reverse(Waypoint{
                    steps: shortest_neighbor.steps+1,
                    valve: vn.clone(),
                }));
            }
        }
    }

    return 0;
}

#[derive(Debug, PartialEq, Eq)]
struct TunnelScore {
    name: String,
    score: u32,
    hops: u32,
}

impl Ord for TunnelScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for TunnelScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct TunnelScores(Vec<TunnelScore>);

fn tunnel_scores(Valves(valves): Valves, start: Valve) -> TunnelScores {
    let mut scores: Vec<TunnelScore> = Vec::new();

    if valves.is_empty() {
        return TunnelScores(scores);
    }

    for valve_end in valves.clone() {
        let hops = shortest_path(Valves(valves.clone()), start.clone(), valve_end.clone());
        let mut score = ((valve_end.clone().rate  as f32 / hops as f32) * 100000 as f32) as u32;

        if hops == 0 {
            score = start.rate * 100000;
        }

        scores.push(TunnelScore {
            name: valve_end.clone().name,
            score: score,
            hops: hops,
        })
    }

    return TunnelScores(scores);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse() {
    //     let data = _input_test_data();

    //     assert_eq!(
    //         vec![],
    //         parse(&data)
    //     );
    // }

    #[test]
    fn test_solution1() {
        let data = _input_test_data();

        assert_eq!(1651, solution1(parse(&data)));
    }

    // #[test]
    // fn test_solution2() {
    //     let data = _input_test_data();

    //     assert_eq!(29, solution2(parse(&data)));
    // }
}

fn _input_test_data() -> String {
return "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II".to_string();
}
