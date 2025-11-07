# Klavier
Klavier is a lightweight stress-testing tool which sends predefined HTTP/S requests to an API endpoint in a stochastic manner. This is a higher-level (layer 7) simulation of application usage when compared to other tools operating at the transport or network layers. This enables realistic application testing and rapid development of testing profiles, while being less well-suited for investigating network or transport capabilities.

## How it Works
Klavier models a user as an HTTP/S client which makes requests in predefined series known as scenarios. Each scenario contains an ordered list of requests, analgous to a single user interaction in the application. The client will make one request at a time and wait for the response before sending the next request in the series. Once all requests in a scenario have been sent, the client will then select a new scenario randomly from the set of available scenarios. This process repeats until the time period for the stress test has elapsed. 

Klavier relies on [tokio](https://tokio.rs/) for efficiently multiplexing client requests to maximise CPU utilisation at high client counts while avoiding overhead from spawning additional threads. This is an improvement over solutions such as Apache Benchmark which rely on a single threaded, event-driven approach which can limit the capability of a single instance.

## Usage:
### The CLI
To run a profile with 16 clients, saving only the request statistics and discarding the raw results, the following command may be used
```
klavier profile.toml 16 --stats-out request_statistics.csv
```

Klavier uses [clap](https://docs.rs/clap/latest/clap/) for its command-line interface.
### Example Profile
```toml
[target]
# The single API root is defined here as an absolute URL
base_url = "https://api.example.com"


[scenarios]
# Scenarios are selected at random and contain an ordered array of requests

[scenarios.health_check]
# Probability of selection is scenario weight divided by the weight of all scenarios
weight = 5

[[scenarios.health_check.requests]]
# Each request requires a method and API path (relative to base URL)
method = "GET"
path = "/api/health"
# Headers defined as a table, inlined here, and are optional
# The Klaiver User-Agent header will be added by default
headers = {"Connection" = "keep-alive"}

[[scenarios.health_check.requests]]
# This request will be sent once the client receives a response from the previous check
method = "GET"
path = "/api/health/detailed"


[scenarios.login]
# login scenario will be selected 1 in 6 times on average
weight = 1
[[scenarios.open_app.requests]]
method = "POST"
path = "/api/auth/login"
# Request body is converted directly to bytes for use in the request
# This example uses a single JSON string to store user credentials
body = '{ "username": "user", "password: "XXXXXXXX" }'


[timings]
# Clients will continuously make requests as described until this period has elapsed
# This is tracked at the client level, not at the stress test level, so clients which
# were started earlier will finish earlier.
test_duration_seconds = 600
# If a scenario is resolved in a shorter period than this interval, the client will
# wait the remaining time before running the next scenario.
min_scenario_interval_ms = 200
```

## Future Improvements
- Variables in profiles
- Extensive request statistics (percentiles, variance, etc)
- Traffic patterns, ramp-up periods
- Utility functions for easily generating profiles, headers and request bodies.

## Notes
I wrote this application primarly to learn Rust and as such would welcome any experienced input on how to write and structure the application in a more idiomatic way.
