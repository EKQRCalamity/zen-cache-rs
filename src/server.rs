use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::Display,
    fs::File,
    io::{prelude::*, ErrorKind, Read},
    net::{SocketAddr, TcpListener, TcpStream},
    path::Path,
    sync::{Arc, Mutex, RwLock},
    time::{Duration, Instant},
};

use crate::cache::Cache;

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros() % 1000;
    let nanos = duration.subsec_nanos() % 1_000_000;

    if secs > 0 {
        format!("{}.{:07}s", secs, millis)
    } else if millis > 0 {
        format!("{}.{}ms", millis, micros)
    } else if micros > 0 {
        format!("{}.{}μs", micros, nanos)
    } else {
        format!("{}ns", nanos)
    }
}

fn get_mime_type(file_extension: &OsStr) -> &'static str {
    match file_extension.to_os_string().to_str().unwrap() {
        "aac" => "audio/aac",
        "abw" => "application/x-abiword",
        "apng" => "image/apng",
        "arc" => "application/x-freearc",
        "avif" => "image/avif",
        "avi" => "video/x-msvideo",
        "azw" => "application/vnd.amazon.ebook",
        "bin" => "application/octet-stream",
        "bmp" => "image/bmp",
        "bz" => "application/x-bzip",
        "bz2" => "application/x-bzip2",
        "cda" => "application/x-cdf",
        "csh" => "application/x-csh",
        "css" => "text/css",
        "csv" => "text/csv",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "eot" => "application/vnd.ms-fontobject",
        "epub" => "application/epub+zip",
        "gz" => "application/gzip",
        "gif" => "image/gif",
        "html" | "htm" => "text/html",
        "ico" => "image/vnd.microsoft.icon",
        "ics" => "text/calendar",
        "jar" => "application/java-archive",
        "jpeg" | "jpg" => "image/jpeg",
        "js" => "text/javascript",
        "json" => "application/json",
        "jsonld" => "application/ld+json",
        "mid" | "midi" => "audio/midi",
        "mjs" => "text/javascript",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "mpeg" => "video/mpeg",
        "mpkg" => "application/vnd.apple.installer+xml",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "odt" => "application/vnd.oasis.opendocument.text",
        "oga" => "audio/ogg",
        "ogv" => "video/ogg",
        "ogx" => "application/ogg",
        "opus" => "audio/opus",
        "otf" => "font/otf",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "php" => "application/x-httpd-php",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "rar" => "application/vnd.rar",
        "rtf" => "application/rtf",
        "sh" => "application/x-sh",
        "svg" => "image/svg+xml",
        "tar" => "application/x-tar",
        "tif" | "tiff" => "image/tiff",
        "ts" => "video/mp2t",
        "ttf" => "font/ttf",
        "txt" => "text/plain",
        "vsd" => "application/vnd.visio",
        "wav" => "audio/wav",
        "weba" => "audio/webm",
        "webm" => "video/webm",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "xhtml" => "application/xhtml+xml",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xml" => "application/xml",
        "xul" => "application/vnd.mozilla.xul+xml",
        "zip" => "application/zip",
        "3gp" => "video/3gpp",
        "3g2" => "video/3gpp2",
        "7z" => "application/x-7z-compressed",
        _ => "application/octet-stream",
    }
}

struct Helper {}

impl Helper {
    pub fn display_list(list: &Vec<Header>) -> String {
        let mut out = String::new();
        for i in 0..list.len() {
            out.push_str(format!("{}", list[i]).as_str());
            out.push_str("\n");
        }
        out
    }
}

#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

impl Header {
    pub fn new(key: &str, value: &str) -> Header {
        Header {
            key: key.trim().to_owned(),
            value: value.trim().to_owned(),
        }
    }
}

pub struct HTMLRequest {
    pub method: String,
    pub endpoint: String,
    pub version: String,
    pub header: Vec<Header>,
    pub body: String,
    pub client_address: SocketAddr,
    pub local_address: SocketAddr,
    pub stream: Mutex<TcpStream>,
}

#[allow(dead_code)]
impl HTMLRequest {
    pub fn from_requeststr(requestin: String, stream: TcpStream) -> HTMLRequest {
        let socketaddr = match stream.peer_addr() {
            Ok(x) => x,
            Err(_) => {
                panic!()
            }
        };
        let localaddr = match stream.local_addr() {
            Ok(x) => x,
            Err(_) => {
                panic!()
            }
        };
        let method: String;
        let endpoint: String;
        let version: String;
        let mut headers: Vec<Header> = vec![];
        let body: String;

        let strings: Vec<String> = requestin
            .trim()
            .split("\r\n\r\n")
            .map(|x| x.trim_end_matches("\0").to_owned())
            .collect();

        body = strings[1].to_owned();
        let headercontent: Vec<String> = strings[0].split("\n").map(|x| x.to_string()).collect();
        let headcon = headercontent[0]
            .as_str()
            .to_string()
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        method = headcon[0].as_str().to_string();
        endpoint = headcon[1].as_str().to_string();
        version = headcon[2].as_str().to_string();

        for i in 1..headercontent.len() {
            let header = headercontent[i]
                .as_str()
                .to_string()
                .split(": ")
                .map(|x| x.trim_end_matches("\r").to_string())
                .collect::<Vec<String>>();
            headers.push(Header::new(header[0].as_str(), header[1].as_str()));
        }

        HTMLRequest {
            method: method,
            endpoint: endpoint,
            version: version,
            header: headers,
            body: body,
            client_address: socketaddr,
            local_address: localaddr,
            stream: Mutex::new(stream),
        }
    }

    pub fn respond(&self, response: u64) {
        let mut stream = self.stream.lock().unwrap();
        let response = format!("HTTP/1.1 {}\r\n\r\n", response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn respond_with_body(&self, response: u64, body: String) {
        let mut stream = self.stream.lock().unwrap();
        let content_type = "Content-Type: text/plain"; // Example content type, can be changed as needed
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n{}\r\n\r\n{}",
            response,
            body.as_bytes().len(),
            content_type,
            body
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn respond_with_file(&self, file_path: &str) {
        let mut stream = self.stream.lock().unwrap();

        let path = Path::new(file_path);
        if path.is_file() {
            let mut file = match File::open(path) {
                Ok(x) => x,
                Err(e) => {
                    self.respond_with_body(500, format!("Error opening file: {}", e));
                    return;
                }
            };

            // Send HTTP response headers
            let content_length = file.metadata().expect("Error getting content length").len();
            let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\nContent-Disposition: attachment; filename=\"{}\"\r\n\r\n",
            content_length,
            get_mime_type(path.extension().unwrap()),
            path.file_name().unwrap().to_string_lossy()
        );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();

            // Send file content
            let mut buffer = [0; 1024];
            loop {
                match file.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            break; // End of file
                        }
                        stream.write_all(&buffer[..bytes_read]).unwrap();
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::Interrupted {
                            continue; // Retry on interrupt
                        }
                        self.respond_with_body(500, format!("Error reading file: {}", e));
                    }
                }
            }
            stream.flush().unwrap();
        } else {
            // File not found
            let response = format!(
                "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                404,
                "File not found".as_bytes().len(),
                "File not found"
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    pub fn get_header(&self, key: &str) -> Option<&Header> {
        for i in 0..self.header.len() {
            if self.header[i].key == key {
                return Some(&self.header[i]);
            }
        }
        None
    }

    pub fn display(&self) -> String {
        format!(
            "Method: {}\nEndpoint: {}\nVersion: {}\nHeaders: {}Body: {}",
            self.method,
            self.endpoint,
            self.version,
            Helper::display_list(&self.header),
            self.body
        )
    }
}

pub struct HTTPServer {
    port: std::borrow::Cow<'static, str>,
    host: std::borrow::Cow<'static, str>,
}

#[allow(dead_code)]
impl HTTPServer {
    pub fn from_default() -> HTTPServer {
        HTTPServer {
            port: std::borrow::Cow::Owned("8080".to_owned()),
            host: std::borrow::Cow::Owned("localhost".to_owned()),
        }
    }

    pub fn new(host: Option<&str>, port: Option<&str>) -> HTTPServer {
        HTTPServer {
            port: std::borrow::Cow::Owned(port.unwrap_or("8080").to_owned()),
            host: std::borrow::Cow::Owned(host.unwrap_or("8080").to_owned()),
        }
    }

    fn interpret_stream(mut stream: TcpStream) -> Result<HTMLRequest, std::io::Error> {
        let mut buffer = [0; 512];

        let _ = stream.read(&mut buffer);

        let content = String::from_utf8_lossy(&buffer[..]).to_string();

        if !content.trim().is_empty() && content.len() > 0 && content.lines().count() > 1 {
            let request = HTMLRequest::from_requeststr(content, stream);
            Ok(request)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Request is empty.",
            ))
        }
    }

    pub fn listen(
        &mut self,
        fnmap: HashMap<String, Arc<crate::handler::Function>>,
        cache: Cache,
    ) -> Result<&HTTPServer, std::io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", &self.host, &self.port))
            .expect("An Error occured while registering the TCP Listener!");

        let cache: Arc<RwLock<Cache>> = Arc::new(
            RwLock::new(
                cache
            )
        );

        let map: Arc<RwLock<HashMap<String, Arc<crate::handler::Function>>>> =
            Arc::new(RwLock::new(fnmap));

        println!("Now listening on {} with port {}", &self.host, &self.port);

        for stream in listener.incoming() {
            let start = Instant::now();
            let stream: TcpStream = stream.unwrap();
            let arc_clone = Arc::clone(&map);
            let cache_clone = Arc::clone(&cache);
            std::thread::spawn(move || {
                let request = match HTTPServer::interpret_stream(stream) {
                    Ok(req) => req,
                    Err(err) => {
                        eprintln!("Error occurred while interpreting the stream: \n {} ", err);
                        return;
                    }
                };

                if let Some(func) = arc_clone.write().unwrap().get(&request.endpoint.to_owned()) {
                    let methods = match &func.methods {
                        Some(methods) => methods.clone(),
                        None => vec![
                            String::from("GET"),
                            String::from("HEAD"),
                            String::from("POST"),
                            String::from("PUT"),
                            String::from("DELETE"),
                            String::from("CONNECT"),
                            String::from("OPTIONS"),
                            String::from("TRACE"),
                            String::from("PATCH"),
                        ],
                    };
                    // Check if the method is viable for the function
                    if methods.contains(&request.method) {
                        // Execute the Fn(Request) method
                        // function is a property containing the Arc<dyn Fn(request)> function,
                        match func.function.as_ref()(&request, &mut cache_clone.write().unwrap()) {
                            Ok(_msg) =>
                                /*println!("Got message: {}", _msg)*/
                                {}
                            Err(err) => eprintln!("Error occurred: {}", err),
                        }
                    } else {
                        request.respond_with_body(
                            405,
                            format!(
                                "Unsupported method. Supported methods: {}",
                                methods.join(", ")
                            ),
                        );
                    }
                    println!(
                        "Received request from {} on local {}{} - {}",
                        request.client_address.to_string(),
                        request.local_address.to_string(),
                        request.endpoint.to_owned(),
                        format_duration(start.elapsed())
                    );
                } else {
                    request.respond(404);
                }
            });
        }
        Ok(self)
    }
}
