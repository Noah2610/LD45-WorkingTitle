use super::component_prelude::*;

pub type CheckpointId = usize;

pub struct Checkpoint {
    pub applied:        bool,
    pub id:             CheckpointId,
    pub respawn_anchor: AmethystAnchor,
}

impl Checkpoint {
    pub fn new(id: CheckpointId, respawn_anchor: AmethystAnchor) -> Self {
        Self {
            id,
            respawn_anchor,
            applied: false,
        }
    }

    pub fn respawn_pos(
        &self,
        pos: &Vector,
        size: &Size,
        padding: &Vector,
    ) -> Vector {
        use AmethystAnchor as AA;

        let half_size = (size.w * 0.5, size.h * 0.5);

        match self.respawn_anchor {
            AA::Middle => (pos.0, pos.1),
            AA::MiddleLeft => (pos.0 - half_size.0 + padding.0, pos.1),
            AA::MiddleRight => (pos.0 + half_size.0 - padding.0, pos.1),
            AA::TopMiddle => (pos.0, pos.1 + half_size.1 - padding.1),
            AA::TopLeft => (
                pos.0 - half_size.0 + padding.0,
                pos.1 + half_size.1 - padding.1,
            ),
            AA::TopRight => (
                pos.0 + half_size.0 - padding.0,
                pos.1 + half_size.1 - padding.1,
            ),
            AA::BottomMiddle => (pos.0, pos.1 - half_size.1 + padding.1),
            AA::BottomLeft => (
                pos.0 - half_size.0 + padding.0,
                pos.1 - half_size.1 + padding.1,
            ),
            AA::BottomRight => (
                pos.0 + half_size.0 - padding.0,
                pos.1 - half_size.1 + padding.1,
            ),
        }
        .into()
    }
}

impl Component for Checkpoint {
    type Storage = VecStorage<Self>;
}
