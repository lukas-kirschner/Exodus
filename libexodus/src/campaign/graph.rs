use crate::campaign::graph::GraphParseError::{
    DuplicateNodeId, IOError, InvalidInteger, SyntaxError,
};
use crate::exodus_serializable::ExodusSerializable;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{prelude::*, BufReader, Error};
use std::num::ParseIntError;

pub type NodeID = u16;
pub type Coord = i16;

#[derive(Debug, Eq, PartialEq)]
pub enum NodeKind {
    Empty,
    MapFilename { map: String },
}
#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub id: NodeID,
    pub kind: NodeKind,
    pub coord: (Coord, Coord),
}

#[derive(Debug)]
pub struct Graph {
    /// All nodes, mapped from their ID
    nodes: HashMap<NodeID, Node>,
    /// All edges
    edges: HashMap<NodeID, Vec<NodeID>>,
    /// All edge labels for labeled edges
    edge_labels: HashMap<(NodeID, NodeID), String>,
    /// The start node at (0,0)
    start_node: Option<Node>,
    /// The smallest X coordinate of all nodes of this graph
    min_x: Coord,
    /// The greatest X coordinate of all nodes of this graph
    max_x: Coord,
    /// The smallest Y coordinate of all nodes of this graph
    min_y: Coord,
    /// The greatest Y coordinate of all nodes of this graph
    max_y: Coord,
}
impl Default for Graph {
    fn default() -> Self {
        Graph {
            nodes: Default::default(),
            edges: Default::default(),
            edge_labels: Default::default(),
            start_node: None,
            min_x: Coord::max_value(),
            max_x: Coord::min_value(),
            min_y: Coord::max_value(),
            max_y: Coord::min_value(),
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
/// An error that might be thrown in a Game World Parser
pub enum GraphParseError {
    SyntaxError { line: usize },
    DuplicateNodeId { line: usize, id: NodeID },
    NegativeIDGiven { id: i32, line: usize },
    MissingEdgeSpecificationSection,
    DuplicateEdgeLabel { label: String },
    IOError { error: std::io::Error },
    InvalidInteger { error: ParseIntError },
}

impl From<std::io::Error> for GraphParseError {
    fn from(error: Error) -> Self {
        IOError { error }
    }
}
impl From<ParseIntError> for GraphParseError {
    fn from(error: ParseIntError) -> Self {
        InvalidInteger { error }
    }
}

impl Display for GraphParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphParseError::IOError { error } => std::fmt::Display::fmt(&error, f),
            GraphParseError::SyntaxError { line } => write!(f, "Syntax Error in line {}", line),
            DuplicateNodeId { line, id } => write!(f, "Duplicate Node {} in line {}", id, line),
            GraphParseError::NegativeIDGiven { id, line } => {
                write!(f, "Invalid negative ID {} in line {}", id, line)
            },
            GraphParseError::MissingEdgeSpecificationSection => {
                write!(f, "Missing Edge Specification Section")
            },
            GraphParseError::DuplicateEdgeLabel { label } => {
                write!(f, "Duplicate Edge Label: {}", label)
            },
            InvalidInteger { error } => write!(f, "Could not parse Integer: {}", error),
        }
    }
}

impl ExodusSerializable for Graph {
    const CURRENT_VERSION: u8 = 0;
    type ParseError = GraphParseError;

    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), Self::ParseError> {
        for node in self.nodes.values() {
            match &node.kind {
                NodeKind::Empty => write!(file, "{} {} {}\n", node.id, node.coord.0, node.coord.1)?,
                NodeKind::MapFilename { map } => write!(
                    file,
                    "{} {} {} {}\n",
                    node.id, map, node.coord.0, node.coord.1
                )?,
            };
        }
        write!(file, "#\n")?;
        for (edge_a, edges) in &self.edges {
            for edge_b in edges {
                match self.edge_labels.get(&(*edge_a, *edge_b)) {
                    None => write!(file, "{} {}\n", edge_a, edge_b)?,
                    Some(label) => write!(file, "{} {} {}\n", edge_a, edge_b, label)?,
                };
            }
        }
        Ok(())
    }

    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        self.parse_current_version(file)
    }

    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        *self = Self::default(); // Reset everything to default
        enum ReadState {
            /// Read all nodes and change to Edges as soon as a single bang (#) sign is encountered.
            Nodes,
            /// Read all edges
            Edges,
        }
        let reader = BufReader::new(file);
        let mut state = ReadState::Nodes;
        for (lineno, line) in reader.lines().enumerate() {
            let line = line?;
            match state {
                ReadState::Nodes => match parse_node_line(line.as_str()) {
                    NodeParseResult::UnnamedNode { id, x, y } => self.nodes.insert(
                        str::parse::<NodeID>(id)?,
                        Node {
                            id: str::parse::<NodeID>(id)?,
                            kind: NodeKind::Empty,
                            coord: (str::parse::<Coord>(x)?, str::parse::<Coord>(y)?),
                        },
                    ),
                    NodeParseResult::NamedNode { id, map_file, x, y } => self.nodes.insert(
                        str::parse::<NodeID>(id)?,
                        Node {
                            id: str::parse::<NodeID>(id)?,
                            kind: NodeKind::MapFilename {
                                map: map_file.to_string(),
                            },
                            coord: (str::parse::<Coord>(x)?, str::parse::<Coord>(y)?),
                        },
                    ),
                    NodeParseResult::Hash => {
                        state = ReadState::Edges;
                        continue;
                    },
                    NodeParseResult::Empty => continue,
                    NodeParseResult::Error => Err(SyntaxError { line: lineno })?,
                }
                .map_or(Ok(()), |v| {
                    Err(DuplicateNodeId {
                        line: lineno,
                        id: v.id,
                    })
                })?,
                ReadState::Edges => match parse_edge_line(line.as_str()) {
                    EdgeParseResult::UnnamedEdge { id_a, id_b } => {
                        self.edges
                            .entry(str::parse::<NodeID>(id_a)?)
                            .or_insert(vec![])
                            .push(str::parse::<NodeID>(id_b)?);
                        None
                    },
                    EdgeParseResult::NamedEdge {
                        id_a,
                        id_b,
                        edge_label,
                    } => {
                        self.edges
                            .entry(str::parse::<NodeID>(id_a)?)
                            .or_insert(vec![])
                            .push(str::parse::<NodeID>(id_b)?);
                        self.edge_labels.insert(
                            (str::parse::<NodeID>(id_a)?, str::parse::<NodeID>(id_b)?),
                            edge_label.to_string(),
                        )
                    },
                    EdgeParseResult::Empty => continue,
                    EdgeParseResult::Error => Err(SyntaxError { line: lineno })?,
                }
                .map_or(Ok(()), |v| {
                    Err(GraphParseError::DuplicateEdgeLabel { label: v.clone() })
                })?,
            }
        }
        Ok(())
    }
}
#[derive(Debug)]
enum NodeParseResult<'s> {
    Error,
    UnnamedNode {
        id: &'s str,
        x: &'s str,
        y: &'s str,
    },
    NamedNode {
        id: &'s str,
        map_file: &'s str,
        x: &'s str,
        y: &'s str,
    },
    Hash,
    Empty,
}
fn parse_node_line(line: &str) -> NodeParseResult {
    if line.trim().is_empty() {
        NodeParseResult::Empty
    } else if matches!(line.trim(), "#") {
        NodeParseResult::Hash
    } else {
        let re_filename = Regex::new(r"^\s*(\S+)\s+(\S.*\S)\s+(\S+)\s+(\S+)\s*$").unwrap();
        let re_no_filename = Regex::new(r"^\s*(\S+)\s+(\S+)\s+(\S+)\s*$").unwrap();
        if let Some(captures) = re_filename.captures(line) {
            NodeParseResult::NamedNode {
                id: captures.get(1).unwrap().as_str(),
                map_file: captures.get(2).unwrap().as_str(),
                x: captures.get(3).unwrap().as_str(),
                y: captures.get(4).unwrap().as_str(),
            }
        } else if let Some(captures) = re_no_filename.captures(line) {
            NodeParseResult::UnnamedNode {
                id: captures.get(1).unwrap().as_str(),
                x: captures.get(2).unwrap().as_str(),
                y: captures.get(3).unwrap().as_str(),
            }
        } else {
            NodeParseResult::Error
        }
    }
}
#[derive(Debug)]
enum EdgeParseResult<'s> {
    Error,
    UnnamedEdge {
        id_a: &'s str,
        id_b: &'s str,
    },
    NamedEdge {
        id_a: &'s str,
        id_b: &'s str,
        edge_label: &'s str,
    },
    Empty,
}

fn parse_edge_line(line: &str) -> EdgeParseResult {
    let re_named = Regex::new(r"^\s*(\S+)\s+(\S+)\s+(\S.*\S)\s*$").unwrap();
    let re_unnamed = Regex::new(r"^\s*(\S+)\s+(\S+)\s*$").unwrap();
    if line.trim().is_empty() {
        EdgeParseResult::Empty
    } else if let Some(captures) = re_named.captures(line) {
        EdgeParseResult::NamedEdge {
            id_a: captures.get(1).unwrap().as_str(),
            id_b: captures.get(2).unwrap().as_str(),
            edge_label: captures.get(3).unwrap().as_str(),
        }
    } else if let Some(captures) = re_unnamed.captures(line) {
        EdgeParseResult::UnnamedEdge {
            id_a: captures.get(1).unwrap().as_str(),
            id_b: captures.get(2).unwrap().as_str(),
        }
    } else {
        EdgeParseResult::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytebuffer::ByteBuffer;
    use strum::{EnumCount, IntoEnumIterator};

    #[test]
    fn test_simple_in_memory_deserialization() {
        let graph_file: String = r#"
        0 0 0
        1 0 1
        2 1 1
        3 1 0
        #
        0 1
        1 2
        2 3
        3 0
        "#
        .to_string();
        let mut graph = Graph::default();
        let result = graph.parse(&mut graph_file.as_bytes().clone());
        assert!(result.is_ok());
        // Check Nodes
        assert_eq!(graph.nodes.len(), 4);
        assert_node_in_graph(&graph, 0, NodeKind::Empty, (0, 0));
        assert_node_in_graph(&graph, 1, NodeKind::Empty, (0, 1));
        assert_node_in_graph(&graph, 2, NodeKind::Empty, (1, 1));
        assert_node_in_graph(&graph, 3, NodeKind::Empty, (1, 0));
        // Check Edges
        assert_edges_are_connected(&graph, &[0, 1, 2, 3, 0]);
    }
    #[test]
    fn test_simple_in_memory_deserialization_with_edge_names() {
        let graph_file: String = r#"
        0 0 0
        1 0 1
        2 1 1
        3 1 0
        #
        0 1 Campaign Mode
        1 2 lululu
        2 3 1 2 3 4 5 Text
        3 0 TestCase
        "#
        .to_string();
        let mut graph = Graph::default();
        let result = graph.parse(&mut graph_file.as_bytes().clone());
        assert!(result.is_ok());
        // Check Nodes
        assert_eq!(graph.nodes.len(), 4);
        assert_node_in_graph(&graph, 0, NodeKind::Empty, (0, 0));
        assert_node_in_graph(&graph, 1, NodeKind::Empty, (0, 1));
        assert_node_in_graph(&graph, 2, NodeKind::Empty, (1, 1));
        assert_node_in_graph(&graph, 3, NodeKind::Empty, (1, 0));
        // Check Edges
        assert_edges_are_connected(&graph, &[0, 1, 2, 3, 0]);
        assert_eq!("Campaign Mode", graph.edge_labels[&(0, 1)]);
        assert_eq!("lululu", graph.edge_labels[&(1, 2)]);
        assert_eq!("1 2 3 4 5 Text", graph.edge_labels[&(2, 3)]);
        assert_eq!("TestCase", graph.edge_labels[&(3, 0)]);
    }
    #[test]
    fn test_simple_in_memory_deserialization_with_edge_names_and_file_names() {
        let graph_file: String = r#"
        0 0 0
        1 testmap.exm 0 1
        2 hello world.exm 1 1
        3 Oh! Really?.exm 1 0
        #
        0 1 Campaign Mode
        1 2 lululu
        2 3 1 2 3 4 5 Text
        3 0 TestCase
        "#
        .to_string();
        let mut graph = Graph::default();
        let result = graph.parse(&mut graph_file.as_bytes().clone());
        assert!(result.is_ok());
        // Check Nodes
        assert_eq!(graph.nodes.len(), 4);
        assert_node_in_graph(&graph, 0, NodeKind::Empty, (0, 0));
        assert_node_in_graph(
            &graph,
            1,
            NodeKind::MapFilename {
                map: "testmap.exm".to_string(),
            },
            (0, 1),
        );
        assert_node_in_graph(
            &graph,
            2,
            NodeKind::MapFilename {
                map: "hello world.exm".to_string(),
            },
            (1, 1),
        );
        assert_node_in_graph(
            &graph,
            3,
            NodeKind::MapFilename {
                map: "Oh! Really?.exm".to_string(),
            },
            (1, 0),
        );
        // Check Edges
        assert_edges_are_connected(&graph, &[0, 1, 2, 3, 0]);
        assert_eq!("Campaign Mode", graph.edge_labels[&(0, 1)]);
        assert_eq!("lululu", graph.edge_labels[&(1, 2)]);
        assert_eq!("1 2 3 4 5 Text", graph.edge_labels[&(2, 3)]);
        assert_eq!("TestCase", graph.edge_labels[&(3, 0)]);
    }
    #[test]
    fn test_parse_node_line_1() {
        let line = "0 1 2".to_string();
        let result = parse_node_line(line.as_str());
        assert!(
            matches!(
                result,
                NodeParseResult::UnnamedNode {
                    id: "0",
                    x: "1",
                    y: "2"
                }
            ),
            "Got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_node_line_2() {
        let line = "  \t 0 12 2345   \t \t".to_string();
        let result = parse_node_line(line.as_str());
        assert!(
            matches!(
                result,
                NodeParseResult::UnnamedNode {
                    id: "0",
                    x: "12",
                    y: "2345"
                }
            ),
            "Got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_node_line_3() {
        let line = "  \t 0 map \t\t  12      2345   \t \t".to_string();
        let result = parse_node_line(line.as_str());
        assert!(
            matches!(
                result,
                NodeParseResult::NamedNode {
                    id: "0",
                    map_file: "map",
                    x: "12",
                    y: "2345"
                }
            ),
            "Got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_node_line_4() {
        let line = "  \t 0 map with whitespaces.exm \t\t  -12345      -2   \t \t".to_string();
        let result = parse_node_line(line.as_str());
        assert!(
            matches!(
                result,
                NodeParseResult::NamedNode {
                    id: "0",
                    map_file: "map with whitespaces.exm",
                    x: "-12345",
                    y: "-2"
                }
            ),
            "Got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_node_line_5() {
        let line = "  \t #   \t \t".to_string();
        let result = parse_node_line(line.as_str());
        assert!(matches!(result, NodeParseResult::Hash), "Got {:?}", result);
    }
    #[test]
    fn test_parse_node_line_6() {
        let line = "  \t    \t \t".to_string();
        let result = parse_node_line(line.as_str());
        assert!(matches!(result, NodeParseResult::Empty), "Got {:?}", result);
    }
    #[test]
    fn test_parse_node_line_err_1() {
        let line = "0 1".to_string();
        let result = parse_node_line(line.as_str());
        assert!(
            matches!(result, NodeParseResult::Error),
            "Expected Parse Error, got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_node_line_err_2() {
        let line = "# 0 ".to_string();
        let result = parse_node_line(line.as_str());
        assert!(
            matches!(result, NodeParseResult::Error),
            "Expected Parse Error, got {:?}",
            result
        );
    }

    #[test]
    fn test_parse_edge_line_1() {
        let line = "0 1".to_string();
        let result = parse_edge_line(line.as_str());
        assert!(
            matches!(
                result,
                EdgeParseResult::UnnamedEdge {
                    id_a: "0",
                    id_b: "1",
                }
            ),
            "Got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_edge_line_2() {
        let line = "0 1 lululu".to_string();
        let result = parse_edge_line(line.as_str());
        assert!(
            matches!(
                result,
                EdgeParseResult::NamedEdge {
                    id_a: "0",
                    id_b: "1",
                    edge_label: "lululu",
                }
            ),
            "Got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_edge_line_3() {
        let line = "0 1 lululu Name with Whitespaces".to_string();
        let result = parse_edge_line(line.as_str());
        assert!(
            matches!(
                result,
                EdgeParseResult::NamedEdge {
                    id_a: "0",
                    id_b: "1",
                    edge_label: "lululu Name with Whitespaces",
                }
            ),
            "Got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_edge_line_4() {
        let line = "\t 0123 12345  \t\t  lululu Name with Whitespaces \t\t".to_string();
        let result = parse_edge_line(line.as_str());
        assert!(
            matches!(
                result,
                EdgeParseResult::NamedEdge {
                    id_a: "0123",
                    id_b: "12345",
                    edge_label: "lululu Name with Whitespaces",
                }
            ),
            "Got {:?}",
            result
        );
    }
    #[test]
    fn test_parse_edge_line_5() {
        let line = "\t   \t\t  \t\t".to_string();
        let result = parse_edge_line(line.as_str());
        assert!(matches!(result, EdgeParseResult::Empty), "Got {:?}", result);
    }
    #[test]
    fn test_parse_edge_line_err_1() {
        let line = "#".to_string();
        let result = parse_edge_line(line.as_str());
        assert!(
            matches!(result, EdgeParseResult::Error),
            "Expected Parse Error, got {:?}",
            result
        );
    }
    #[test]
    fn test_simple_in_memory_deserialization_with_filenames() {
        let graph_file: String = r#"
        0 0 0
        1 map1.exm 0 1
        2 map with whitespaces.exm 1 1
        3 map 1 2 3 1 0
        #
        0 1
        1 2
        2 3
        3 0
        "#
        .to_string();
        let mut graph = Graph::default();
        match graph.parse(&mut graph_file.as_bytes().clone()) {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        };
        // Check Nodes
        assert_eq!(graph.nodes.len(), 4);
        assert_node_in_graph(&graph, 0, NodeKind::Empty, (0, 0));
        assert_node_in_graph(
            &graph,
            1,
            NodeKind::MapFilename {
                map: "map1.exm".to_string(),
            },
            (0, 1),
        );
        assert_node_in_graph(
            &graph,
            2,
            NodeKind::MapFilename {
                map: "map with whitespaces.exm".to_string(),
            },
            (1, 1),
        );
        assert_node_in_graph(
            &graph,
            3,
            NodeKind::MapFilename {
                map: "map 1 2 3".to_string(),
            },
            (1, 0),
        );
        // Check Edges
        assert_edges_are_connected(&graph, &[0, 1, 2, 3, 0]);
    }

    #[test]
    fn test_simple_in_memory_deserialization_with_empty_edges() {
        let graph_file: String = r#"
        0 0 0
        1 0 1
        2 1 1
        3 1 0
        #
        "#
        .to_string();
        let mut graph = Graph::default();
        let result = graph.parse(&mut graph_file.as_bytes().clone());
        assert!(result.is_ok());
        assert_eq!(graph.nodes.len(), 4);
        // Check Edges
        assert_edges_are_connected(&graph, &[]);
    }

    /// Assert that a node with the given properties exists in the graph.
    fn assert_node_in_graph(graph: &Graph, id: NodeID, kind: NodeKind, coord: (Coord, Coord)) {
        assert_eq!(
            graph.nodes.get(&id),
            Some(&Node { id, kind, coord }),
            "Expected Node with ID {} to be inside graph!",
            id
        )
    }

    /// Assert that the ordered list of node IDs is connected in the given graph.
    fn assert_edges_are_connected(graph: &Graph, edges: &[NodeID]) {
        if edges.len() <= 1 {
            assert_eq!(
                graph.edges.values().map(|con| con.len()).sum::<usize>(),
                0,
                "Expected Graph to have no edges at all!"
            );
        }
        let mut prev: Option<NodeID> = None;
        for &next_node in edges {
            prev = match prev {
                None => next_node,
                Some(prev_node) => {
                    assert!(
                        graph
                            .edges
                            .get(&prev_node)
                            .map(|con| con.contains(&next_node))
                            .unwrap_or(false),
                        "Expected Node {} to be connected to node {}!",
                        prev_node,
                        next_node
                    );
                    next_node
                },
            }
            .into();
        }
    }
}
