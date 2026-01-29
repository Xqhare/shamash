mod local_outage;
pub use local_outage::local_outage;
mod isp_outage;
pub use isp_outage::isp_outage;
mod online;
pub use online::online;
mod diagnosing;
pub use diagnosing::diagnosing;
mod complete_network_outage;
pub use complete_network_outage::complete_network_outage;

#[derive(PartialEq, Debug)]
pub enum ConnectionState {
    Online,
    Diagnosing,
    IspOutage,
    LocalOutage,
    CompleteNetworkOutage,
}
