use std::env;
use dotenv::dotenv;

pub struct Config{
    pub port: u16,
    pub host:String
}

pub fn read_config()-> Config{
    dotenv().ok();

    Config{
        port: env::var("PORT").expect("No port found").parse::<u16>().expect("Incorrect port number"),
        host: env::var("HOST").expect("No host found")
    }
}
