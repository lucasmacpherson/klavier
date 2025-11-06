# Klavier
---
Klavier is a lightweight stress-testing tool which sends predefined HTTP/S requests to an API endpoint in a stochastic manner. This is a higher-level (layer 7) simulation of application usage when compared to other tools operating at the transport or network layers. This enables realistic application testing and rapid development of testing profiles, while being less well-suited for investigating network or transport capabilities.

## How it Works
Klavier models a user as an HTTP/S client which makes requests in predefined series known as scenarios. Each scenario contains an ordered list of requests, analgous to a single user interaction in the application. The client will make one request at a time and wait for the response before sending the next request in the series, subject to a configurable minimum interval. Once all requests in a scenario have been sent, the client will then select a new scenario randomly from the set of available scenarios. This process repeats until the time period for the stress test has elapsed. 

Klavier relies on [tokio](https://github.com/tokio-rs/tokio) for efficiently multiplexing client requests to maximise CPU utilisation at high client counts while avoiding overhead from spawning additional threads. This is an improvement over solutions such as Apache Benchmark which rely on a single threaded, event-driven approach which can limit the capability of a single instance.

## Usage
### The CLI

### Example Profile
```toml
[target]
base_url = "https://api.example.com"

[scenarios]
[scenarios.health_check]
weight = 5
[[scenarios.health_check.requests]]
method = "GET"
path = "/api/health"
headers = {"User-Agent" = "Klavier/1.0"}

[scenarios.login]
weight = 1
[[scenarios.login.requests]]
method = "GET"
path = "/api/auth/login"
headers = {"User-Agent" = "Klavier/1.0"}

[timings]
test_duration_seconds = 60
min_scenario_interval_ms = 1000
```

## Future Improvements
- Variables in profiles
- Extensive request statistics (percentiles, variance, etc)
- Traffic patterns, ramp-up periods

## Notes
I wrote this application primarly to learn Rust and as such would welcome any experienced input on how to write and structure the application in a more idiomatic way.
