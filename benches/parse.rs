#[macro_use]
extern crate bencher;
extern crate rsipfix;

use bencher::{black_box, Bencher};
use rsipfix::{parser, state};

fn parse_data_with_template(bench: &mut Bencher) {
    // contains templates 500, 999, 501
    let template_bytes = include_bytes!("../tests/parse_temp.bin");

    // contains data sets for templates 999, 500, 999
    let data_bytes = include_bytes!("../tests/parse_data.bin");

    let mut s = state::State::new();
    let p = parser::Parser::new();

    // parse the template so parsing data can be done
    assert!(p.parse_message(&mut s, template_bytes).is_ok());

    bench.iter(|| {
        let _ = p.parse_message(&mut s, black_box(data_bytes)).unwrap();
    })
}

fn parse_template(bench: &mut Bencher) {
    // contains templates 500, 999, 501
    let template_bytes = include_bytes!("../tests/parse_temp.bin");

    let mut s = state::State::new();
    let p = parser::Parser::new();

    // parse the template so parsing data can be done
    bench.iter(|| {
        let _ = p.parse_message(&mut s, black_box(template_bytes)).unwrap();
    })
}

benchmark_group!(benches, parse_template, parse_data_with_template);
benchmark_main!(benches);
