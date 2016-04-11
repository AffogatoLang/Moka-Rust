use uuid::Uuid;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Token {
    ident: String,
    content: String,
    fileloc: (String, u32, u32)
}
