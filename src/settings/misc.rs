use deathframe::geo::Vector;

#[derive(Clone, Deserialize)]
pub struct MiscSettings {
    pub menu_selector_animation_speed:    Vector,
    pub menu_selector_animation_deadzone: Vector,
}
