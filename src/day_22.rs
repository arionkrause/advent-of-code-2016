pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input));
    println!("Part 2: {}.", part_2::solve(&input));
    println!();
}

mod part_1 {
    use regex::Regex;

    #[derive(Debug)]
    struct Node {
        available: usize,
        used: usize,
    }

    fn decode_input(input: &str) -> Vec<Node> {
        let re = Regex::new(r"^/dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+) +\d+T +(?P<used>\d+)T +(?P<available>\d+)T +\d+%$").unwrap();

        input.lines().skip(2).map(|line| {
            let captures = re.captures(&line).unwrap();

            Node {
                available: captures.name("available").unwrap().as_str().parse().unwrap(),
                used: captures.name("used").unwrap().as_str().parse().unwrap(),
            }
        }).collect()
    }

    pub fn solve(input: &str) -> usize {
        let nodes = decode_input(&input);
        let mut viable_pairs = 0;

        for (index, node) in nodes.iter().enumerate() {
            for node_other in nodes[index + 1..].iter() {
                if node.used > 0 && node.used <= node_other.available
                        || node_other.used > 0 && node_other.used <= node.available {
                    viable_pairs += 1;
                }
            }
        }

        viable_pairs
    }
}

mod part_2 {
    use regex::Regex;
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashMap, HashSet};

    #[derive(Debug)]
    struct Node {
        size: usize,
        available: usize,
        used: usize,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    struct State {
        position: Position,
        manhattan_distance: usize,
        amount_steps: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.manhattan_distance.cmp(&self.manhattan_distance)
                    .then(other.position.y.cmp(&self.position.y))
                    .then(other.position.x.cmp(&self.position.x))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &State) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Position {
        x: usize,
        y: usize,
    }

    fn decode_input(input: &str) -> Vec<Vec<Node>> {
        let re = Regex::new(r"^/dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+) +(?P<size>\d+)T +(?P<used>\d+)T +(?P<available>\d+)T +\d+%$").unwrap();
        let mut buffer_nodes = HashMap::new();

        for line in input.lines().skip(2) {
            let captures = re.captures(&line).unwrap();
            let x: usize = captures.name("x").unwrap().as_str().parse().unwrap();
            let y: usize = captures.name("y").unwrap().as_str().parse().unwrap();

            let node = Node {
                size: captures.name("size").unwrap().as_str().parse().unwrap(),
                available: captures.name("available").unwrap().as_str().parse().unwrap(),
                used: captures.name("used").unwrap().as_str().parse().unwrap(),
            };

            buffer_nodes.insert((x, y), node);
        }

        let rows = *buffer_nodes.keys().map(|(_, y)| y).max().unwrap() + 1;
        let columns = *buffer_nodes.keys().map(|(x, _)| x).max().unwrap() + 1;
        let mut grid = Vec::new();

        for y in 0..rows {
            let mut row = Vec::new();

            for x in 0..columns {
                row.push(buffer_nodes.remove(&(x, y)).unwrap());
            }

            grid.push(row);
        }

        grid
    }

    pub fn solve(input: &str) -> usize {
        let grid = decode_input(&input);
        let origin = get_position_empty_node(&grid);
        let target = Position { x: grid[0].len() - 2, y: 0 };
        let amount_steps_to_target_node = get_amount_steps_to_target_node(&grid, &origin, &target);
        amount_steps_to_target_node + target.x * 5 + 1
    }

    fn get_position_empty_node(grid: &Vec<Vec<Node>>) -> Position {
        let mut position_empty_node = None;

        for (index_row, row) in grid.iter().enumerate() {
            for (index_column, node) in row.iter().enumerate() {
                if node.used == 0 {
                    position_empty_node = Some((index_column, index_row));
                }
            }
        }

        let position_empty_node = position_empty_node.unwrap();
        Position { x: position_empty_node.0, y: position_empty_node.1 }
    }

    fn get_amount_steps_to_target_node(grid: &Vec<Vec<Node>>, origin: &Position, target: &Position) -> usize {
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        queue.push(State {
            position: origin.clone().into(),
            manhattan_distance: get_manhattan_distance(&origin, &target),
            amount_steps: 0,
        });

        while let Some(state) = queue.pop() {
            if &state.position == target {
                return state.amount_steps;
            }

            visited.insert(state.position.clone());
            try_add_new_state(&grid, &mut queue, &mut visited, &state, &target, 0, 1);
            try_add_new_state(&grid, &mut queue, &mut visited, &state, &target, 0, -1);
            try_add_new_state(&grid, &mut queue, &mut visited, &state, &target, 1, 0);
            try_add_new_state(&grid, &mut queue, &mut visited, &state, &target, -1, 0);
        }

        panic!()
    }

    fn get_manhattan_distance(from: &Position, to: &Position) -> usize {
        ((from.x as isize - to.x as isize).abs()
                + (from.y as isize - to.y as isize).abs()) as usize
    }

    fn try_add_new_state(grid: &Vec<Vec<Node>>,
                         queue: &mut BinaryHeap<State>,
                         visited: &mut HashSet<Position>,
                         state: &State,
                         target: &Position,
                         offset_x: isize,
                         offset_y: isize) {
        let new_x = state.position.x as isize + offset_x;

        if new_x < 0 || new_x as usize >= grid[0].len() {
            return;
        }

        let new_y = state.position.y as isize + offset_y;

        if new_y < 0 || new_y as usize >= grid.len() {
            return;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if grid[new_y][new_x].size >= 500 {
            return;
        }

        let new_position = Position {
            x: new_x,
            y: new_y,
        };

        if visited.contains(&new_position) {
            return;
        }

        if queue.iter().any(|state| &state.position == &new_position) {
            return;
        }

        let manhattan_distance = get_manhattan_distance(&new_position, &target);

        let new_state = State {
            position: new_position.clone(),
            manhattan_distance,
            amount_steps: state.amount_steps + 1,
        };

        queue.push(new_state);
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        let input = "root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%";

        assert_eq!(solve(&input), 7);
    }
}
