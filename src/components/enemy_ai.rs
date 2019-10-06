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
            Self {
                origin,
                pace_distance,
                pacing_direction: (
                    PacingDirectionX::default(),
                    PacingDirectionY::default(),
                ),
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
