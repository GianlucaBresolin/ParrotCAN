pub struct CANFrame {
    pub id: u16, 
    pub rtr: bool, 
    pub dlc: u8, 
    pub data_low: u32, 
    pub data_high: u32
}