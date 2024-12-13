// Testing that the test is correctly seted up XD
use crate::test::{TrustlessManagerTest};

#[test]
fn test_setup() {
    let test = TrustlessManagerTest::setup();

    let factory_admin = test.defindex_factory.admin();
    let factory_defindex_receiver = test.defindex_factory.defindex_receiver();
  
    assert_eq!(factory_admin, test.admin);
    assert_eq!(factory_defindex_receiver, test.defindex_receiver);

    // Mint tokens to user
    let amount = 987654321;
    test.token_0_admin_client.mint(&test.user, &amount);
    test.token_1_admin_client.mint(&test.user, &amount);

    // check balances
    let balance_0 = test.token_0.balance(&test.user);
    let balance_1 = test.token_1.balance(&test.user);
    assert_eq!(balance_0, amount);
    assert_eq!(balance_1, amount);


}