use super::component_prelude::*;

pub mod enemy_ai_data {
    use super::super::component_prelude::*;

    pub struct PacerData {
        pub origin:           Vector,
        pub pace_distance:    (Option<f32>, Option<f32>),
        pub pacing_direction: (PacingDirectionX, PacingDirectionY),
    }

    #[derive(Clone)]
    pub enum PacingDirectionX {
        Left,
        Right,
    }
    impl PacingDirectionX {
        pub fn invert(&mut self) {
            *self = match self {
                PacingDirectionX::Left => PacingDirectionX::Right,
                PacingDirectionX::Right => PacingDirectionX::Left,
            };
        }
    }
    impl Default for PacingDirectionX {
        fn default() -> Self {
            PacingDirectionX::Right
        }
    }

    #[derive(Clone)]
    pub enum PacingDirectionY {
        Up,
        Down,
    }
    impl PacingDirectionY {
        pub fn invert(&mut self) {
            *self = match self {
                PacingDirectionY::Up => PacingDirectionY::Down,
                PacingDirectionY::Down => PacingDirectionY::Up,
            };
        }
    }
    impl Default for PacingDirectionY {
        fn default() -> Self {
            PacingDirectionY::Up
        }
    }

    impl PacerData {
        pub fn new(
            origin: Vector,
            pace_distance: (Option<f32>, Option<f32>),
        ) -> Self {
            let pace_dist_signs = (
                pace_distance.0.map(|d| d.signum()),
                pace_distance.1.map(|d| d.signum()),
            );
            let pacing_direction = (
                if let Some(sign) = pace_dist_signs.0 {
                    if sign >= 0.0 {
                        PacingDirectionX::Right
                    } else {
                        PacingDirectionX::Left
                    }
                } else {
                    PacingDirectionX::default()
                },
                if let Some(sign) = pace_dist_signs.1 {
                    if sign >= 0.0 {
                        PacingDirectionY::Up
                    } else {
                        PacingDirectionY::Down
                    }
                } else {
                    PacingDirectionY::default()
                },
            );
            let pace_distance = (
                pace_distance.0.map(|d| d.abs()),
                pace_distance.1.map(|d| d.abs()),
            );
            Self {
                origin,
                pace_distance,
                pacing_direction,
            }
        }
    }
}

pub enum EnemyAi {
    Pacer(enemy_ai_data::PacerData),
}

impl Component for EnemyAi {
    type Storage = DenseVecStorage<Self>;
}
