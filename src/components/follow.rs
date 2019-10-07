use std::cmp;

use super::component_prelude::*;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum FollowTag {
    Player,
    Camera,
}

#[derive(PartialEq)]
pub struct Follower {
    pub tag:       FollowTag,
    pub priority:  u32,
    pub round_pos: bool,
}

impl Follower {
    pub fn new(tag: FollowTag) -> Self {
        Self {
            tag,
            priority: 0,
            round_pos: false,
        }
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_round_pos(mut self) -> Self {
        self.round_pos = true;
        self
    }
}

impl cmp::PartialOrd for Follower {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl Component for Follower {
    type Storage = VecStorage<Self>;
}

pub struct Followed {
    pub tag: FollowTag,
}

impl Followed {
    pub fn new(tag: FollowTag) -> Self {
        Followed { tag }
    }
}

impl Component for Followed {
    type Storage = VecStorage<Self>;
}
