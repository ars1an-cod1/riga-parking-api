struct Zone {
    zone: String,
    first_hour: f64,
    next_hour: f64,
    min_fee: f64,
}

impl Zone {
    fn info(&self) {
        println!(
            "Zone {}: {} EUR first hour, {} EUR next hours",
            self.zone, self.first_hour, self.next_hour
        );
    }

    fn price_for(&self, hours: f64) -> f64 {
        let price = self.first_hour + ((hours - 1.0) * self.next_hour);

        return price;
    }
}

fn main() {
    let d_zone = Zone {
        zone: String::from("D"),
        first_hour: 1.00,
        next_hour: 1.50,
        min_fee: 0.30,
    };

    d_zone.info();
}
