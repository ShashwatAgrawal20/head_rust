use std::error::Error;
use std::fs;

pub struct ArgsVal {
    pub filep: String,
    pub number_of_lines: usize,
}

impl ArgsVal {
    pub fn new(args: &[String]) -> Result<ArgsVal, &'static str> {
        if args.len() < 2 {
            return Err("Needs 1 arguments");
        }
        let filep = args[1].clone();
        let number_of_lines = args
            .get(2)
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap_or(10 as usize);

        Ok(ArgsVal {
            filep,
            number_of_lines,
        })
    }
}

pub fn run(args: ArgsVal) -> Result<(), Box<dyn Error>> {
    let f_contents = fs::read_to_string(args.filep)?;

    for line in f_contents.lines().take(args.number_of_lines) {
        println!("{}", line);
    }

    Ok(())
}
