pub trait AppListener {
    fn on_receive(&mut self, frame: &[u8]);
}