use deathframe::geo::Vector;

#[derive(Clone, Deserialize)]
pub struct CameraSettings {
    pub size: Vector,
}
