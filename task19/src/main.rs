use std::{cmp::min, collections::HashMap};

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
const TYPES: usize = 4;

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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Resources {
    robots: [u32; TYPES],
    materials: [u32; TYPES],
}

impl Resources {
    fn new() -> Self {
        Resources {
            robots: [1, 0, 0, 0],
            materials: [0; TYPES],
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

        return true;
    }

    fn gather_resources(&mut self, time: usize, time_limit: usize, blueprint: &Blueprint) {
        let remaining_time = (time_limit - time) as u32;
        for r in 0..self.robots.len() {
            self.materials[r] += self.robots[r];
        }
        self.materials[ORE] = min(
            self.materials[ORE],
            remaining_time * blueprint.robot_limits[ORE] - (remaining_time - 1) * self.robots[ORE],
        );
        self.materials[CLAY] = min(
            self.materials[CLAY],
            remaining_time * blueprint.prev_cost[OBSIDIAN]
                - (remaining_time - 1) * self.robots[CLAY],
        );
        self.materials[OBSIDIAN] = min(
            self.materials[OBSIDIAN],
            remaining_time * blueprint.prev_cost[GEODE]
                - (remaining_time - 1) * self.robots[OBSIDIAN],
        );
    }

    fn proceed(
        &self,
        time: usize,
        time_limit: usize,
        blueprint: &Blueprint,
        robot_type: Option<usize>,
    ) -> Option<Self> {
        let mut result: Resources = self.clone();

        if robot_type.is_none() {
            result.gather_resources(time, time_limit, blueprint);
            return Some(result);
        }

        if !result.request_robot(blueprint, robot_type.unwrap()) {
            return None;
        }
        result.gather_resources(time, time_limit, blueprint);
        result.robots[robot_type.unwrap()] += 1;
        return Some(result);
    }
}

type Cache = Vec<HashMap<Resources, u32>>;

trait CacheTrait {
    fn create(time_limit: usize) -> Self;
}

impl CacheTrait for Cache {
    fn create(time_limit: usize) -> Self {
        let mut result: Cache = Vec::new();
        for _ in 0..time_limit {
            result.push(HashMap::new());
        }
        result
    }
}

fn get_max_geodes(
    time: usize,
    time_limit: usize,
    blueprint: &Blueprint,
    resources: &Option<Resources>,
    cache: &mut Cache,
) -> u32 {
    if resources.is_none() {
        return 0;
    }
    let resources = &resources.unwrap();

    if time >= time_limit {
        return resources.materials[GEODE];
    }

    let cached_result = cache[time].get(&resources);
    if cached_result.is_some() {
        return *cached_result.unwrap();
    }

    let mut maximum = get_max_geodes(
        time + 1,
        time_limit,
        blueprint,
        &resources.proceed(time, time_limit, blueprint, None),
        cache,
    );
    for robot_type in 0..TYPES {
        let result = get_max_geodes(
            time + 1,
            time_limit,
            blueprint,
            &resources.proceed(time, time_limit, blueprint, Some(robot_type)),
            cache,
        );
        if maximum < result {
            maximum = result;
        }
    }
    cache[time].insert(*resources, maximum);

    return maximum;
}

fn main() {
    let blueprints: Vec<Blueprint> = std::io::stdin()
        .lines()
        .map(|x| Blueprint::from_string(&x.unwrap()))
        .collect();

    let first = false;
    if first {
        let mut sum = 0u32;
        for (i, blueprint) in blueprints.iter().enumerate() {
            let mut cache = Cache::create(24);
            let max_geodes = get_max_geodes(0, 24, blueprint, &Some(Resources::new()), &mut cache);
            println!("{} {}", i + 1, max_geodes);
            sum += (i as u32 + 1) * max_geodes;
        }
        println!("Result {sum}");
    } else {
        let mut product = 1u32;
        for blueprint in &blueprints[0..min(3, blueprints.len())] {
            let mut cache = Cache::create(32);
            let max_geodes = get_max_geodes(0, 32, blueprint, &Some(Resources::new()), &mut cache);
            println!("{}", max_geodes);
            product *= max_geodes;
        }
        println!("Result {product}");
    }
}
