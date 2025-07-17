#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod qf_funding {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Contribution {
        pub amount: Balance,
        pub matched_amount: Balance,
        pub contributor: AccountId,
        pub time_stamp: Timestamp,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Mechanism {
        Default,
        Betas
    }

    #[ink(storage)]
    pub struct QfProject {
        project_id: AccountId,
        name: String,
        contributions: Vec<Contribution>,
        pool: Balance,
        target: Balance,
        mechanism: Mechanism

    }

    impl QfProject {
        /// Constructor to create a new QfProject with a name, pool amount, and mechanism.
        #[ink(constructor)]
        pub fn new(name: String, target: Balance, mechanism: Mechanism) -> Self {
            let project_id = Self::env().account_id();
            Self { 
                project_id,
                name,
                contributions: Vec::new(),
                pool: 0,
                target, 
                mechanism
            }
        }

        
        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn get_contributions(&self) -> Vec<Contribution> {
            self.contributions.clone()
        }

    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the constructor works.
        #[ink::test]
        fn constructor_works() {
            let qf_project = QfProject::new("Test Project".into(), 1000, Mechanism::Default);
            assert_eq!(qf_project.get_name(), "Test Project");
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
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its constructor.
        #[ink_e2e::test]
        async fn constructor_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = QfProjectRef::new("Test Project".into(), 1000, Mechanism::Default);

            // When
            let contract = client
                .instantiate("qf_funding", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<QfProject>();

            // Then
            let get_name = call_builder.get_name();
            let get_result = client.call(&ink_e2e::alice(), &get_name).dry_run().await?;
            assert_eq!(get_result.return_value(), "Test Project");

            Ok(())
        }
    }
}
