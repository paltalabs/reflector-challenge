// Testing that the test is correctly seted up XD
use crate::test::{TrustlessManagerTest};

#[test]
fn test_setup() {
    let test = TrustlessManagerTest::setup();

    let factory_admin = test.defindex_factory.admin();
    let factory_defindex_receiver = test.defindex_factory.defindex_receiver();
  
    assert_eq!(factory_admin, test.admin);
    assert_eq!(factory_defindex_receiver, test.defindex_receiver);
}