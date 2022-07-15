trait DurationTime {
    fn time(&self) -> u8;
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl DurationTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 50,
            TrafficLight::Green => 60,
            TrafficLight::Yellow => 5,
        }
    }
}

fn main() {
    let light = TrafficLight::Green;
    let seconds = light.time();
    print!("light duration time is {} seconds", seconds)
}
