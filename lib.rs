#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod test1 {
    use ink::{prelude::string::String, env::call,prelude::vec::Vec};
    use chrono::Date;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Test1 {
        value: bool,
        contents: Vec<Content>
    }
    
    #[derive(Debug,Clone,scale::Encode,scale::Decode)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Content {
        // テキストデータを格納
        content : ink::prelude::string::String,
        //送信先アカウントID
        to : AccountId,
        //送信元アカウントID
        from : AccountId,
        dataid : u128,
        timestump :u128,       
    }
    
    #[ink(event)]
    pub struct TransferSingle {
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        content : ink::prelude::string::String,
    }

    #[ink(event)]
    pub struct Getcontent {
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        content : ink::prelude::string::String,
    }

    impl Test1 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value, contents:Vec::new()}
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
        ///
        #[ink(message)]
        pub fn sendmessage(&mut self,to:AccountId,message : ink::prelude::string::String)  {
            
            let caller = self.env().caller();
            self.contents.push(Content { content: message, to: to, from:caller, dataid: 0, timestump:  });
            self.env().emit_event(TransferSingle{
                operator: caller,
                from:caller,
                to,
                content: message.clone(),
            })
        }
        
        ///
        #[ink(message)]
        pub fn getmessage(&self) {
            ink::env::debug_println!("[INFO]getmessage");
            let caller = self.env().caller();
            let mut latestcontent:Option<&Content> = None; 
            //最新のみ参照する。
            for i in self.contents.iter().rev(){
                if i.to != caller{
                    continue;
                }
                latestcontent = Some(i);
            };

            self.env().emit_event(Getcontent{
                operator: caller,
                from:caller,
                to: latestcontent.unwrap().to,
                content: latestcontent.unwrap().clone().content
            })
            
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use chrono::DateTime;

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let test1 = Test1::default();
            assert_eq!(test1.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut test1 = Test1::new(false);
            assert_eq!(test1.get(), false);
            test1.flip();
            assert_eq!(test1.get(), true);
        }
        
        #[ink::test]
        fn chronotest(){
            use chrono::{DateTime, Local};
            
            let time = Local::now();
            let ts: i64 = time.timestamp();

            println!("{}", ts);           
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = Test1Ref::default();

            // When
            let contract_account_id = client
                .instantiate("test1", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<Test1Ref>(contract_account_id.clone())
                .call(|test1| test1.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = Test1Ref::new(false);
            let contract_account_id = client
                .instantiate("test1", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<Test1Ref>(contract_account_id.clone())
                .call(|test1| test1.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<Test1Ref>(contract_account_id.clone())
                .call(|test1| test1.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<Test1Ref>(contract_account_id.clone())
                .call(|test1| test1.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
