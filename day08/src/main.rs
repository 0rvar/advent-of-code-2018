use std::collections::HashMap;

fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    {
        let mut stack: Vec<Frame> = vec![];
        stack.push(Frame {
            id: "".to_string(),
            children_left: 1,
            num_children: 1,
            num_metadata: 0,
        });
        let mut index = 0;
        loop {
            let frame = stack.last_mut().expect("wat");
            if frame.children_left > 0 {
                let num_children = input[index];
                let num_metadata = input[index + 1];
                index += 2;

                let node_index = frame.num_children - frame.children_left;
                let node_id = format!("{}->{}", frame.id, node_index);

                frame.children_left -= 1;
                stack.push(Frame {
                    id: node_id,
                    children_left: num_children,
                    num_children,
                    num_metadata,
                });
            } else {
                let mut metadata = vec![];
                for _ in 0..frame.num_metadata {
                    metadata.push(input[index]);
                    index += 1;
                }
                nodes.insert(
                    frame.id.clone(),
                    Node {
                        id: frame.id.clone(),
                        num_children: frame.num_children,
                        metadata,
                    },
                );
                stack.pop().unwrap();
                if stack.len() < 1 {
                    break;
                }
            }
        }
    }
    println!(
        "Part 1: {}",
        nodes.values().flat_map(|x| &x.metadata).sum::<usize>()
    );

    nodes.get_mut("").unwrap().metadata = vec![1];

    let mut values: HashMap<&str, usize> = HashMap::new();
    loop {
        if values.contains_key("") {
            break;
        }
        'scan: for (id, node) in &nodes {
            let id: &str = &*id;
            if values.contains_key(id) {
                continue;
            }
            if node.num_children > 0 {
                let mut sum = 0;
                for index in &node.metadata {
                    if *index == 0 || *index > node.num_children {
                        // skip
                    } else {
                        let sub_id: &str = &format!("{}->{}", id, index - 1);
                        if let Some(value) = values.get(sub_id) {
                            sum += value;
                        } else {
                            // node has no score yet
                            continue 'scan;
                        }
                    }
                }
                values.insert(id, sum);
            } else {
                values.insert(id, node.metadata.iter().sum::<usize>());
            }
        }
    }
    println!("Part 2: {}", values.get("->0").unwrap());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    id: String,
    num_children: usize,
    metadata: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Frame {
    id: String,
    children_left: usize,
    num_children: usize,
    num_metadata: usize,
}
