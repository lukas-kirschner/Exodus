use crate::campaign::graph::GraphParseError::{
    DuplicateNodeId, IOError, InvalidInteger, SyntaxError,
};
use crate::exodus_serializable::ExodusSerializable;
use std::collections::HashMap;
use std::convert::Infallible;
use std::io::{self, prelude::*, BufReader, Error};
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
            let parts: Vec<&str> = line.split_whitespace().collect();
            match state {
                ReadState::Nodes => match parts[..] {
                    [id, x, y] => self.nodes.insert(
                        str::parse::<NodeID>(id)?,
                        Node {
                            id: str::parse::<NodeID>(id)?,
                            kind: NodeKind::Empty,
                            coord: (str::parse::<Coord>(x)?, str::parse::<Coord>(y)?),
                        },
                    ),
                    [id, map_file, x, y] => self.nodes.insert(
                        str::parse::<NodeID>(id)?,
                        Node {
                            id: str::parse::<NodeID>(id)?,
                            kind: NodeKind::MapFilename {
                                map: map_file.to_string(),
                            },
                            coord: (str::parse::<Coord>(x)?, str::parse::<Coord>(y)?),
                        },
                    ),
                    ["#"] => {
                        state = ReadState::Edges;
                        continue;
                    },
                    [] => continue,
                    _ => Err(SyntaxError { line: lineno })?,
                }
                .map_or(Ok(()), |v| {
                    Err(DuplicateNodeId {
                        line: lineno,
                        id: v.id,
                    })
                })?,
                ReadState::Edges => match parts[..] {
                    [id_a, id_b] => {
                        self.edges
                            .entry(str::parse::<NodeID>(id_a)?)
                            .or_insert(vec![])
                            .push(str::parse::<NodeID>(id_b)?);
                        None
                    },
                    [id_a, id_b, edge_label] => {
                        self.edges
                            .entry(str::parse::<NodeID>(id_a)?)
                            .or_insert(vec![])
                            .push(str::parse::<NodeID>(id_b)?);
                        self.edge_labels.insert(
                            (str::parse::<NodeID>(id_a)?, str::parse::<NodeID>(id_b)?),
                            edge_label.to_string(),
                        )
                    },
                    [] => continue,
                    _ => return Err(SyntaxError { line: lineno }),
                }
                .map_or(Ok(()), |v| {
                    Err(GraphParseError::DuplicateEdgeLabel { label: v })
                })?,
            }
        }
        Ok(())
    }
}
