use std::env;

#[derive(Debug)]
pub enum ArgType {
    NoneProvided,
    PersonSearch(String),
    PersonCompare((String, String)),
    PersonChain(Vec<String>),
}

impl ArgType {
    pub fn from_env() -> ArgType {
        let args: Vec<String> = env::args().collect();

        match args.len() {
            // 0 shouldn't happen, since that's the location
            // of the program being executed.
            0 | 1 => ArgType::NoneProvided,
            2 => {
                let person = &args[1];

                ArgType::PersonSearch(person.clone())
            }
            3 => {
                let first = &args[1];
                let second = &args[2];

                ArgType::PersonCompare((first.clone(), second.clone()))
            }
            _ => ArgType::PersonChain(args),
        }
    }
}
