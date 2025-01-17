use appinsights::TelemetryClient;

pub fn init(app_insights_key: &str) -> Option<TelemetryClient> {
    if !app_insights_key.is_empty() {
        let client = TelemetryClient::new(app_insights_key.to_string());
        client.track_event("App Started");
        Some(client)
    } else {
        None
    }
}
