use std::path::Path;
#[derive(Clone)]
pub struct Config {
    pub replicator: String,
    pub src: String,
    pub dst: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 3 {
            return Err("not enough arguments");
        }

        let replicator = args[1].clone();
        let src = args[2].clone();
        let dst = args[3].clone();

        Ok(Config {
            replicator,
            src,
            dst,
        })
    }

    pub fn validate(&self) {
        if !Path::new(&self.src).exists() {
            panic!("Source directory {} doesn't exist", self.src);
        }
        if Path::new(&self.dst).exists() {
            panic!("Destination directory {} already exists", self.dst);
        }

        println!("Replication multiplier {}", &self.replicator);
        println!("Source directory {}", &self.src);
        println!("Destination directory {}", &self.dst);
    }
}
