use std::{path::PathBuf, time::Instant};

use horae::Utc;

pub struct Logger {
    logs: Vec<String>,
    pub log_dir_path: String,
    pub log_start: Instant,
    pub event_type: EventType,
}

pub enum EventType {
    IspOutage,
    LocalOutage,
    CompleteNetworkOutage,
    Online,
}

impl Logger {
    pub fn new(log_dir_path: String) -> Self {
        let (isp_dir, local_dir, router_dir) = {
            let isp_path = PathBuf::from(&log_dir_path);
            let local_path = PathBuf::from(&log_dir_path);
            let router_path = PathBuf::from(&log_dir_path);
            (
                isp_path.join("isp_outage"),
                local_path.join("local_outage"),
                router_path.join("complete_network_outage"),
            )
        };
        if let Err(e) = std::fs::create_dir_all(isp_dir) {
            panic!("OS ERROR {e}")
        }
        if let Err(e) = std::fs::create_dir_all(local_dir) {
            panic!("OS ERROR {e}")
        }
        if let Err(e) = std::fs::create_dir_all(router_dir) {
            panic!("OS ERROR {e}")
        }
        Self {
            logs: vec![],
            log_dir_path,
            log_start: Instant::now(),
            event_type: EventType::Online,
        }
    }

    pub fn has_unsaved_log(&self) -> bool {
        !self.logs.is_empty()
    }

    pub fn add_log_line(&mut self, log_line: String) {
        self.logs.push(log_line);
    }

    pub fn add_small_separator(&mut self) {
        self.logs.push(format!("{}-", make_long_repeat("- ", 20)));
    }

    pub fn add_large_separator(&mut self) {
        self.logs.push("\n".to_string());
        self.logs.push(format!("{}=", make_long_repeat("=-", 20)));
        self.logs.push("\n".to_string());
    }

    fn make_log(&self) -> String {
        let mut out = self.logs.join("\n");
        out.push('\n');
        out
    }

    pub fn end_log(&mut self, last_log: String) {
        self.add_log_line(last_log);
        self.add_small_separator();
        let log_duration = self.log_start.elapsed();
        self.add_log_line("\n".to_string());
        self.add_log_line(format!(
            "Time from Log creation to saving: {} seconds",
            log_duration.as_secs_f32()
        ));
        self.add_large_separator();
        if let Err(e) = self.write_log() {
            panic!("OS ERROR {e}")
        }
        self.reset();
    }

    fn write_log(&self) -> Result<(), std::io::Error> {
        let now = Utc::now();
        match self.event_type {
            EventType::IspOutage => {
                let this_log_path = PathBuf::from(self.log_dir_path.clone())
                    .join(format!("isp_outage/{now}.log"));
                std::fs::write(this_log_path, self.make_log())
            }
            EventType::LocalOutage => {
                let this_log_path = PathBuf::from(self.log_dir_path.clone())
                    .join(format!("local_outage/{now}.log"));
                std::fs::write(this_log_path, self.make_log())
            }
            EventType::CompleteNetworkOutage => {
                let this_log_path = PathBuf::from(self.log_dir_path.clone())
                    .join(format!("complete_network_outage/{now}.log"));
                std::fs::write(this_log_path, self.make_log())
            }
            EventType::Online => {
                let this_log_path =
                    PathBuf::from(self.log_dir_path.clone()).join(format!("{now}.log"));
                std::fs::write(this_log_path, self.make_log())
            }
        }
    }

    pub fn reset(&mut self) {
        self.logs.clear();
        self.log_start = Instant::now();
        self.event_type = EventType::Online;
    }
}

fn make_long_repeat(pattern: &str, times: usize) -> String {
    let mut result = String::new();
    for _ in 0..times {
        result.push_str(pattern);
    }
    result
}
