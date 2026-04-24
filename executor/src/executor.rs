#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}

pub struct Executor {
    pose: Pose,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose }
    }

    pub fn execute(&mut self, cmds: &str) {
        for c in cmds.chars() {
            match c {
                'M' => match self.pose.heading {
                    'N' => self.pose.y += 1,
                    'S' => self.pose.y -= 1,
                    'E' => self.pose.x += 1,
                    'W' => self.pose.x -= 1,
                    _ => {}
                },
                'L' => self.pose.heading = match self.pose.heading {
                    'N' => 'W',
                    'W' => 'S',
                    'S' => 'E',
                    'E' => 'N',
                    _ => self.pose.heading,
                },
                'R' => self.pose.heading = match self.pose.heading {
                    'N' => 'E',
                    'E' => 'S',
                    'S' => 'W',
                    'W' => 'N',
                    _ => self.pose.heading,
                },
                _ => {}
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
