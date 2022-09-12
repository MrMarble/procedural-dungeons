use std::fmt;

use crate::{map::Map, map_builders::*};

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum Algorithm {
    #[default]
    None = -1,
    Random,
    Rooms,
    Bsp,
    BspInterior,
    CellularAutomata,
    Drunkard,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Algorithm::None => write!(f, "None"),
            Algorithm::Random => write!(f, "Random walls"),
            Algorithm::Rooms => write!(f, "Rooms and corridors"),
            Algorithm::Bsp => write!(f, "BSP"),
            Algorithm::BspInterior => write!(f, "BSP without corridors"),
            Algorithm::CellularAutomata => write!(f, "Cellular automata"),
            Algorithm::Drunkard => write!(f, "Drunkard's walk"),
        }
    }
}

impl Algorithm {
    pub fn all() -> Vec<Self> {
        vec![
            Algorithm::Random,
            Algorithm::Rooms,
            Algorithm::Bsp,
            Algorithm::BspInterior,
            Algorithm::CellularAutomata,
            Algorithm::Drunkard,
        ]
    }
    pub fn get(&self) -> Box<dyn MapBuilder> {
        match self {
            Algorithm::Random => Box::new(RandomMap::default()),
            Algorithm::Rooms => Box::new(RoomsMap::default()),
            Algorithm::Bsp => Box::new(BspMap::default()),
            Algorithm::BspInterior => Box::new(BspInteriorMap::default()),
            Algorithm::CellularAutomata => Box::new(CellularAutomataBuilder::default()),
            Algorithm::Drunkard => Box::new(DrunkardsWalkBuilder::open_area()),
            _ => panic!("No algorithm selected"),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Algorithm::Random => "Place walls at the edges and randomly elsewhere",
            Algorithm::Rooms => "Place random rooms and connect them with corridors",
            Algorithm::Bsp => {
                "Divide the map into rooms with a binary space partitioning algorithm"
            },
            Algorithm::BspInterior => {
                "Divide the map into rooms with a binary space partitioning algorithm, filling the whole map"
            },
            Algorithm::CellularAutomata => "Use cellular automata to generate the map",
            Algorithm::Drunkard => "Use a drunkard's walk to generate the map",
            _ => panic!("No algorithm selected"),
        }
    }

    pub fn options(&self) -> &[Option] {
        match self {
            Algorithm::Random => &[Option {
                name: "Ratio",
                value: 4,
                min: 1,
                max: 10,
            }],
            Algorithm::Rooms => &[
                Option {
                    name: "Max rooms",
                    value: 5,
                    min: 1,
                    max: 30,
                },
                Option {
                    name: "Min room size",
                    value: 4,
                    min: 6,
                    max: 10,
                },
                Option {
                    name: "Max room size",
                    value: 10,
                    min: 10,
                    max: 15,
                },
            ],
            Algorithm::Bsp => &[Option {
                name: "Max rooms",
                value: 240,
                min: 1,
                max: 350,
            }],
            Algorithm::BspInterior => &[
                Option {
                    name: "Min room size",
                    value: 6,
                    min: 6,
                    max: 100,
                },
                Option {
                    name: "Vertical split percentage",
                    value: 50,
                    min: 1,
                    max: 100,
                },
            ],
            Algorithm::CellularAutomata => &[
                Option {
                    name: "Floor percentage",
                    value: 55,
                    min: 1,
                    max: 100,
                },
                Option {
                    name: "Iterations",
                    value: 15,
                    min: 1,
                    max: 100,
                },
            ],
            _ => &[],
        }
    }
}

#[derive(Clone)]
pub struct Option {
    pub name: &'static str,
    pub value: i32,
    pub min: i32,
    pub max: i32,
}
