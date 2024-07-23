# Comparing Trustify (PM-mode) releases

```shell
v0.1.0-alpha.10
v0.1.0-alpha.11
v0.1.0-alpha.12
```

Start Hyperfoil

```shell
podman run --rm -it --net host quay.io/hyperfoil/hyperfoil cli
```

Run

```shell
start-local
upload https://raw.githubusercontent.com/trustification/scale-testing/main/benchmarks/hyperfoil.yml
```

Run Trustify in other terminal

```shell
git checkout v0.1.0-alpha.10
cargo run --bin trustd --release
```

Run Hyperfoil

```shell
run
```

Stop Trustify, checkout other tag and run Hyperfoil again.

```shell
git checkout v0.1.0-alpha.11
cargo run --bin trustd --release
```

Run Hyperfoil

```shell
run
```

Stop Trustify, checkout other tag and run Hyperfoil again.

```shell
git checkout v0.1.0-alpha.12
cargo run --bin trustd --release
```

Run Hyperfoil

```shell
run
```

Compare `v0.1.0-alpha.10` and `v0.1.0-alpha.11`

```shell
[hyperfoil@in-vm]$ compare 0000 0001
Comparing runs 0000 and 0001
PHASE   METRIC                REQUESTS     MEAN                  p50                   p90                   p99                   p99.9                 p99.99
basic   fetchSboms             +0(+0.00%)   -5.25 ms(-1200.23%)   -5.26 ms(-1200.94%)   -5.26 ms(-1200.94%)   -5.26 ms(-1200.94%)   -5.26 ms(-1200.94%)   -5.26 ms(-1200.94%)
rampUp  fetchAdvisories       +70(+2.37%)    -52.27 μs(-10.73%)     -24.58 μs(-4.72%)    -77.82 μs(-12.75%)   -643.07 μs(-88.70%)    -2.30 ms(-130.70%)    -6.03 ms(-161.40%)
rampUp  fetchImporters        +70(+2.37%)    -58.62 μs(-12.22%)     -24.58 μs(-4.80%)    -77.82 μs(-12.84%)   -679.94 μs(-91.21%)    -2.16 ms(-133.33%)  -29.11 ms(-1493.28%)
rampUp  fetchSboms            +70(+2.37%)     -56.93 μs(-8.99%)     -32.77 μs(-5.13%)   -102.40 μs(-13.37%)   -524.29 μs(-41.83%)    -6.11 ms(-176.78%)    +15.86 ms(+61.42%)
rampUp  fetchVulnerabilities  +70(+2.37%)    -53.56 μs(-10.40%)     -24.58 μs(-4.62%)    -94.21 μs(-15.23%)  -806.91 μs(-103.14%)    -2.42 ms(-100.00%)    +10.49 ms(+33.33%)
```

Compare `v0.1.0-alpha.11` and `v0.1.0-alpha.12`

```shell
[hyperfoil@in-vm]$ compare 0001 0002
Comparing runs 0001 and 0002
PHASE   METRIC                REQUESTS     MEAN                 p50                  p90                  p99                  p99.9                p99.99
basic   fetchSboms             +0(+0.00%)  +105.47 μs(+24.12%)  +106.50 μs(+24.30%)  +106.50 μs(+24.30%)  +106.50 μs(+24.30%)  +106.50 μs(+24.30%)   +106.50 μs(+24.30%)
rampUp  fetchAdvisories       -54(-1.82%)    -17.66 μs(-3.76%)    -10.24 μs(-2.01%)    -12.29 μs(-2.05%)     -4.10 μs(-0.57%)  -237.57 μs(-15.59%)     -1.20 ms(-47.10%)
rampUp  fetchImporters        -54(-1.82%)    -18.67 μs(-4.05%)     -6.14 μs(-1.21%)    -16.38 μs(-2.78%)    -36.86 μs(-5.20%)    -73.73 μs(-4.76%)    +180.22 μs(+9.24%)
rampUp  fetchSboms            -55(-1.85%)    -29.04 μs(-4.81%)     +8.19 μs(+1.28%)     -8.19 μs(-1.08%)  -311.30 μs(-33.04%)    -1.56 ms(-81.90%)  -38.17 ms(-1088.79%)
rampUp  fetchVulnerabilities  -54(-1.82%)    -29.42 μs(-6.06%)    -10.24 μs(-1.96%)     -8.19 μs(-1.34%)    -53.25 μs(-7.30%)  -761.86 μs(-45.81%)   -33.75 ms(-412.00%)
```
