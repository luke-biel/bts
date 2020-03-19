use std::str::FromStr;

#[derive(Default, Debug, Clone)]
pub struct TemplateName(String);

impl From<String> for TemplateName {
    fn from(s: String) -> Self {
        TemplateName(s)
    }
}

impl From<&str> for TemplateName {
    fn from(s: &str) -> Self {
        TemplateName(s.to_string())
    }
}

impl FromStr for TemplateName {
    type Err = String; // TODO: replace with never type (!) when its stable

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TemplateName(s.to_string()))
    }
}

impl TemplateName {
    #[cfg(windows)]
    pub fn normalized(&self) -> String {
        self.0.replace('/', "\\")
    }

    #[cfg(not(windows))]
    pub fn normalized(&self) -> String {
        self.0.clone()
    }
}
