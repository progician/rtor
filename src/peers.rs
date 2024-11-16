use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: Uuid,
    pub address: String,
    pub port: u16,

    connected: Vec<Peer>,
}


impl Peer {
    pub fn new() -> Self {
        Peer {
            id: Uuid::new_v4(),
            address: "0.0.0.0".to_string(),
            port: 0,

            connected: vec!(),
        }
    }

    pub fn connect_to(&mut self, to: &mut Peer) {
        to.connected.push(self.clone());
    }

    pub fn connections(&self) -> &[Peer] {
        &self.connected
    }
}


impl PartialEq for Peer {
    fn eq(&self, rhs: &Peer) -> bool {
        self.id == rhs.id
    }
}
