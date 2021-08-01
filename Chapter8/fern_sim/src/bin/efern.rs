fn main() {
    let mut fern = fern_sim::Fern {
        size: 1.0,
        growth_rate: 0.001,
    };

    fern_sim::run_simulation(&mut fern, 1000);
    println!("final fern size = {:.5}", fern.size);
}
