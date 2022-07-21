use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg1 = args.get(1);

    match arg1 {
        Some(arg) => {
            if arg.eq("test7") {
                skyscrapers::run_test_7_program();
            } else {
                skyscrapers::run_program(arg.parse().unwrap());
            }
        }
        None => {
            println!("provide a number N to specify the NxN grid of skyscrapers!");
            return;
        }
    }
}