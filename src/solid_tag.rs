use deathframe::components::solid::SolidTag as STag;

#[derive(Clone)]
pub enum SolidTag {
    None,
    PlayerNoCollision,
    PlayerWithCollision,
    Enemy,
}

impl Default for SolidTag {
    fn default() -> Self {
        SolidTag::None
    }
}

impl STag for SolidTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (SolidTag::PlayerNoCollision, _)
            | (_, SolidTag::PlayerNoCollision) => false,
            (SolidTag::PlayerWithCollision, SolidTag::Enemy)
            | (SolidTag::Enemy, SolidTag::PlayerWithCollision) => true,
            _ => false,
        }
    }
}
