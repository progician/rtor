use googletest::prelude::*;
use rstest::*;
use rtor::peers::Peer;

#[rstest]
fn test_peers_can_connect() {
    let mut peer1 = Peer::new();
    let mut peer2 = Peer::new();
    peer2.connect_to(&mut peer1);
    assert_that!(peer1.connections(), contains(eq(&peer2)));
}
