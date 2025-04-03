pub struct Features {
    pub undetectable: bool,
    pub ban_free: bool,
    pub customizable: bool,
}

impl Features {
    pub fn new() -> Self {
        Features {
            undetectable: true,
            ban_free: true,
            customizable: true,
        }
    }

    pub fn toggle_feature(&mut self, feature: &str) {
        match feature {
            "undetectable" => self.undetectable = !self.undetectable,
            "ban_free" => self.ban_free = !self.ban_free,
            "customizable" => self.customizable = !self.customizable,
            _ => {}
        }
    }
}