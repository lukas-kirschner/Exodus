use crate::exodus_serializable::ExodusSerializable;
use std::collections::HashMap;
use std::io::{Read, Write};

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
    edges: HashMap<NodeID, NodeID>,
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
    NegativeIDGiven { id: i32, line: usize },
    MissingEdgeSpecificationSection,
}

impl ExodusSerializable for Graph {
    const CURRENT_VERSION: u8 = 0;
    type ParseError = GraphParseError;

    fn serialize<T: Write>(&self, file: &mut T) -> Result<(), Self::ParseError> {
        todo!()
    }

    fn parse<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        self.parse_current_version(file)
    }

    fn parse_current_version<T: Read>(&mut self, file: &mut T) -> Result<(), Self::ParseError> {
        todo!()
    }
}
