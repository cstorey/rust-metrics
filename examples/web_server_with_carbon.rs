// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// An example of sending data to a Prometheus server with a local webserver

extern crate iron;
extern crate metrics;
extern crate histogram;
extern crate hyper;

use iron::prelude::*;
use iron::status;
use metrics::metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
use metrics::reporter::CarbonReporter;
use histogram::Histogram;
use std::thread;
use metrics::reporter::Reporter;

fn main() {
    println!("WebServer Starting");
    thread::spawn(|| {
        let m = StdMeter::new();
        m.mark(100);

        let c = StdCounter::new();
        c.inc();

        let g = StdGauge::new();
        g.set(10);

        let mut h = Histogram::configure()
            .max_value(100)
            .precision(1)
            .build()
            .unwrap();

        h.increment_by(1, 1).unwrap();

        println!("Starting carbon recorder at carbon_graphite:2003");
        let mut reporter =
                CarbonReporter::new("test", "carbon_graphite:2003", "asd.asdf",1024);
        let _ = reporter.add("meter1", Metric::Meter(m.clone()));
        let _ = reporter.add("counter1", Metric::Counter(c.clone()));
        let _ = reporter.add("gauge1", Metric::Gauge(g.clone()));
        let _ = reporter.add("histogram", Metric::Histogram(h));

        loop { c.inc() }
    });
    Iron::new(|_: &mut Request| Ok(Response::with(status::NotFound)))
        .http("0.0.0.0:3000")
        .unwrap();
    println!("WebServer Running");
}
