
//enum Frame {
//    Data(),
//
//}

enum Method {
    Get,
    Put,
    Post,
    Delete,
    Option,
}
impl Method {
    fn from_string(s : &str) -> Option<Method> {

        match s.to_lowercase().as_str() {
            "get" => Some(Method::Get),
            "put" => Some(Method::Put),
            "post" => Some(Method::Post),
            "delete" => Some(Method::Delete),
            "option" => Some(Method::Option),
            _ => None,
        }
//        if s.to_lowercase() == "get" { Some(Method::Get)}
//        else {None}
    }
}
enum Protocol {
    _11,
    _2
}

struct Request {
    method: Method,
    path: String,
    protocol: Protocol,

}

impl Request {

    fn from_string(s: &str) -> Result<Request, &'static str> {
        let mut token = s.split(' ');
        let method = token.next().and_then(|s| Method::from_string(s));

        Err("not implemented")
    }

}

fn main() {
    println!("Hello, world!");
}
