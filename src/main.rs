use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use neith::Neith;

fn main() {
    // DB set up
    let mut con = Neith::connect("shamash");
    let _table = con.execute("new table uptime with (id true, time false, up_bool false)");

}

fn ping() -> Result<(), ping::Error> {
    let dur = Some(Duration::from_secs(2));
    // This needs sudo to work!!! so cargo run won't!
    let addr = IpAddr::from_str("209.85.233.101").unwrap();
    // let addr = "209.85.233.101:80".parse::<IpAddr>().unwrap();
    ping::ping(addr, dur, None, None, None, None)
}
