use std::str::FromStr;
use crate::configuration::otel::OtelConfiguration;

pub fn create_otlp_tracer(otel_configuration: &OtelConfiguration) -> Option<opentelemetry_sdk::trace::Tracer> {
    let protocol = otel_configuration.otlp_protocol.clone();

    let tracer = opentelemetry_otlp::new_pipeline().tracing();
    let headers = parse_otlp_headers_from_config(otel_configuration);


    let tracer = match protocol.as_str() {
        "grpc" => {
            let mut exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_metadata(metadata_from_headers(headers));

            // Check if we need TLS
            let endpoint = otel_configuration.otlp_endpoint.clone();
            if endpoint.starts_with("https") {
                exporter = exporter.with_tls_config(Default::default());
            }
        tracer.with_exporter(exporter)
        }
        "http/protobuf" => {
            let exporter = opentelemetry_otlp::new_exporter()
                .http()
                .with_headers(headers.into_iter().collect());
            tracer.with_exporter(exporter)
        }
        p => panic!("Unsupported protocol {}", p),
    };

    Some(
        tracer
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap(),
    )
}

fn metadata_from_headers(headers: Vec<(String, String)>) -> tonic::metadata::MetadataMap {
    use tonic::metadata;

    let mut metadata = metadata::MetadataMap::new();
    headers.into_iter().for_each(|(name, value)| {
        let value = value
            .parse::<metadata::MetadataValue<metadata::Ascii>>()
            .expect("Header value invalid");
        metadata.insert(metadata::MetadataKey::from_str(&name).unwrap(), value);
    });
    metadata
}

fn parse_otlp_headers_from_config(otel_configuration: &OtelConfiguration) -> Vec<(String, String)> {
    let mut headers = Vec::new();

    let team_name = otel_configuration.honeycomb_team.clone();
    headers.push(
        ("x-honeycomb-team".into(), team_name.into())
    );
    headers
}
