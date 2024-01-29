pub struct Satellite {
    id: String,
    position: Position,
    orientation: Orientation,
    power_level: f64,
    thrust: f64,
    // TODO add more fields
}

struct Position {
    x: f64,
    y: f64,
    z: f64,
}

struct Orientation {
    pitch: f64,
    yaw: f64,
    roll: f64,
}

impl Satellite {
    pub fn new(id: String) -> Satellite {
        println!("New satellite {} up and running...", id);
        Satellite {
            id,
            position: Position{ x: 0.0, y: 0.0, z: 0.0 },
            orientation: Orientation{ pitch: 0.0, yaw: 0.0, roll: 0.0 },
            thrust: 1.0,
            power_level: 100.0,
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        if self.power_level <= 0.0 {
            return
        }
        println!("Updating {}...", self.id);
        println!("{}", std::iter::repeat("-").take(20).collect::<String>());
        println!("Telemetry");
        println!("Position: ({}, {}, {})", self.position.x, self.position.y, self.position.z);
        println!("Orientation: ({}, {}, {})", self.orientation.pitch, self.orientation.yaw, self.orientation.roll);
        println!("thrust: {}", self.thrust);
        println!("Power: {}", self.power_level);
        println!("{}", std::iter::repeat("#").take(50).collect::<String>());

        self.update_position();
        self.update_orientation();
        self.update_power();

    }

    fn update_position(&mut self) {
        self.position.y += self.thrust
    }

    fn update_orientation(&mut self) {
        // update pitch

        // update yaw

        // update roll
    }

    fn update_power(&mut self) {
        self.power_level -= self.thrust;
    }
}