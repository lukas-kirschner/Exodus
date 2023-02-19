use crate::movement::Movement;
use std::collections::LinkedList;

#[derive(Clone)]
pub struct Player {
    movement_queue: LinkedList<Movement>,
    facing_left: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
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
    /// Remove all elements of the movement queue.
    pub fn clear_movement_queue(&mut self) {
        self.movement_queue.clear()
    }

    ///
    /// Get the atlas index of the player sprite facing right.
    /// TODO: Animation when player is walking?
    fn atlas_index_right() -> usize {
        255
    }
    ///
    /// Get the atlas index of the player sprite facing left.
    /// TODO: Animation when player is walking?
    fn atlas_index_left() -> usize {
        249
    }
    ///
    /// Get the atlas index facing in the correct direction
    pub fn atlas_index(&self) -> usize {
        match self.facing_left {
            true => Player::atlas_index_left(),
            false => Player::atlas_index_right(),
        }
    }

    ///
    /// Set if the player is facing right
    pub fn set_face_right(&mut self, right: bool) {
        self.facing_left = !right;
    }

    ///
    /// Check if the player is facing right
    pub fn is_facing_right(&self) -> bool {
        !self.facing_left
    }
    ///
    /// Check if the movement queue is empty
    pub fn movement_queue_is_empty(&self) -> bool {
        self.movement_queue.is_empty()
    }
}
