pub enum Mime {
    TextPlain,
    TextHtml,
    ApplicationJson,   
}

impl Mime {
    pub fn to_string(&self) -> &str {
        match self {
            Mime::TextPlain => "text/plain",
            Mime::TextHtml => "text/html",
            Mime::ApplicationJson => "application/json",
        }
    }

    pub fn from_string(mime: &str) -> Self {
        match mime {
            "text/plain" => Mime::TextPlain,
            "text/html" => Mime::TextHtml,
            "application/json" => Mime::ApplicationJson,
            _ => Mime::TextPlain,
        }
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "txt" => Mime::TextPlain,
            "html" => Mime::TextHtml,
            "json" => Mime::ApplicationJson,
            _ => Mime::TextPlain,
        }
    }
}