use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client_derive_encode::{EncodeLabelSet, EncodeLabelValue};

//
// In prometheus, you have 4 important pieces of information that make up a metric:
// 1. metric name
// 2. metric help text
// 3. metric label(s)
// 4. metric value
//
// The easiest way I can think of it is that the metric name and value are the 
// descriptive name and value of the metric you're trying to measure, whereas the 
// labels are like tags where you can filter based on them. Many systems will probably
// report the same metric name with different values, but the labels for each system
// should be different to allow separation.
//

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum ServiceStateLabel {
    Stopped,
    Started,
    Dead,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ServiceResetLabels {
    pub owner: String,
    pub service_name: String,
    pub service_state: ServiceStateLabel,
}

#[derive(Debug)]
pub struct ServiceResetMetrics {
    pub name: String,
    pub help: String,
    pub metric: Family<ServiceResetLabels, Gauge>,
}

impl ServiceResetMetrics {
    pub fn default() -> Self {
        let name = String::from("service_resets");
        let help = String::from("how many times the service reset itself");
        let metric = Family::default();
        return Self { name, help, metric }.gather();
    }
    pub fn gather(self) -> Self {
        // go do the thing to get the value
        let resets_value = 8;
        let owner = String::from("Lucas");
        let service_name = String::from("myservice");
        let service_state = ServiceStateLabel::Started;
        // set the value on the gauge
        self.metric
            .get_or_create(&ServiceResetLabels {
                owner,
                service_name,
                service_state,
            })
            .set(resets_value);
        self
    }
}
