use advent_of_code::helpers::math::least_common_multiple;
use hashbrown::HashMap;

advent_of_code::solution!(20);

#[derive(Clone, Copy, Debug, PartialEq)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug)]
enum NodeState<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, PulseType>),
}

#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    destinations: Vec<&'a str>,
}

type Nodes<'a> = HashMap<&'a str, Node<'a>>;

#[derive(Debug)]
struct QueueItem<'a>(&'a str, &'a str, PulseType);

#[derive(Debug, Default)]
struct States<'a> {
    cycle_count: usize,
    cycles: HashMap<&'a str, usize>,
    data: HashMap<&'a str, NodeState<'a>>,
    queue: Vec<QueueItem<'a>>,
}

impl<'a> States<'a> {
    fn drain_queue(&mut self, nodes: &'a Nodes) -> (u64, u64) {
        let mut pulse_count_low = 0;
        let mut pulse_count_high = 0;

        let pulses: Vec<QueueItem> = self.queue.drain(0..).collect();
        for QueueItem(target, destination, pulse_type) in pulses {
            let (low, high) = self.send_pulse(nodes, target, destination, pulse_type);
            pulse_count_low += low;
            pulse_count_high += high;
        }

        (pulse_count_low, pulse_count_high)
    }

    fn send_pulse(
        &mut self,
        nodes: &'a Nodes,
        sender: &'a str,
        target: &'a str,
        pulse_type: PulseType,
    ) -> (u64, u64) {
        // try turning it on and off again.
        if sender == "button" {
            self.cycle_count += 1;
        }

        if let Some(node) = nodes.get(target) {
            let node_state = self.data.get_mut(target).unwrap();

            match node_state {
                NodeState::Broadcast => {
                    for destination in &node.destinations {
                        self.queue.push(QueueItem(target, destination, pulse_type));
                    }
                }
                NodeState::FlipFlop(current) => {
                    if pulse_type == PulseType::Low {
                        *current = !*current;

                        let pulse = if *current {
                            PulseType::High
                        } else {
                            PulseType::Low
                        };

                        for destination in &node.destinations {
                            self.queue.push(QueueItem(target, destination, pulse));
                        }
                    }
                }
                NodeState::Conjunction(current) => {
                    let current_state = current.get_mut(sender).unwrap();

                    *current_state = pulse_type;

                    let pulse = if current.values().all(|v| *v == PulseType::High) {
                        PulseType::Low
                    } else {
                        self.cycles.insert(target, self.cycle_count);
                        PulseType::High
                    };

                    for destination in &node.destinations {
                        self.queue.push(QueueItem(target, destination, pulse));
                    }
                }
            };
        }

        if pulse_type == PulseType::Low {
            (1, 0)
        } else {
            (0, 1)
        }
    }
}

fn parse(input: &str) -> (Nodes, States) {
    let (nodes, mut states) = input
        .lines()
        .filter_map(|l| {
            l.split_once(" -> ").map(|(node_s, destination_s)| {
                let destinations: Vec<&str> = destination_s.split(", ").collect();

                let id = if node_s == "broadcaster" {
                    node_s
                } else {
                    &node_s[1..]
                };

                let node_state = if node_s == "broadcaster" {
                    NodeState::Broadcast
                } else if node_s.starts_with('%') {
                    NodeState::FlipFlop(false)
                } else {
                    NodeState::Conjunction(HashMap::new())
                };

                (Node { id, destinations }, node_state)
            })
        })
        .fold(
            (Nodes::default(), States::default()),
            |mut acc, (node, node_state)| {
                acc.1.data.insert(node.id, node_state);
                acc.0.insert(node.id, node);
                acc
            },
        );

    states.data.iter_mut().for_each(|(key, val)| {
        if let NodeState::Conjunction(state) = val {
            nodes
                .values()
                .filter(|n| n.destinations.contains(key))
                .for_each(|n| {
                    state.insert(n.id, PulseType::Low);
                })
        }
    });

    (nodes, states)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (nodes, mut states) = parse(input);

    let mut pulse_count_low = 0;
    let mut pulse_count_high = 0;

    for _ in 0..1000 {
        states
            .queue
            .push(QueueItem("button", "broadcaster", PulseType::Low));

        while !states.queue.is_empty() {
            let (low, high) = states.drain_queue(&nodes);
            pulse_count_low += low;
            pulse_count_high += high;
        }
    }

    Some(pulse_count_low * pulse_count_high)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (nodes, mut states) = parse(input);

    let target = "rx";

    // INVARIANT: the parent of target is a conjunction, which in turn, has conjunctions as parents.
    // find the parent and then all grandparents.
    let parent = nodes.values().find(|n| n.destinations.contains(&target))?;

    let grandparents: Vec<&str> = nodes
        .values()
        .filter(|n| n.destinations.contains(&parent.id))
        .map(|n| n.id)
        .collect();

    for _ in 0..usize::MAX {
        states
            .queue
            .push(QueueItem("button", "broadcaster", PulseType::Low));

        while !states.queue.is_empty() {
            states.drain_queue(&nodes);
        }

        let grandparent_cycles: Vec<usize> = states
            .cycles
            .iter()
            .filter(|(key, _)| grandparents.contains(*key))
            .map(|(_, val)| *val)
            .collect();

        if grandparent_cycles.len() == grandparents.len() {
            return Some(least_common_multiple(&grandparent_cycles));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_two() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11687500));
    }
}
