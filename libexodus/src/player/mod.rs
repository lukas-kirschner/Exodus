use std::collections::LinkedList;
use crate::movement::Movement;

pub struct Player {
    movement_queue: LinkedList<Movement>,
    facing_left: bool,
}

impl Player {
    ///
    /// Create a new player
    pub fn new() -> Self {
        Player {
            movement_queue: LinkedList::new(),
            facing_left: false,
        }
    }
    ///
    /// Get the next movement that is enqueued
    pub fn peek_movement_queue(&self) -> Option<&Movement> {
        self.movement_queue.front()
    }
    ///
    /// Push a movement to the movement queue
    pub fn push_movement_queue(&mut self, movement: Movement) -> &mut Self {
        self.movement_queue.push_back(movement);
        self
    }
    ///
    /// Remove the front of the movement queue and return the element
    pub fn pop_movement_queue(&mut self) -> Option<Movement> {
        self.movement_queue.pop_front()
    }
    ///
    /// Get the atlas index of the player sprite facing right.
    /// TODO: Animation when player is walking?
    pub fn atlas_index_right() -> usize {
        0
    }
    ///
    /// Get the atlas index of the player sprite facing left.
    /// TODO: Animation when player is walking?
    pub fn atlas_index_left() -> usize {
        4
    }
    ///
    /// Get the atlas index facing in the correct direction
    pub fn atlas_index(&self) -> usize {
        match self.facing_left {
            true => {
                Player::atlas_index_left()
            }
            false => {
                Player::atlas_index_right()
            }
        }
    }
}