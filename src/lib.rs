use near_sdk::test_utils::test_env::{alice, bob, carol};

use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{UnorderedMap},
    serde::{Deserialize, Serialize},
    serde_json, AccountId,
};
use std::{collections::HashMap};

mod big_decimal;

use crate::big_decimal::*;

type PairId = (AccountId, AccountId);

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
struct Order {
    account_id: AccountId,
    amount: BigDecimal,
    sell_token: AccountId,
    buy_token: AccountId,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct Contract {
    order_nonce: u64,
    orders: UnorderedMap<AccountId, HashMap<u64, Order>>,
    ref_orders: UnorderedMap<PairId, HashMap<u64, Order>>,
}

impl Contract {
    fn new() -> Self {
        Self {
            order_nonce: 0,
            orders: UnorderedMap::new(b"m"),
            ref_orders: UnorderedMap::new(b"m"),
        }
    }

    fn new_1() -> Self {
        let pair_id: PairId = ("usdt.near".parse().unwrap(), "wnear.near".parse().unwrap());

        let map = HashMap::from([
            (1, Order{ account_id: alice(), amount: BigDecimal::from(1), sell_token: pair_id.0.clone(), buy_token: pair_id.1.clone()}),
            (2, Order{ account_id: alice(), amount: BigDecimal::from(1), sell_token: pair_id.0.clone(), buy_token: pair_id.1.clone()}),
            (3, Order{ account_id: alice(), amount: BigDecimal::from(1), sell_token: pair_id.0.clone(), buy_token: pair_id.1.clone()}),
        ]);
        
        let mut orders = UnorderedMap::<AccountId, HashMap<u64, Order>>::new(b"m");
        orders.insert(&alice(), &map);

        let mut ref_orders = UnorderedMap::<PairId, HashMap<u64, Order>>::new(b"m");

        Self {
            order_nonce: 3,
            orders,
            ref_orders,
        }
    }

    fn add_order_from_string_1(&mut self, account_id: AccountId, order: String) {
        self.order_nonce += 1;
        let order_id = self.order_nonce;

        let order: Order = serde_json::from_str(order.clone().as_str()).unwrap();

        self.insert_order(&account_id, order.clone(), order_id);
        // self.insert_ref_order(
        //     &(order.sell_token.clone(), order.buy_token.clone()),
        //     order,
        //     order_id,
        // );
    }

    fn add_order_from_string_2(&mut self, account_id: AccountId, order: String) {
        self.order_nonce += 1;
        let order_id = self.order_nonce;

        let order: Order = serde_json::from_str(order.clone().as_str()).unwrap();

        // self.insert_order(&account_id, order.clone(), order_id);
        self.insert_ref_order(
            &(order.sell_token.clone(), order.buy_token.clone()),
            order,
            order_id,
        );
    }

    fn add_order_from_string_3(&mut self, account_id: AccountId, order: String) {
        self.order_nonce += 1;
        let order_id = self.order_nonce;

        let order: Order = serde_json::from_str(order.clone().as_str()).unwrap();

        self.insert_order(&account_id, order.clone(), order_id);
        self.insert_ref_order(
            &(order.sell_token.clone(), order.buy_token.clone()),
            order,
            order_id,
        );
    }
 
    fn insert_order(&mut self, account_id: &AccountId, order: Order, order_id: u64) {
        let mut get_orders = self.orders.get(account_id).unwrap_or_default();
        
        get_orders.insert(order_id, order);
        self.orders.insert(account_id, &get_orders);
    }

    fn insert_ref_order(&mut self, pair_id: &PairId, order: Order, order_id: u64) {
        let mut get_ref_orders = self.ref_orders.get(pair_id).unwrap_or_default(); 
        
        get_ref_orders.insert(order_id, order);
        self.ref_orders.insert(pair_id, &get_ref_orders);
    }

    fn view_orders(&self, account_id: &AccountId) -> Vec<(u64, Order)> {
        let mut orders = self
            .orders
            .get(account_id)
            .unwrap_or_default(); 
            
            let mut sort_orders = 
            orders.into_iter().collect::<Vec<(u64, Order)>>();
            sort_orders.sort_by(|a, b| a.0.cmp(&b.0));
            sort_orders
    
    }

    fn view_ref_orders(&self, pair_id: &PairId) -> Vec<(u64, Order)> {
        let mut ref_orders = self
            .ref_orders
            .get(pair_id)
            .unwrap()
            .into_iter()
            .collect::<Vec<(u64, Order)>>();
        ref_orders.sort_by(|a, b| a.0.cmp(&b.0));
        ref_orders
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::test_env::{alice, bob, carol};

    #[test]
    fn test_1() {
        let mut contract = Contract::new();

        println!("Number of users before- {}", contract.orders.len());
        println!("Number of pairs before - {}", contract.ref_orders.len());
        println!("-------------------");

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                let account_id = alice();
                contract.add_order_from_string_1(account_id, order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_1(bob(), order_bob.clone());
            } else {
                let order = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                let account_id = carol();
                contract.add_order_from_string_1(account_id, order.clone());
            }
        }

        println!("Number of users after- {}", contract.orders.len());
        println!("Number of pairs afters - {}", contract.ref_orders.len());
        println!("-------------------");

        println!("Number of orders from Alice - {}", contract.orders.get(&alice()).unwrap().len());
        println!("Number of orders from Bob - {}", contract.orders.get(&bob()).unwrap().len());
        println!("Number of orders from Carol - {}", contract.orders.get(&carol()).unwrap().len());
        println!("-------------------");

        println!("{:#?}", contract.view_orders(&alice()));
        println!("-------------------");
        println!("{:#?}", contract.view_orders(&bob()));
        println!("-------------------");
        println!("{:#?}", contract.view_orders(&carol()));
    }


    #[test]
    fn test_2() {
        let mut contract = Contract::new();

        let pair_id: PairId = ("usdt.near".parse().unwrap(), "wnear.near".parse().unwrap());

        println!("Number of users before- {}", contract.orders.len());
        println!("Number of pairs before - {}", contract.ref_orders.len());
        println!("-------------------");

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(alice(), order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(bob(), order_bob);
            } else {
                let order_carol = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(carol(), order_carol);
            }
        }

        println!("Number of users after- {}", contract.orders.len());
        println!("Number of pairs afters - {}", contract.ref_orders.len());
        println!("-------------------");

        println!("Number of orders from usdt.near|wnear.near - {}", contract.ref_orders.get(&pair_id.clone()).unwrap().len());
        println!("-------------------");

        println!("{:#?}", contract.view_ref_orders(&pair_id));
    }

    #[test]
    fn test_3() {
        let mut contract = Contract::new();

        let pair_id: PairId = ("usdt.near".parse().unwrap(), "wnear.near".parse().unwrap());

        println!("Number of users before- {}", contract.orders.len());
        println!("Number of pairs before - {}", contract.ref_orders.len());
        println!("-------------------");

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_3(alice(), order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_3(bob(), order_bob);
            } else {
                let order_carol = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_3(carol(), order_carol);
            }
        }

        println!("Number of users after- {}", contract.orders.len());
        println!("Number of pairs afters - {}", contract.ref_orders.len());
        println!("-------------------");

        println!("Number of orders from Alice - {}", contract.orders.get(&alice()).unwrap().len());
        println!("Number of orders from Bob - {}", contract.orders.get(&bob()).unwrap().len());
        println!("Number of orders from Carol - {}", contract.orders.get(&carol()).unwrap().len());
        println!("-------------------");

        println!("Number of orders from usdt.near|wnear.near - {}", contract.ref_orders.get(&pair_id.clone()).unwrap().len());
        println!("-------------------");

        println!("{:#?}", contract.view_orders(&alice()));
        println!("-------------------");
        println!("{:#?}", contract.view_orders(&bob()));
        println!("-------------------");
        println!("{:#?}", contract.view_orders(&carol()));
        println!("-------------------"); 

        println!("{:#?}", contract.view_ref_orders(&pair_id));
    }

    #[test]
    fn test_4() {
        let mut contract = Contract::new_1();

        let pair_id: PairId = ("usdt.near".parse().unwrap(), "wnear.near".parse().unwrap());

        println!("Number of users before- {}", contract.orders.len());
        println!("Number of pairs before - {}", contract.ref_orders.len());
        println!("-------------------");

        println!("Number of orders from Alice - {}", contract.orders.get(&alice()).unwrap().len());
        println!("-------------------");

        println!("{:#?}", contract.view_orders(&alice()));
        println!("-------------------");



        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(alice(), order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(bob(), order_bob);
            } else {
                let order_carol = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(carol(), order_carol);
            }
        }

        println!("Number of users after- {}", contract.orders.len());
        println!("Number of pairs afters - {}", contract.ref_orders.len());
        println!("-------------------");

        println!("Number of orders from Alice - {}", contract.orders.get(&alice()).unwrap().len());
        println!("-------------------");

        println!("Number of orders from usdt.near|wnear.near - {}", contract.ref_orders.get(&pair_id.clone()).unwrap().len());
        println!("-------------------");

        println!("{:#?}", contract.view_orders(&alice()));
        println!("-------------------");

        println!("{:#?}", contract.view_ref_orders(&pair_id));
    }

    #[test]
    fn test_5() {
        let mut contract = Contract::new();

        println!("Number of users before- {}", contract.orders.len());
        println!("Number of pairs before - {}", contract.ref_orders.len());
        println!("-------------------");

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                let account_id = alice();
                contract.add_order_from_string_1(account_id, order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_1(bob(), order_bob.clone());
            } else {
                let order = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                let account_id = carol();
                contract.add_order_from_string_1(account_id, order.clone());
            }
        }

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_1(alice(), order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_1(bob(), order_bob);
            } else {
                let order_carol = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_1(carol(), order_carol);
            }
        }


        println!("Number of users after- {}", contract.orders.len());
        println!("Number of pairs afters - {}", contract.ref_orders.len());
        println!("-------------------");

        println!("Number of orders from Alice - {}", contract.orders.get(&alice()).unwrap().len());
        println!("Number of orders from Bob - {}", contract.orders.get(&bob()).unwrap().len());
        println!("Number of orders from Carol - {}", contract.orders.get(&carol()).unwrap().len());
        println!("-------------------");

        println!("{:#?}", contract.view_orders(&alice()));
        println!("-------------------");
        println!("{:#?}", contract.view_orders(&bob()));
        println!("-------------------");
        println!("{:#?}", contract.view_orders(&carol()));
    }

    #[test]
    fn test_6() {
        let mut contract = Contract::new();

        let pair_id_one: PairId = ("usdt.near".parse().unwrap(), "wnear.near".parse().unwrap());
        let pair_id_two: PairId = ("wnear.near".parse().unwrap(), "usdt.near".parse().unwrap());

        println!("Number of users before- {}", contract.orders.len());
        println!("Number of pairs before - {}", contract.ref_orders.len());
        println!("-------------------");

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(alice(), order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(bob(), order_bob);
            } else {
                let order_carol = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_2(carol(), order_carol);
            }
        }

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_2(alice(), order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_2(bob(), order_bob);
            } else {
                let order_carol = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_2(carol(), order_carol);
            }
        }

        println!("Number of users after- {}", contract.orders.len());
        println!("Number of pairs afters - {}", contract.ref_orders.len());
        println!("-------------------");

        println!("Number of orders from usdt.near|wnear.near - {}", contract.ref_orders.get(&pair_id_one.clone()).unwrap().len());
        println!("-------------------");
        println!("Number of orders from wnear.near|usdt.near - {}", contract.ref_orders.get(&pair_id_two.clone()).unwrap().len());
        println!("-------------------");

        println!("{:#?}", contract.view_ref_orders(&pair_id_one));
        println!("{:#?}", contract.view_ref_orders(&pair_id_two));

    }


    #[test]
    fn test_7() {
        let mut contract = Contract::new();

        let pair_id_one: PairId = ("usdt.near".parse().unwrap(), "wnear.near".parse().unwrap());
        let pair_id_two: PairId = ("wnear.near".parse().unwrap(), "usdt.near".parse().unwrap());


        println!("Number of users before- {}", contract.orders.len());
        println!("Number of pairs before - {}", contract.ref_orders.len());
        println!("-------------------");

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_3(alice(), order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_3(bob(), order_bob);
            } else {
                let order_carol = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"usdt.near\",\"buy_token\":\"wnear.near\"}".to_string();
                contract.add_order_from_string_3(carol(), order_carol);
            }
        }

        for count in 1..=9 {
            if count <= 3 {
                let order_alice = "{\"account_id\":\"alice.near\",\"amount\":\"100\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_3(alice(), order_alice);
            } else if count <= 6 {
                let order_bob = "{\"account_id\":\"bob.near\",\"amount\":\"300\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_3(bob(), order_bob);
            } else {
                let order_carol = "{\"account_id\":\"bob.near\",\"amount\":\"500\",\"sell_token\":\"wnear.near\",\"buy_token\":\"usdt.near\"}".to_string();
                contract.add_order_from_string_3(carol(), order_carol);
            }
        }


        println!("Number of users after- {}", contract.orders.len());
        println!("Number of pairs afters - {}", contract.ref_orders.len());
        println!("-------------------");

        println!("Number of orders from Alice - {}", contract.orders.get(&alice()).unwrap().len());
        println!("Number of orders from Bob - {}", contract.orders.get(&bob()).unwrap().len());
        println!("Number of orders from Carol - {}", contract.orders.get(&carol()).unwrap().len());
        println!("-------------------");

        println!("Number of orders from usdt.near|wnear.near - {}", contract.ref_orders.get(&pair_id_one.clone()).unwrap().len());
        println!("-------------------");

        println!("{:#?}", contract.view_orders(&alice()));
        println!("-------------------");
        println!("{:#?}", contract.view_orders(&bob()));
        println!("-------------------");
        println!("{:#?}", contract.view_orders(&carol()));
        println!("-------------------"); 

        println!("{:#?}", contract.view_ref_orders(&pair_id_one));
    }

}
