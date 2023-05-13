trait Application {
    pub fn start(&self) -> Result<(), ()>;
    pub fn update(&self, dt: f32);
    pub fn render(&self);
    pub fn exit(&self);
}

trait WebClient {
    pub fn init(&Self) -> Result<(), ()>;
}