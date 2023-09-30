mod appointment;
use crate::appointment::Appointment;

fn main() {
    let a: Appointment = Appointment::new(
        String::from("test"),
        String::from("10.01.2023  02:00"),
        String::from("10.01.2023  03:00"),
        Some(String::from("test")),
        Some(true),
        Some(true),
    );
    a.create_appointment();
}
