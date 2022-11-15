pub type SocketStatus = bool;

pub struct Socket {
    description: String,
    power_consumption: f32,
    status: SocketStatus,
}

impl Socket {
    pub fn new(_description: String) -> Self {
        todo!()
    }

    pub fn get_description(&self) -> String {
        todo!()
    }

    pub fn enable(&mut self) {
        todo!()
    }

    pub fn disable(&mut self) {
        todo!()
    }

    pub fn get_current_power_consumption(&self) -> f32 {
        todo!()
    }
}
