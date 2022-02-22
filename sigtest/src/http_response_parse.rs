use nom::bytes::complete::take_while1;
use nom::bytes::complete::*;
use nom::character::complete::crlf;
use nom::sequence::tuple;
use nom::sequence::terminated;
use nom::sequence::preceded;
use nom::character::is_digit;
use nom::IResult;
use nom::multi::many0;
use std::error::Error;
#[derive(Debug)]
pub struct StatusLine { //状态行
    pub status: u16, //状态码
    pub msg: String, //状态消息
}
#[derive(Debug)]
pub struct HttpResponse {
    pub status_line: StatusLine,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
impl HttpResponse {
    pub fn get_header(&self, name:&str) -> Option<&str> {
        self.headers.iter().find(|header| header.0.eq(name)).map(|v| v.1.as_ref())
    }
}
pub fn parse_status_line(input: &[u8]) -> Result<(&[u8], StatusLine), Box<dyn Error + '_>> {
    let http = tag("HTTP/");
    let version = tuple((take_while1(is_digit), tag("."), take_while1(is_digit)));
    let space = take_while1(|c| c == b' ');
    let status = take_while1(is_digit);
    let msg = terminated(is_not("\r\n".as_bytes()), tag(b"\r\n"));
//将以上匹配解析器组合为最终解析器，并并解析
    let res: IResult<&[u8], (&[u8], (&[u8], &[u8], &[u8]), &[u8], &[u8], &[u8])> = tuple((http, version, space, status, msg))(input);
    let res = res?;

    let status = res.1.3;
    let status = String::from_utf8_lossy(status).to_string();
    let status = status.parse::<u16>()?;
    Ok((res.0, StatusLine { status, msg: String::from_utf8_lossy(res.1.4).trim().to_string() }))
}
fn parse_response_header(input: &[u8]) -> IResult<&[u8], Vec<(String, String)>> {
    let name = terminated(is_not(":".as_bytes()), tag(":"));
    let value = terminated(is_not("\r\n".as_bytes()), tag(b"\r\n"));
    let kv = tuple((name, value));
    let headers: IResult<&[u8], Vec<(&[u8], &[u8])>> = many0(kv)(input);
    match headers {
        Ok(hs) => {
            let hs2 = hs.1.iter().map(|v| (String::from_utf8_lossy(v.0).trim().to_string(),
                                            String::from_utf8_lossy(v.1).trim().to_string()))
                .collect::<Vec<(String, String)>>();
            Ok((hs.0, hs2))
        }
        Err(e) => Err(e)
    }
}
fn parse_response_body(input: &[u8], len: usize) -> IResult<&[u8], &[u8]> {
    let body = take(len);
    preceded(crlf, body)(input)
}
pub fn parse_response(input: &[u8]) -> Result<HttpResponse, Box<dyn Error + '_>> { 
    let (input, status_line) = parse_status_line(input)?;
    let (input, headers) = parse_response_header(input)?;
    let content_length = headers.iter().find(|kv| kv.0.eq("Content-Length"))
        .map(|kv| kv.1.parse::<usize>().unwrap_or(0)).unwrap_or(0);
    let (_input, body) = parse_response_body(input, content_length)?;
    Ok(HttpResponse {
        status_line,
        headers,
        body: Vec::from(body)
    })
}