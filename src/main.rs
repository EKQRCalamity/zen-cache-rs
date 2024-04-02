use std::panic;

mod server;
mod handler;
mod arghelper;
mod cache;

use arghelper::ArgHelper;

fn main() {    
    let function_helper = handler::FuncHelper::new();
    let functionmap = function_helper.get_func_map_raw();

    let arghelper = ArgHelper::parse(std::env::args().map(|x| x.to_string()).collect());

    let cache = cache::Cache::new(
        String::from(
            std::env::current_dir().unwrap().to_string_lossy()
        )
    );

    // For now unused, it's for choosing between server methods
    // Planned: Websocket, HyperHttp, RocketHttp
    let _method = arghelper.get_value("method").unwrap_or(String::from("asynchttp"));
    let port = arghelper.get_value("port").unwrap_or(String::from("8080"));
    match _method.to_lowercase().as_str() {
        "asynchttp" => {
            server::HTTPServer::new(
                Some("0.0.0.0"), 
                Some(port.as_str())
            ).listen(functionmap, cache).unwrap();
        }
        _ => {
            panic!("Error. Specified method not found.")
        }
    }
}
