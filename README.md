# trustify loadtest

A set of simple [goose](https://book.goose.rs/) load tests against the web and rest endpoints.

## quickstart

1. Ensure trustify is running.

2. Set environment variables for OIDC authentication:
   ```bash
   export ISSUER_URL = "http://localhost:8090/realms/trustify"
   export CLIENT_ID = "testing-user"
   export CLIENT_SECRET = "****************"
   ```

   If you're using the devmode auth settings, you can use:

   ```bash
   export ISSUER_URL = "http://localhost:8090/realms/trustify"
   export CLIENT_ID = "testing-user"
   export CLIENT_SECRET = "R8A6KFeyxJsMDBhjfHbpZTIF0GWt43HP"
   ```

   Or prefix calls with:

   ```shell
   env ISSUER_URL="http://localhost:8090/realms/trustify" CLIENT_ID="testing-user" CLIENT_SECRET="R8A6KFeyxJsMDBhjfHbpZTIF0GWt43HP"
   ```

   To change wait times between http invokes set the following env vars:

   ```bash
   export WAIT_TIME_FROM = 1
   export WAIT_TIME_TO =  2
   ```

   Alternately, for no wait times between http invokes set these env vars to 0.

3. Choose a scenario file or let it evaluate one

   Other provide an existing file using `SCENARIO_FILE`:

   ```bash
   export SCENARIO_FILE=empty.json5 # for disabling all test with variables
   ```

   Or provide `DATABASE_URL` to generate one on the fly:

   ```bash
   export DATABASE_URL="postgresql://postgres:trustify@localhost:5432/trustify"
   ```

   See below for creating a scenario file. Or check the folder [scenarios/](scenarios/) for existing files.

4. To load trustify endpoints with 3 concurrent users.
   ```bash
   cargo run --release --bin loadtest -- --host http://localhost:8080 -u 3
   ```

   To stop load test hit [ctl-C], which should generate aggregate statistics.

   To load trustify endpoints against 10 concurrent users, generating an html report.

   ```bash
   cargo run --release -- --host http://localhost:8080  --report-file=report.html --no-reset-metrics -u 10
   ```

5. More goose run-time options [here](https://book.goose.rs/getting-started/runtime-options.html)

## Using an existing database dump

You can either create a database dump using the following command in the `trustify` repository:

```shell
cargo run --bin xtask generate-dump
```

Or, download one from the S3 bucket. e.g.:
`https://trustify-dumps.s3.eu-west-1.amazonaws.com/20250604T002104Z/dump.sql.gz`

Then, add the dump to the compose startup `trustify/etc/deploy/compose/compose.yaml` (in the `trustify` repository):

```yaml
    volumes:
      - /dump_path_here/dump.sql.gz:/docker-entrypoint-initdb.d/dump.sql.gz:Z
```

Start Postgres like you normally would do:

```shell
podman-compose -f etc/deploy/compose/compose.yaml up
```

## Scenario files

Some tests require a single document, like an SBOM. This information can be provided using a "scenario" file, using the
environment variable `SCENARIO_FILE`. All fields are mandatory, though it is possible to disable tests by providing
a `null`, value:

```json5
{
  "disable_me": null,
}
```

The repository has a default file (`empty.json5`), disabling all such tests.

If the scenario file is not being provided, the scale tests will try to auto-evaluate candidate documents. However, this
is not reproducible, and should only be used for local testing.

It is possible to generate the content for a scenario file by setting the environment variable `GENERATE_SCENARIO=true`:

```bash
env GENERATE_SCENARIO=true DATABASE_URL=postgresql://postgres:trustify@localhost:5432/trustify cargo run --release 
```

## Request timeouts

To prevent calls from failing due to timeouts, it is possible to use `REQUEST_TIMEOUT` with a humantime format
(`1s`, `1m` = "60 seconds"). The default is `5m`.

## Memory profiling with heaptrack

* Install [heaptrack](https://github.com/KDE/heaptrack)

```shell
sudo dnf install heaptrack
```

* [Use a database dump](#using-an-existing-database-dump)

* Change `trustify/Cargo.toml`
    * Add the following:

```toml
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
