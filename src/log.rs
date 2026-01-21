use std::path::PathBuf;

use horae::Utc;


pub struct Logger {
    logs: Vec<String>,
    log_dir_path: String,
}

impl Logger {
    pub fn new(log_dir_path: String) -> Self {
        Self { logs: vec![], log_dir_path }
    }

    pub fn add_log_line(&mut self, log_line: String) {
        self.logs.push(log_line);
    }

    pub fn add_separator(&mut self) {
        self.logs.push("\n".to_string());
        self.logs.push("---------------------".to_string());
        self.logs.push("\n".to_string());
    }

    fn make_log(&self) -> String {
        self.logs.join("\n")
    }

    pub fn end_log(&mut self, last_log: String) {
        self.add_log_line(last_log);
        if let Err(e) = self.write_log() {
            panic!("OS ERROR {}", e)
        }
        self.clear();
    }

    fn write_log(&self) -> Result<(), std::io::Error> {
        let now = Utc::now();
        let this_log_path = PathBuf::from(self.log_dir_path.clone()).join(format!("{}.log", now));
        std::fs::write(this_log_path, self.make_log())
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }
}
