pub enum Auth {
    Basic(String, String),
    Bearer(String),
}
