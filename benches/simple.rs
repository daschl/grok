#![feature(test)]

extern crate grok;
extern crate test;

use test::Bencher;
use grok::Grok;

#[bench]
fn bench_simple_pattern_match(b: &mut Bencher) {
    let mut grok = Grok::empty();
    grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");
    let pattern = grok.compile("%{USERNAME}", false).expect(
        "Error while compiling!",
    );

    b.iter(|| match pattern.match_against("user") {
        Some(found) => {
            test::black_box(&found);
        }
        None => (),
    });
}

#[bench]
fn bench_simple_pattern_no_match(b: &mut Bencher) {
    let mut grok = Grok::empty();
    grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");
    let pattern = grok.compile("%{USERNAME}", false).expect(
        "Error while compiling!",
    );

    b.iter(|| match pattern.match_against("$$$$") {
        Some(found) => {
            test::black_box(&found);
        }
        None => (),
    });
}

#[bench]
fn bench_simple_pattern_match_with_anchor(b: &mut Bencher) {
    let mut grok = Grok::empty();
    grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");
    let pattern = grok.compile("^%{USERNAME}$", false).expect(
        "Error while compiling!",
    );

    b.iter(|| match pattern.match_against("user") {
        Some(found) => {
            test::black_box(&found);
        }
        None => (),
    });
}

#[bench]
fn bench_simple_pattern_no_match_with_anchor(b: &mut Bencher) {
    let mut grok = Grok::empty();
    grok.insert_definition("USERNAME", r"[a-zA-Z0-9._-]+");
    let pattern = grok.compile("^%{USERNAME}$", false).expect(
        "Error while compiling!",
    );

    b.iter(|| match pattern.match_against("$$$$") {
        Some(found) => {
            test::black_box(&found);
        }
        None => (),
    });
}
