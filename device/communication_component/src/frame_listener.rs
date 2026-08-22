pub trait FrameListener {
    fn on_interesting_frame(&mut self, frame: &[u8]);
}