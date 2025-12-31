# Card Router

A Rust-based card routing and payment orchestration service that integrates with multiple payment providers.

## Features

- **Multi-Provider Support**: Integrates with Adyen, Lithic, and Footprint
- **Transaction Routing**: Rule-based routing engine for card transactions
- **Wallet Management**: Create and manage virtual wallets with passthrough cards
- **Ledger System**: Track funds and transaction history
- **Webhook Handling**: Process real-time events from payment providers
- **Auth0 Integration**: JWT-based authentication

## Tech Stack

- **Framework**: Actix-web
- **Database**: PostgreSQL with Diesel ORM (async)
- **Cache**: Redis
- **Tracing**: OpenTelemetry with Honeycomb support

## Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- Redis

### Configuration

Copy the example configuration and set your environment variables:

```bash
# Database
APP_DATABASE__USERNAME=
APP_DATABASE__PASSWORD=

# Adyen
APP_ADYEN__API_KEY=
APP_ADYEN__MERCHANT_ACCOUNT_NAME=

# Auth0
APP_AUTH0__AUTHORITY=
APP_AUTH0__AUDIENCE=
APP_AUTH0__DOMAIN=
APP_AUTH0__CLIENT_ORIGIN_URL=

# Lithic
APP_LITHIC__API_KEY=
APP_LITHIC__MODE=

# Footprint
APP_FOOTPRINT__SECRET_KEY=

# OpenTelemetry
APP_OTEL__HONEYCOMB_TEAM=
OTEL_EXPORTER_OTLP_ENDPOINT=
OTEL_SERVICE_NAME=

RUST_LOG=info
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test -- --nocapture --test-threads=1
```

## Project Structure

```
src/
├── adyen/          # Adyen payment provider integration
├── asa/            # Authorization service adapter
├── auth/           # Authentication (Auth0)
├── category/       # Transaction categories
├── charge/         # Charge processing
├── configuration/  # App configuration
├── footprint/      # Footprint KYC integration
├── ledger/         # Transaction ledger
├── lithic/         # Lithic card issuing integration
├── rule/           # Routing rules engine
├── user/           # User management
├── wallet/         # Wallet management
├── webhooks/       # Webhook handlers
└── ...
```

## License

This project is licensed under the GNU Affero General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
