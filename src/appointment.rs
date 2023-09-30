pub struct Appointment {
    pub title: String,
    pub body: String,
    pub start_time: String,
    pub end_time: String,
    pub display: bool,
    pub verbose: bool,
}

impl Appointment {
    pub fn new(
        title: String,
        start_time: String,
        end_time: String,
        body: Option<String>,
        display: Option<bool>,
        verbose: Option<bool>,
    ) -> Appointment {
        Appointment {
            title,
            body: body.unwrap_or("".to_string()),
            start_time,
            end_time,
            display: display.unwrap_or(false),
            verbose: verbose.unwrap_or(false),
        }
    }
    pub fn create_appointment(self) {
        // call a powershell command that creates a appointment over the com api with outlook
        // make it a format string and paste the right attributes in
        let mut com = format!(
            r#"
            $outlook = New-Object -ComObject Outlook.Application
            $appointment = $outlook.CreateItem(1)
            $appointment.Subject = "{}"
            $appointment.Start = "{}"
            $appointment.End = "{}"
            $appointment.Body = "{}"
            $appointment.ReminderSet = $true
            $appointment.ReminderMinutesBeforeStart = 15
            $appointment.Save()
            "#,
            self.title, self.start_time, self.end_time, self.body
        );
        if self.display {
            com += "\n$appointment.Save()";
        };
        print!("{}", com);
        // execute the command with powershell
        let output = std::process::Command::new("powershell.exe")
            .arg("-Command")
            .arg(com)
            .output()
            .expect("failed to execute process");
        println!("stdout: {:#?}\nstderr: {:#?}", output.stdout, output.stderr)
    }
}
