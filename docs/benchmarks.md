# Benchmarks

## ENT1-M Scaleway

Benchmarks done with two ENT1-M from scaleway where one execute the datastore
and the other execute `memtier_benchmark`.

- 16 vCPUS
- RAM: 64G
- BW: 3,2 Gbps 

### Redis
```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 0:1
```

```
ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec
----------------------------------------------------------------------------------------------------------------------------
Sets            0.00          ---          ---             ---             ---             ---             ---         0.00
Gets       124852.63         0.00    124852.63         2.64966         2.52700         5.56700         9.15100      4863.52
Waits           0.00          ---          ---             ---             ---             ---             ---          ---
Totals     124852.63         0.00    124852.63         2.64966         2.52700         5.56700         9.15100      4863.52
```


```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 1:0 
```

ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec
----------------------------------------------------------------------------------------------------------------------------
Sets       119409.00          ---          ---         2.67856         2.59100         6.07900         8.95900     35436.60
Gets            0.00         0.00         0.00             ---             ---             ---             ---         0.00
Waits           0.00          ---          ---             ---             ---             ---             ---          ---
Totals     119409.00         0.00         0.00         2.67856         2.59100         6.07900         8.95900     35436.60


```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 1:1 
```

```
ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec
----------------------------------------------------------------------------------------------------------------------------
Sets        61471.96          ---          ---         2.60189         2.52700         5.53500        10.49500     18242.84
Gets        61466.29     60235.66      1230.63         2.60108         2.52700         5.56700        10.55900     17629.74
Waits           0.00          ---          ---             ---             ---             ---             ---          ---
Totals     122938.25     60235.66      1230.63         2.60149         2.52700         5.53500        10.49500     35872.57


```

### Dragonfly

```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 0:1
```

```
ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec
----------------------------------------------------------------------------------------------------------------------------
Sets            0.00          ---          ---             ---             ---             ---             ---         0.00
Gets       119064.95         0.00    119064.95         2.68611         2.55900         7.74300        19.19900      4638.07 
Waits           0.00          ---          ---             ---             ---             ---             ---          --- 
Totals     119064.95         0.00    119064.95         2.68611         2.55900         7.74300        19.19900      4638.07 
```

```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 1:0
```

```
ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec
----------------------------------------------------------------------------------------------------------------------------
Sets       111018.53          ---          ---         2.88085         2.68700         6.97500        18.04700     32946.59
Gets            0.00         0.00         0.00             ---             ---             ---             ---         0.00
Waits           0.00          ---          ---             ---             ---             ---             ---          ---
Totals     111018.53         0.00         0.00         2.88085         2.68700         6.97500        18.04700     32946.59
```

```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 1:1
```

```
ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec 
----------------------------------------------------------------------------------------------------------------------------
Sets        57413.22          ---          ---         2.78828         2.54300         9.85500        22.14300     17038.33 
Gets        57407.58      4673.40     52734.18         2.78248         2.54300         9.79100        21.63100      3418.29 
Waits           0.00          ---          ---             ---             ---             ---             ---          --- 
Totals     114820.80      4673.40     52734.18         2.78538         2.54300         9.85500        21.88700     20456.63 
```

### Roster

```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 0:1
```

```
ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec
----------------------------------------------------------------------------------------------------------------------------
Sets            0.00          ---          ---             ---             ---             ---             ---         0.00
Gets       122546.70         0.00    122546.70         2.60989         2.44700         4.83100        11.96700      4773.69
Waits           0.00          ---          ---             ---             ---             ---             ---          ---
Totals     122546.70         0.00    122546.70         2.60989         2.44700         4.83100        11.96700      4773.69
```

```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 1:0
```

```
ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec
----------------------------------------------------------------------------------------------------------------------------
Sets       116508.45          ---          ---         2.74520         2.57500         6.68700        15.48700     34575.81 
Gets            0.00         0.00         0.00             ---             ---             ---             ---         0.00 
Waits           0.00          ---          ---             ---             ---             ---             ---          --- 
Totals     116508.45         0.00         0.00         2.74520         2.57500         6.68700        15.48700     34575.81 
```

```
memtier_benchmark  -c 20 --test-time 30 -t 16 -d 256 --distinct-client-seed --ratio 1:1
```

```
ALL STATS
============================================================================================================================
Type         Ops/sec     Hits/sec   Misses/sec    Avg. Latency     p50 Latency     p99 Latency   p99.9 Latency       KB/sec 
----------------------------------------------------------------------------------------------------------------------------
Sets        60154.39          ---          ---         2.66261         2.52700         5.59900        14.46300     17851.82 
Gets        60148.96       357.80     59791.15         2.65457         2.52700         5.53500        13.69500      2433.54 
Waits           0.00          ---          ---             ---             ---             ---             ---          --- 
Totals     120303.35       357.80     59791.15         2.65859         2.52700         5.56700        13.82300     20285.36 
```

