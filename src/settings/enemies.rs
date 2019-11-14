use deathframe::geo::Vector;

#[derive(Clone, Deserialize)]
pub struct EnemiesSettings {
    pub ground: EnemySettings,
    pub flying: EnemySettings,
}

#[derive(Clone, Deserialize)]
pub struct EnemySettings {
    pub size:         Vector,
    pub gravity:      Option<Vector>,
    pub acceleration: Vector,
    pub max_velocity: (Option<f32>, Option<f32>),
}
