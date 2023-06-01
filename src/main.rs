use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::Tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Head,
}

impl Method {
    fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Head => "HEAD",
        }
    }
    fn from_str(method: &str) -> Option<Method> {
        match method {
            "GET" => Some(Method::Get),
            "POST" => Some(Method::Post),
            "HEAD" => Some(Method::Head),
            _ => None,
        }
    }
}

type Path = String;

#[derive(Debug, PartialEq)]
pub enum HTTPVersion {
    ZeroPointNine,
    OnePointZero,
}

#[derive(Debug, PartialEq)]
pub struct RequestLine {
    pub method: Method,
    pub path: Path,
    pub version: HTTPVersion,
}

fn first_line(input: &str) -> IResult<&str, RequestLine> {
    let (input, matched_method) = alt((
        tag(Method::Get.as_str()),
        tag(Method::Post.as_str()),
        tag(Method::Head.as_str()),
    ))(input)?;

    Ok((
        input,
        RequestLine {
            method: Method::from_str(matched_method).unwrap(),
            path: "/index.html".to_string(),
            version: HTTPVersion::OnePointZero,
        },
    ))
}

fn main() {
    println!("{:?}", first_line("GET /index.html HTTP/1.0"));
}

#[test]
fn parse_http_methods() {
    assert_eq!(
        first_line("GET /index.html HTTP/1.0"),
        Ok((
            " /index.html HTTP/1.0",
            RequestLine {
                method: Method::Get,
                path: "/index.html".to_string(),
                version: HTTPVersion::OnePointZero
            }
        ))
    );
    assert_eq!(
        first_line("POST /index.html HTTP/1.0"),
        Ok((
            " /index.html HTTP/1.0",
            RequestLine {
                method: Method::Post,
                path: "/index.html".to_string(),
                version: HTTPVersion::OnePointZero
            }
        ))
    );
    assert_eq!(
        first_line("HEAD /index.html HTTP/1.0"),
        Ok((
            " /index.html HTTP/1.0",
            RequestLine {
                method: Method::Head,
                path: "/index.html".to_string(),
                version: HTTPVersion::OnePointZero
            }
        ))
    );
}
