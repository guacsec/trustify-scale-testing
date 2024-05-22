# Basic benchmarks with hyperfoil (in-vm) and Trustification (stable - running locally)

* Clone the repository
* Switch to stable branch `git checkout stable`
* Open a terminal and run [stable containerized version](https://github.com/trustification/trustification/tree/stable?tab=readme-ov-file#running-locally)
* Open other terminal and run hyperfoil:

```shell
cd benchmarks/trustification
./get-hf.sh
cd hyperfoil*/
```

> [!NOTE]
> Change from `-Djava.net.preferIPv4Stack=true` to `-Djava.net.preferIPv4Stack=false` in `hyperfoil*/bin/parse-opts.sh` see: <https://github.com/Hyperfoil/Hyperfoil/issues/346>

Run hyperfoil

```shell
./bin/cli.sh
```

Run the following commands:

```shell
start-local
upload ../trustification-hf.yml
run
stats
```

We can see something like:

```shell
[hyperfoil]$ start-local
Starting controller in default directory (/tmp/hyperfoil)
Controller started, listening on 127.0.0.1:36171
Connecting to the controller...
Connected to 127.0.0.1:36171!
[hyperfoil@in-vm]$ upload ../trustification-hf.yml
Loaded benchmark trustification, uploading...
... done.
[hyperfoil@in-vm]$ run
Started run 0000
Run 0000, benchmark trustification
Agents: in-vm[STARTING]
Agents: in-vm[REGISTERED]
Agents: in-vm[READY]
Agents: in-vm[STOPPED]
Started: 2024/05/22 14:57:20.576    Terminated: 2024/05/22 14:58:20.595
NAME    STATUS      STARTED       REMAINING  COMPLETED     TOTAL DURATION                DESCRIPTION
basic   TERMINATED  14:57:20.576             14:57:20.598  22 ms (exceeded by 23 ms)     1 users at once
rampUp  TERMINATED  14:57:20.576             14:58:20.595  60019 ms (exceeded by 19 ms)  1.00 - 100.00 users per second
[hyperfoil@in-vm]$ stats
Total stats from run 0000
PHASE   METRIC               THROUGHPUT   REQUESTS  MEAN       p50        p90        p99        p99.9      p99.99     TIMEOUTS  ERRORS  BLOCKED  2xx   3xx  4xx
                             5xx          CACHE
----------------------------------------------------------------------------------------------------------------------------------------------------------------
basic   fetchAdvisorySearch  45.45 req/s         1    1.03 ms    1.03 ms    1.03 ms    1.03 ms    1.03 ms    1.03 ms         0       0     0 ns     1    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
basic   fetchCveSearch       45.45 req/s         1  845.82 μs  847.87 μs  847.87 μs  847.87 μs  847.87 μs  847.87 μs         0       0     0 ns     1    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
basic   fetchPackagesSearch  45.45 req/s         1  858.11 μs  860.16 μs  860.16 μs  860.16 μs  860.16 μs  860.16 μs         0       0     0 ns     1    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
basic   fetchSbomSearch      45.45 req/s         1  956.42 μs  958.46 μs  958.46 μs  958.46 μs  958.46 μs  958.46 μs         0       0     0 ns     1    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
basic   successfulLogin      45.45 req/s         1    9.86 ms    9.90 ms    9.90 ms    9.90 ms    9.90 ms    9.90 ms         0       0     0 ns     1    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
rampUp  fetchAdvisorySearch  50.48 req/s      3032  766.15 μs  831.49 μs    1.02 ms    1.61 ms    3.44 ms    9.24 ms         0       2     0 ns  3030    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
rampUp  fetchCveSearch       50.45 req/s      3028  711.88 μs  778.24 μs  987.14 μs    1.70 ms    2.83 ms    8.26 ms         0       0     0 ns  3028    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
rampUp  fetchPackagesSearch  50.43 req/s      3028  698.65 μs  770.05 μs  974.85 μs    1.61 ms    3.13 ms    4.33 ms         0       1     0 ns  3027    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
rampUp  fetchSbomSearch      50.45 req/s      3030  734.06 μs  798.72 μs  995.33 μs    1.74 ms    4.26 ms    6.52 ms         0       2     0 ns  3028    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
rampUp  successfulLogin      50.52 req/s      3032  922.16 μs  978.94 μs    1.25 ms    2.13 ms    4.26 ms    8.19 ms         0       0     0 ns  3032    0    0
                                       0         0
----------------------------------------------------------------------------------------------------------------------------------------------------------------
[hyperfoil@in-vm]$
```

Now run the `report` command:

```shell
[hyperfoil@in-vm]$ report --destination=/whatever_dir_here
```

A browser will pop up and show this:

(you can visualize more options on browser report)

![01](img/1.png)

![02](img/2.png)
