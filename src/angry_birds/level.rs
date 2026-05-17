use crate::angry_birds::bird::BirdType;
use crate::angry_birds::block::BlockType;
use crate::angry_birds::pig::PigSize;

pub struct LevelData {
    pub birds: Vec<BirdType>,
    pub blocks: Vec<BlockPlacement>,
    pub pigs: Vec<PigPlacement>,
}

pub struct BlockPlacement {
    pub block_type: BlockType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct PigPlacement {
    pub pig_size: PigSize,
    pub x: f32,
    pub y: f32,
}

pub struct LevelManager {
    pub levels: Vec<LevelData>,
    pub current_level: usize,
}

impl LevelManager {
    pub fn new() -> Self {
        let levels = vec![
            Self::create_level_1(),
            Self::create_level_2(),
            Self::create_level_3(),
            Self::create_level_4(),
            Self::create_level_5(),
        ];

        LevelManager {
            levels,
            current_level: 0,
        }
    }

    pub fn get_current_level(&self) -> &LevelData {
        &self.levels[self.current_level]
    }

    pub fn get_current_level_mut(&mut self) -> &mut LevelData {
        &mut self.levels[self.current_level]
    }

    pub fn next_level(&mut self) -> bool {
        if self.current_level < self.levels.len() - 1 {
            self.current_level += 1;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.current_level = 0;
    }

    pub fn get_level_number(&self) -> usize {
        self.current_level + 1
    }

    fn create_level_1() -> LevelData {
        LevelData {
            birds: vec![BirdType::Red, BirdType::Red, BirdType::Yellow],
            blocks: vec![
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 550.0,
                    y: 520.0,
                    width: 20.0,
                    height: 80.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 650.0,
                    y: 520.0,
                    width: 20.0,
                    height: 80.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 600.0,
                    y: 475.0,
                    width: 120.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 600.0,
                    y: 455.0,
                    width: 80.0,
                    height: 15.0,
                },
            ],
            pigs: vec![
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 600.0,
                    y: 430.0,
                },
            ],
        }
    }

    fn create_level_2() -> LevelData {
        LevelData {
            birds: vec![BirdType::Red, BirdType::Blue, BirdType::Yellow, BirdType::Yellow],
            blocks: vec![
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 500.0,
                    y: 530.0,
                    width: 20.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 560.0,
                    y: 530.0,
                    width: 20.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 620.0,
                    y: 530.0,
                    width: 20.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 560.0,
                    y: 490.0,
                    width: 140.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 700.0,
                    y: 530.0,
                    width: 20.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 700.0,
                    y: 490.0,
                    width: 60.0,
                    height: 15.0,
                },
            ],
            pigs: vec![
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 560.0,
                    y: 465.0,
                },
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 700.0,
                    y: 465.0,
                },
            ],
        }
    }

    fn create_level_3() -> LevelData {
        LevelData {
            birds: vec![BirdType::Red, BirdType::Yellow, BirdType::Blue, BirdType::White, BirdType::Yellow],
            blocks: vec![
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 480.0,
                    y: 540.0,
                    width: 15.0,
                    height: 50.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 540.0,
                    y: 540.0,
                    width: 15.0,
                    height: 50.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 510.0,
                    y: 510.0,
                    width: 80.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 510.0,
                    y: 490.0,
                    width: 60.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 620.0,
                    y: 540.0,
                    width: 25.0,
                    height: 50.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 680.0,
                    y: 540.0,
                    width: 15.0,
                    height: 50.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 650.0,
                    y: 510.0,
                    width: 80.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 650.0,
                    y: 490.0,
                    width: 60.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 510.0,
                    y: 470.0,
                    width: 20.0,
                    height: 40.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 650.0,
                    y: 470.0,
                    width: 20.0,
                    height: 40.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 580.0,
                    y: 450.0,
                    width: 160.0,
                    height: 15.0,
                },
            ],
            pigs: vec![
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 510.0,
                    y: 445.0,
                },
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 650.0,
                    y: 445.0,
                },
                PigPlacement {
                    pig_size: PigSize::Big,
                    x: 580.0,
                    y: 425.0,
                },
            ],
        }
    }

    fn create_level_4() -> LevelData {
        LevelData {
            birds: vec![BirdType::Blue, BirdType::Yellow, BirdType::Black, BirdType::Red, BirdType::White],
            blocks: vec![
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 450.0,
                    y: 550.0,
                    width: 30.0,
                    height: 30.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 500.0,
                    y: 550.0,
                    width: 15.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 550.0,
                    y: 550.0,
                    width: 30.0,
                    height: 30.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 475.0,
                    y: 510.0,
                    width: 80.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 525.0,
                    y: 510.0,
                    width: 80.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 500.0,
                    y: 490.0,
                    width: 20.0,
                    height: 40.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 550.0,
                    y: 490.0,
                    width: 20.0,
                    height: 40.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 525.0,
                    y: 460.0,
                    width: 100.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 650.0,
                    y: 550.0,
                    width: 25.0,
                    height: 80.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 700.0,
                    y: 550.0,
                    width: 15.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 750.0,
                    y: 550.0,
                    width: 25.0,
                    height: 80.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 675.0,
                    y: 510.0,
                    width: 120.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 650.0,
                    y: 490.0,
                    width: 60.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 700.0,
                    y: 490.0,
                    width: 60.0,
                    height: 15.0,
                },
            ],
            pigs: vec![
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 500.0,
                    y: 485.0,
                },
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 550.0,
                    y: 485.0,
                },
                PigPlacement {
                    pig_size: PigSize::Big,
                    x: 675.0,
                    y: 465.0,
                },
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 700.0,
                    y: 485.0,
                },
            ],
        }
    }

    fn create_level_5() -> LevelData {
        LevelData {
            birds: vec![BirdType::Black, BirdType::Blue, BirdType::Yellow, BirdType::White, BirdType::Red, BirdType::Red],
            blocks: vec![
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 400.0,
                    y: 560.0,
                    width: 30.0,
                    height: 30.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 450.0,
                    y: 560.0,
                    width: 15.0,
                    height: 80.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 500.0,
                    y: 560.0,
                    width: 30.0,
                    height: 30.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 475.0,
                    y: 510.0,
                    width: 100.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 430.0,
                    y: 510.0,
                    width: 50.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 520.0,
                    y: 510.0,
                    width: 50.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 450.0,
                    y: 490.0,
                    width: 20.0,
                    height: 40.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 500.0,
                    y: 490.0,
                    width: 20.0,
                    height: 40.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 475.0,
                    y: 460.0,
                    width: 80.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 600.0,
                    y: 560.0,
                    width: 30.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 650.0,
                    y: 560.0,
                    width: 15.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 700.0,
                    y: 560.0,
                    width: 30.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 625.0,
                    y: 520.0,
                    width: 120.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 600.0,
                    y: 500.0,
                    width: 15.0,
                    height: 40.0,
                },
                BlockPlacement {
                    block_type: BlockType::Ice,
                    x: 650.0,
                    y: 500.0,
                    width: 15.0,
                    height: 40.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 625.0,
                    y: 470.0,
                    width: 80.0,
                    height: 15.0,
                },
                BlockPlacement {
                    block_type: BlockType::Stone,
                    x: 550.0,
                    y: 400.0,
                    width: 25.0,
                    height: 25.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 500.0,
                    y: 400.0,
                    width: 15.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 600.0,
                    y: 400.0,
                    width: 15.0,
                    height: 60.0,
                },
                BlockPlacement {
                    block_type: BlockType::Wood,
                    x: 550.0,
                    y: 360.0,
                    width: 120.0,
                    height: 15.0,
                },
            ],
            pigs: vec![
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 450.0,
                    y: 485.0,
                },
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 500.0,
                    y: 485.0,
                },
                PigPlacement {
                    pig_size: PigSize::Big,
                    x: 625.0,
                    y: 455.0,
                },
                PigPlacement {
                    pig_size: PigSize::Small,
                    x: 650.0,
                    y: 485.0,
                },
                PigPlacement {
                    pig_size: PigSize::Big,
                    x: 550.0,
                    y: 335.0,
                },
            ],
        }
    }
}