use scrypto::prelude::*;


#[derive(ScryptoSbor)]
pub struct Proposal {
    description: String,
    votes_for: Decimal,
    votes_against: Decimal,
}

#[blueprint]
mod dao {
    struct Dao {
        sample_vault: Vault,
        proposals: HashMap<u128, Proposal>,
        proposal_count: u128
    }

    impl Dao {
        pub fn instantiate_dao() -> Global<Dao> {
            let my_bucket: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata! {
                    init {
                        "name" => "INSIDER PASS", locked;
                        "symbol" => "IP", locked;
                    }
                })
                .mint_initial_supply(1000)
                .into();

            Self {
                sample_vault: Vault::with_bucket(my_bucket),
                proposals: HashMap::new(),
                proposal_count: 0
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        //one token can be given away
        //other token needs to be purchased
        pub fn get_first_insider_pass(&mut self) -> Bucket {
            info!(
                "DAO welcomes more: {} members. Now giving away a first free INSIDER PASS membership token!",
                self.sample_vault.amount() - 1
            );

            self.sample_vault.take(1)
        }

        pub fn create_a_proposal(&mut self, description: String) -> u128 {

            let proposal_id = self.proposal_count;

            self.proposals.insert(
                proposal_id,
                Proposal {
                    description,
                    votes_for: Decimal::zero(),
                    votes_against: Decimal::zero(),
                },
            );

            self.proposal_count += 1;

            info!(
                "your proposal has been submitted with id : {}",
                proposal_id
            );

            proposal_id
        }

        pub fn cast_a_vote(&mut self, proposal_id: u128, support: bool) {
            let proposal = self.proposals.get_mut(&proposal_id).unwrap();

            if support {
                proposal.votes_for += Decimal::one();
            } else {
                proposal.votes_against += Decimal::one();
            }
        }

        pub fn results(&self, proposal_id: u128) -> Option<(Decimal, Decimal)> {
            self.proposals
                .get(&proposal_id)
                .map(|p| (p.votes_for, p.votes_against))
        }

        //function I will make :
        //get_all_proposals
        //store_information_of_proposal_creator
        //condition check - Address having INSIDER PASS should be able to create proposal, cast a vote


    }
}

//instantiate
//resim call-function package_sim1pk3cmat8st4ja2ms8mjqy2e9ptk8y6cx40v4qnfrkgnxcp2krkpr92 Dao instantiate_dao

//component balance
//resim show component_sim1crkp7q8sfhg7xa0xvqtdjezltj3hams2hrk4ztzqs2c90sy0cslv6a

//account balance
//resim show account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma

//call_get_first_insider_pass
//resim call-method component_sim1crkp7q8sfhg7xa0xvqtdjezltj3hams2hrk4ztzqs2c90sy0cslv6a get_first_insider_pass

//call_create_a_proposal
//resim call-method component_sim1crkp7q8sfhg7xa0xvqtdjezltj3hams2hrk4ztzqs2c90sy0cslv6a create_a_proposal "I want this platform to bring REWARD TOKENS for vote casters"

//another proposal
//resim call-method component_sim1crkp7q8sfhg7xa0xvqtdjezltj3hams2hrk4ztzqs2c90sy0cslv6a create_a_proposal "introduce NEW TOKEN STANDARD"

//call_cast_a_vote
//resim call-method component_sim1crkp7q8sfhg7xa0xvqtdjezltj3hams2hrk4ztzqs2c90sy0cslv6a cast_a_vote 0 true

//call_results
//resim call-method component_sim1crkp7q8sfhg7xa0xvqtdjezltj3hams2hrk4ztzqs2c90sy0cslv6a results 0




