# graphql memory profiling with heaptrack and disabled auth

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

```shell
MEM_PROF=true cargo run --release --bin loadtest -- --host http://localhost:8080 -u 256
```

* Stop loadtest and trustify, and heaptrack will show the results
