use std::net::IpAddr;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use chrono;
use neith::Neith;

fn main() {
    // DB set up
    let mut con = Neith::connect("shamash");
    let _table = con.execute("new table uptime with (id true, time false, up_bool false)");

    if con.exists_table("config".to_string()) {
        // Normal execution
        uptime_loop(con)
    } else {
        // First execution
        let _table = con.execute("new table config with (id true, value false)");
        let _set_interval = con.execute("new data config (id = 0,+ value = 30)");
        let _set_alt_interval = con.execute("new data config (id = 1,+ value = 5)");
        let _ = con.clone().save();
        // State is now set, normal execution
        uptime_loop(con)
    }
}
fn uptime_loop(mut con: Neith) {
    let interval_select = con.execute("select (value) from config where [id = 0]");
    let alt_interval_select = con.execute("select (value) from config where [id = 1]");
    if interval_select.is_ok() && alt_interval_select.is_ok() {
        let interval = &interval_select.unwrap().get_result().unwrap()[0].get_list().unwrap()[0];
        let temp = interval.get_float().unwrap().to_string().parse::<u64>().unwrap();
        let alt_interval = alt_interval_select.unwrap().get_result().unwrap()[0].get_list().unwrap()[0].get_float().unwrap().to_string().parse::<u64>().unwrap();
        loop {
            if internet_upstate() {
                println!("ONLINE");
                // Now I schleeeep
                thread::sleep(Duration::from_secs(temp))
            } else {
                // INTERNET DOWN!!!
                loop {
                    println!("OFFLINE!");
                    let new_con = write_upstate(false, con.clone());
                    con = new_con;
                    thread::sleep(Duration::from_secs(alt_interval));
                    if internet_upstate() {
                        break;
                    }
                }
            }
            
        }
    } else {
        // Do error handling!
        unimplemented!()
    }
}
fn write_upstate(up_bool: bool, mut con: Neith) -> Neith {
    let id = con.execute("get len of uptime").unwrap().get_result().unwrap()[0].get_float().unwrap();
    let time = chrono::Utc::now().to_rfc3339().to_string();
    let cmd = format!("new data uptime (id = {id},+ time = {time},+ up_bool = {})", up_bool.to_string());
    let _up_data = con.execute(&cmd);
    let _ = con.clone().save();
    con
}
fn internet_upstate() -> bool {
    if ping().is_ok() {
        true
    } else {
        false
    }
}
fn ping() -> Result<(), ping::Error> {
    let dur = Some(Duration::from_secs(2));
    // This needs sudo to work!!! so cargo run won't!
    let addr = IpAddr::from_str("209.85.233.101").unwrap();
    ping::ping(addr, dur, None, None, None, None)
}
