# trustify loadtest

A set of simple [goose](https://book.goose.rs/) load tests against the web and rest endpoints.

## quickstart

1. Ensure trustify is running.

2. Set environment variables for OIDC authentication:
   ```bash
   export ISSUER_URL = "http://localhost:8090/realms/trustify"
   export CLIENT_ID =  "testing-user"
   export CLIENT_SECRET = "****************"
   ```

   To change wait times between http invokes set the following env vars:

   ```bash
   export WAIT_TIME_FROM = 1
   export WAIT_TIME_TO =  2
   ```

   Alternately, for no wait times between http invokes set these env vars to 0.

3. To load trustify endpoints with 3 concurrent users.
   ```bash
   cargo run --release --bin loadtest -- --host http://localhost:8080 -u 3
   ```

   To stop load test hit [ctl-C], which should generate aggregate statistics.

   To load trustify endpoints against 10 concurrent users, generating an html report.

   ```bash
   cargo run --release -- --host http://localhost:8080  --report-file=report.html --no-reset-metrics -u 10
   ```

4. More goose run-time options [here](https://book.goose.rs/getting-started/runtime-options.html)

## Memory profiling with heaptrack

* Install [heaptrack](https://github.com/KDE/heaptrack)

```shell
sudo dnf install heaptrack
```

* Clone trustify and generate a database dump

```shell
cargo run --bin xtask generate-dump
```

* Use the generated dump
  * Change `trustify/etc/deploy/compose/compose.yaml`
  * Add the following:

```yaml
volumes:
  - /dump_path_here/dump.sql:/docker-entrypoint-initdb.d/dump.sql:Z
```

* Open a terminal and start postgres

```shell
podman-compose -f etc/deploy/compose/compose.yaml up
```

* Change `trustify/Cargo.toml`
  * Add the following:

```yaml
[profile.release]
debug = true
```

* Clean and build with `--release`

```shell
cargo clean ; cargo build --release
```

* Open a terminal and start trustify with heaptrack and graphql feature

```shell
cd target/release/
TRUSTD_WITH_GRAPHQL=true heaptrack ./trustd api --db-password eggs --devmode --auth-disabled
```

* Run loadtest

* Stop loadtest and trustify, and heaptrack will show the results
