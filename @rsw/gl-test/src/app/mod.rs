use web_sys::WebGlRenderingContext;

pub trait Application {
    fn start(&mut self) -> Result<(), ()>;
    fn update(&mut self, dt: f32);
    fn render(&self);
    fn exit(&self);
}
