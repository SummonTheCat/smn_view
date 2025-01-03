use wry::application::dpi::LogicalSize;

pub struct WindowConfig {
    pub title: String,
    pub size: LogicalSize<f64>,
    pub url: String,
}

impl WindowConfig {
    pub fn default() -> Self {
        Self {
            title: "SmnView".to_string(),
            size: LogicalSize::new(800.0, 600.0),
            url: "http://127.0.0.1:3030/".to_string(),
        }
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn set_size(mut self, width: f64, height: f64) -> Self {
        self.size = LogicalSize::new(width, height);
        self
    }

    pub fn set_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }
}

