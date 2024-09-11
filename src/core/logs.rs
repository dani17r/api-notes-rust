fn init_logging() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
}

fn info_server() {
    log::info!("Server running at 🖥️ http://localhost:8080 💻");
}

pub fn main() {
    init_logging();
    info_server();   
}