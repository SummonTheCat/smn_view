pub enum Mime {
    TextPlain,
    TextHtml,
    ApplicationJson,
    CSS,
    JavaScript,
}

impl Mime {
    pub fn to_string(&self) -> &str {
        match self {
            Mime::TextPlain => "text/plain",
            Mime::TextHtml => "text/html",
            Mime::ApplicationJson => "application/json",
            Mime::CSS => "text/css",
            Mime::JavaScript => "application/javascript",
        }
    }

    pub fn from_string(mime: &str) -> Self {
        match mime {
            "text/plain" => Mime::TextPlain,
            "text/html" => Mime::TextHtml,
            "application/json" => Mime::ApplicationJson,
            "text/css" => Mime::CSS,
            "application/javascript" => Mime::JavaScript,
            _ => Mime::TextPlain,
        }
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "txt" => Mime::TextPlain,
            "html" => Mime::TextHtml,
            "json" => Mime::ApplicationJson,
            "css" => Mime::CSS,
            "js" => Mime::JavaScript,
            _ => Mime::TextPlain,
        }
    }
}