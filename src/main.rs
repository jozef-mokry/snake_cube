#[derive(Debug, Clone, Copy)]
pub enum Change {
    RotateRight = 0,
    RotateUp = 1,
    RotateLeft = 2,
    RotateDown = 3,
    NoChange = 4,
}

impl Change {
    fn apply_change(&self, Direction(a, b, c): Direction) -> Direction {
        match self {
            // NOTE: these don't necessarily correspond to up/left/right/down rotation correctly
            // but that does not matter
            Change::NoChange => Direction(a, b, c),
            Change::RotateRight => Direction(c, a, b),
            Change::RotateUp => Direction(b, c, a),
            Change::RotateDown => Direction(-b, -c, -a),
            Change::RotateLeft => Direction(-c, -a, -b),
        }
    }
}

impl From<u32> for Change {
    fn from(v: u32) -> Change {
        use Change::*;
        match v {
            0 => RotateRight,
            1 => RotateUp,
            2 => RotateLeft,
            3 => RotateDown,
            4 => NoChange,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Snake {
    steps: [Change; 27],
    state: u32, // optimization to quickly generate next folding of a snake
}

#[derive(Clone, Copy)]
struct Direction(i32, i32, i32);
const RIGHT: Direction = Direction(0, 1, 0);

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction(0, 1, 0) => write!(f, "RIGHT"),
            Direction(0, -1, 0) => write!(f, "LEFT"),
            Direction(0, 0, 1) => write!(f, "UP"),
            Direction(0, 0, -1) => write!(f, "DOWN"),
            Direction(1, 0, 0) => write!(f, "TOWARDS"),
            Direction(-1, 0, 0) => write!(f, "AWAY"),
            _ => panic!(),
        }
    }
}

impl Snake {
    const CHANGERS: [usize; 16] = [2, 3, 4, 6, 7, 9, 10, 11, 13, 15, 16, 17, 18, 20, 22, 24];
    fn new() -> Self {
        use Change::*;
        let mut steps = [NoChange; 27];
        for c in Self::CHANGERS {
            steps[c] = 0.into();
        }

        Snake { steps, state: 0 }
    }

    fn is_cube(&self, mut x: i32, mut y: i32, mut z: i32) -> bool {
        let mut visited = [[[false; 3]; 3]; 3];

        let mut dir = RIGHT;

        for step in self.steps {
            if x < 0 || x == 3 || y < 0 || y == 3 || z < 0 || z == 3 {
                return false;
            }
            if visited[x as usize][y as usize][z as usize] {
                return false;
            }
            visited[x as usize][y as usize][z as usize] = true;
            dir = step.apply_change(dir);
            x += dir.0;
            y += dir.1;
            z += dir.2;
        }

        true
    }

    fn rotate_cube(&mut self) {
        self.state += 1;
        let mut v = self.state;
        for &c in Self::CHANGERS.iter() {
            let d = v & 3;
            self.steps[c] = d.into();
            v >>= 2;
        }
    }

    fn solve() {
        let mut snake = Snake::new();
        // solution seems to exist only for 0,0,0
        for (x, y, z) in [(0, 0, 0), (1, 0, 0), (1, 0, 1)] {
            for i in 0u64..(u32::MAX as u64) {
                if snake.is_cube(x, y, z) {
                    println!("Solution {x} {y} {z}:");
                    let mut dir = RIGHT;
                    for step in snake.steps {
                        println!("{dir:?}");
                        dir = step.apply_change(dir);
                    }
                }
                snake.rotate_cube();
                if i % 100_000_000 == 0 {
                    println!("{i}");
                }
            }
            println!("Done: {x} {y} {z}");
        }
    }
}

fn main() {
    Snake::solve();
}
