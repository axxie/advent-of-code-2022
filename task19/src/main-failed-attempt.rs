use core::slice::Iter;
use std::cmp::max;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
const TYPES: usize = 4;

#[derive(Debug, Copy, Clone)]
enum Material {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}

impl Material {
    const VALUES: [Self; 4] = [Self::ORE, Self::CLAY, Self::OBSIDIAN, Self::GEODE];
    fn values() -> Iter<'static, Material> {
        Self::VALUES.iter()
    }
}

struct Blueprint {
    ore_costs: [u32; TYPES],
    prev_cost: [u32; TYPES],
    robot_limits: [u32; TYPES],
}

impl Blueprint {
    // Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 18 clay. Each geode robot costs 4 ore and 19 obsidian.
    fn from_string(s: &str) -> Self {
        let re = regex::Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
        let captures = re.captures(s).unwrap();
        let mut result = Blueprint {
            ore_costs: [
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
                captures[5].parse().unwrap(),
            ],
            prev_cost: [
                0,
                0,
                captures[4].parse().unwrap(),
                captures[6].parse().unwrap(),
            ],
            robot_limits: [0; TYPES],
        };

        for robot_type in CLAY..=OBSIDIAN {
            result.robot_limits[robot_type] = result.prev_cost[robot_type + 1];
        }
        result.robot_limits[ORE] = *result.ore_costs.iter().max().unwrap();
        result.robot_limits[GEODE] = 1_000_000;

        return result;
    }
}

#[derive(Copy, Clone)]
struct State {
    time: u32,
    materials: [u32; TYPES],
    robots: [u32; TYPES],
    requested_robot: Option<usize>,
    global_max: u32,
}

impl State {
    fn step_forward(&mut self) -> Option<usize> {
        let mut result: Option<usize> = None;
        self.time += 1;
        for mat_type in 0..TYPES {
            self.materials[mat_type] += self.robots[mat_type];
        }
        if let Some(requested_robot) = self.requested_robot {
            self.robots[requested_robot] += 1;
            result = Some(requested_robot);
        }
        self.requested_robot = None;
        return result;
    }
    fn step_backward(&mut self, blueprint: &Blueprint, made_robot: Option<usize>) {
        self.time -= 1;
        if let Some(made_robot) = made_robot {
            // self.robots[made_robot] -= 1;
            self.unmake_robot(blueprint, made_robot);
        }
        for mat_type in 0..TYPES {
            self.materials[mat_type] -= self.robots[mat_type];
        }
    }

    fn request_robot(&mut self, blueprint: &Blueprint, robot_type: usize) -> bool {
        if self.materials[ORE] < blueprint.ore_costs[robot_type] {
            return false;
        }
        if robot_type > ORE && self.materials[robot_type - 1] < blueprint.prev_cost[robot_type] {
            return false;
        }
        if self.robots[robot_type] >= blueprint.robot_limits[robot_type] {
            return false;
        }

        self.materials[ORE] -= blueprint.ore_costs[robot_type];
        if robot_type > ORE {
            self.materials[robot_type - 1] -= blueprint.prev_cost[robot_type];
        }
        self.requested_robot = Some(robot_type);

        return true;
    }

    fn unmake_robot(&mut self, blueprint: &Blueprint, robot_type: usize) {
        self.materials[ORE] += blueprint.ore_costs[robot_type];
        if robot_type > ORE {
            self.materials[robot_type - 1] += blueprint.prev_cost[robot_type];
        }
        self.robots[robot_type] -= 1;
    }
}

const robot_types: [usize; 24] = [
    0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 3, 0, 0, 3, 0, 0, 0,
];

static mut states: Vec<State> = Vec::new();

fn get_max_geodes(blueprint: &Blueprint, state: &mut State) -> u32 {
    if state.time == 3 {
        if state.global_max < state.materials[GEODE] {
            state.global_max = state.materials[GEODE];
            if state.materials[GEODE] == 9 {
                let br = 0;
            }
            println!("{:?} - {:?}", state.robots, state.materials);
        }
        return state.materials[GEODE];
    }
    // println!("{}: {:?} - {:?}", state.time, state.robots, state.materials);
    let made_robot = state.step_forward();
    unsafe {
        states.push(state.clone());
    }
    let mut maximum = get_max_geodes(blueprint, state);
    for robot_type in 1..TYPES {
        // let mut maximum = 0;
        // let robot_type = robot_types[state.time as usize];
        // if robot_type == 0 {
        //     maximum = get_max_geodes(blueprint, state);
        // } else {
        if state.time == 2 && robot_type == 2 {
            let br = 0;
        }
        if state.request_robot(blueprint, robot_type) {
            let result = get_max_geodes(blueprint, state);
            // state.unmake_robot(blueprint, robot_type);
            if maximum < result {
                maximum = result;
            }
        }
    }

    unsafe {
        states.pop();
    }
    state.step_backward(blueprint, made_robot);

    return maximum;
}

fn main() {
    let blueprints: Vec<Blueprint> = std::io::stdin()
        .lines()
        .map(|x| Blueprint::from_string(&x.unwrap()))
        .collect();

    let mut state = State {
        time: 0,
        materials: [0; TYPES],
        robots: [0; TYPES],
        requested_robot: None,
        global_max: 0,
    };
    state.robots[ORE] = 1;
    // state.step_forward();
    // println!("{}: {:?} - {:?}", state.time, state.robots, state.materials);
    // state.step_forward();
    // println!("{}: {:?} - {:?}", state.time, state.robots, state.materials);
    // state.request_robot(&blueprints[0], CLAY);
    // state.step_forward();
    // println!("{}: {:?} - {:?}", state.time, state.robots, state.materials);
    // state.step_forward();
    // println!("{}: {:?} - {:?}", state.time, state.robots, state.materials);
    let max_geodes = get_max_geodes(&blueprints[0], &mut state);
    println!("Result {max_geodes}");
}
