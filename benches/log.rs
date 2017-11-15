#![feature(test)]

extern crate grok;
extern crate test;

use test::Bencher;
use grok::Grok;

#[bench]
fn bench_log_match(b: &mut Bencher) {
    let msg = "2016-09-19T18:19:00 [8.8.8.8:prd] DEBUG this is an example log message";

    let mut grok = Grok::default();
    let pattern = grok.compile(r"%{TIMESTAMP_ISO8601:timestamp} \[%{IPV4:ip}:%{WORD:environment}\] %{LOGLEVEL:log_level} %{GREEDYDATA:message}", false)
        .expect("Error while compiling!");

    b.iter(|| match pattern.match_against(msg) {
        Some(found) => {
            test::black_box(&found);
        }
        None => (),
    });
}

#[bench]
fn bench_log_no_match(b: &mut Bencher) {
    let msg = "2016-09-19T18:19:00 [8.8.8.8:prd] DEBUG this is an example log message";

    let mut grok = Grok::default();
    let pattern = grok.compile(r"%{TIMESTAMP_ISO8601:timestamp} \[%{IPV4:ip};%{WORD:environment}\] %{LOGLEVEL:log_level} %{GREEDYDATA:message}", false)
        .expect("Error while compiling!");

    b.iter(|| match pattern.match_against(msg) {
        Some(found) => {
            test::black_box(&found);
        }
        None => (),
    });
}

#[bench]
fn bench_log_match_with_anchors(b: &mut Bencher) {
    let msg = "2016-09-19T18:19:00 [8.8.8.8:prd] DEBUG this is an example log message";

    let mut grok = Grok::default();
    let pattern = grok.compile(r"^%{TIMESTAMP_ISO8601:timestamp} \[%{IPV4:ip}:%{WORD:environment}\] %{LOGLEVEL:log_level} %{GREEDYDATA:message}$", false)
        .expect("Error while compiling!");

    b.iter(|| match pattern.match_against(msg) {
        Some(found) => {
            test::black_box(&found);
        }
        None => (),
    });
}

#[bench]
fn bench_log_no_match_with_anchors(b: &mut Bencher) {
    let msg = "2016-09-19T18:19:00 [8.8.8.8;prd] DEBUG this is an example log message";

    let mut grok = Grok::default();
    let pattern = grok.compile(r"^%{TIMESTAMP_ISO8601:timestamp} \[%{IPV4:ip}:%{WORD:environment}\] %{LOGLEVEL:log_level} %{GREEDYDATA:message}$", false)
        .expect("Error while compiling!");

    b.iter(|| match pattern.match_against(msg) {
        Some(found) => {
            test::black_box(&found);
        }
        None => (),
    });
}