impl CommunicationComponent {
    pub fn check_frame(
        &self, 
        frame: CANFrame
    ) {
    if self.my_IDs[..self.my_IDs_count].contains(&frame.id) {
            // defense mode: ON
        }
    }
}