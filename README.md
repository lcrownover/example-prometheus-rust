# Example Prometheus Exporter in Rust

In prometheus, you have 4 important pieces of information that make up a metric:

1. metric name

2. metric help text

3. metric label(s)

4. metric value

The easiest way I can think of it is that the metric name and value are the
descriptive name and value of the metric you're trying to measure, whereas the
labels are like tags where you can filter based on them. Many systems will probably
report the same metric name with different values, but the labels for each system
should be different to allow separation.

