use std::collections::HashMap;

struct Valve {
  flow: usize,
  next: Vec<usize>,
}

fn flowrate(valves: &Vec<Valve>, open: &Vec<usize>) -> usize {
  let mut out = 0;
  for val in open {
    out += valves[*val].flow;
  }
  out
}

fn dfs(valves: &Vec<Valve>, path: &mut Vec<usize>, open: &mut Vec<usize>, timerem: i32, released: usize) -> usize {
  if timerem < 0 {
    return released;
  }

  let curr = &valves[path[path.len()-1]];
  let mut flow = flowrate(valves, open);
  let mut max = 0;
  let mut rel = released;
  let mut timeleft = timerem;
  for v in curr.next.iter() {
    // TODO: Make sure not going in loops
    rel = released;

    
    // Try opening
    let val = &valves[*v];
    if !open.contains(&v) {
      open.push(*v);
      rel += flow;
      flow += val.flow;
      timeleft -= 1;
    }

    // Go there
    path.push(*v);
    rel += flow;
    timeleft -= 1;
    let res = dfs(valves, path, open, timeleft, rel);
    if res > max {
      max = res;
    }
    path.pop();

    // Go there without opening
    open.pop();
    flow -= val.flow;
    rel = released + flow;
    timeleft += 1;
    let newres = dfs(valves, path, open, timeleft, rel);
    if newres > max {
      max = res;
    }
  }

  max
}

pub fn day16() {
  // Parse
  let inp = include_str!("day16.txt").replace("tunnel leads to valve", "tunnels lead to valves");
  let mut names = HashMap::new();
  let mut nextraw = Vec::new();
  let mut valves = Vec::new();
  for line in inp.split("\n") {
    let parts = line.split(" has flow rate=").collect::<Vec<_>>();
    let right = parts[1].split("; tunnels lead to valves ").collect::<Vec<_>>();
    valves.push(Valve {
      flow: right[0].parse::<usize>().unwrap(),
      next: Vec::new(),
    });
    names.insert(parts[0].replace("Valve ", ""), nextraw.len());
    nextraw.push(right[1].split(", ").collect::<Vec<_>>());
  }

  // Update indices
  for (i, val) in nextraw.iter().enumerate() {
    for v in val {
      valves[i].next.push(*names.get(*v).unwrap());
    }
  }

  // DFS
  let mut path = vec![*names.get("AA").unwrap()];
  let mut open = Vec::new();
  let res = dfs(&valves, &mut path, &mut open, 30, 0);
  println!("Part 1: {}", res);
}