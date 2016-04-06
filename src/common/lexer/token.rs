use uuid::Uuid;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Token {
    id: Uuid,
    ident: String,
    content: String,
    fileloc: (String, i32, i32)
}
