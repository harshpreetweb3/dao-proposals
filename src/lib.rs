use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub struct Proposal {
    description: String,
    votes_for: Decimal,
    votes_against: Decimal,
    creator: ComponentAddress,
}

#[derive(ScryptoSbor)]
pub struct StatusOfGovToken {
    pub price: Decimal,
    pub amount: Decimal,
}

#[blueprint]
mod dao {

    // enable_method_auth! {
    //     // decide which methods are public and which are restricted to the component's owner
    //     methods {
    //         buy_insider_pass_token => PUBLIC;
    //         get_status_of_governance_token => PUBLIC;
    //         cast_a_vote => PUBLIC;
    //         create_a_proposal => PUBLIC;
    //         get_first_insider_pass => PUBLIC;
    //         results => PUBLIC;
    //         set_price => restrict_to: [OWNER];
    //         withdraw_earnings => restrict_to: [OWNER];
    //     }
    // }


    struct Dao {
        insider_pass: Vault,
        proposals: HashMap<u128, Proposal>,
        proposal_count: u128,
        token_price: Decimal,
        collected_xrd: Vault,
        received_free_tokens: HashSet<ComponentAddress>,
        // collected_insider_passes : Vault
    }

    impl Dao {
        pub fn instantiate_dao(price: Decimal) -> Global<Dao> {
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
                insider_pass: Vault::with_bucket(my_bucket),
                proposals: HashMap::new(),
                proposal_count: 0,
                token_price: price,
                collected_xrd: Vault::new(XRD),
                received_free_tokens: HashSet::new(),
                // collected_insider_passes : Vault::new(IP)
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn get_status_of_governance_token(&self) -> StatusOfGovToken {
            StatusOfGovToken {
                price: self.token_price,
                amount: self.insider_pass.amount(),
            }
        }

        pub fn get_first_insider_pass(&mut self, your_address: ComponentAddress) -> Bucket {
            assert!(
                !self.received_free_tokens.contains(&your_address),
                "You have already received a free token"
            );

            info!(
                "DAO welcomes more: {} members. And gives away a first free INSIDER PASS membership token!",
                self.insider_pass.amount() - 1
            );

            self.received_free_tokens.insert(your_address);

            info!("you have got your first free token. Just go and check your account balance");

            self.insider_pass.take(1)
        }

        pub fn create_a_proposal(
            &mut self,
            description: String,
            mut payment: Bucket,
            creator: ComponentAddress,
        ) -> u128 {
            let proposal_id = self.proposal_count;

            let our_share = payment.take(1);
            our_share.burn();

            self.proposals.insert(
                proposal_id,
                Proposal {
                    description,
                    votes_for: Decimal::zero(),
                    votes_against: Decimal::zero(),
                    creator,
                },
            );

            self.proposal_count += 1;

            info!("your proposal has been submitted with id : {}", proposal_id);

            proposal_id
        }

        pub fn cast_a_vote(
            &mut self,
            mut payment: Bucket,
            amount: Decimal,
            proposal_id: u128,
            support: bool,
        ) {
            let proposal = self.proposals.get_mut(&proposal_id).unwrap();
            let our_share = payment.take(amount);
            our_share.burn();
            if support {
                proposal.votes_for +=  10000 * Decimal::one();
            } else {
                proposal.votes_against += 10000 * Decimal::one();
            }
        }

        pub fn results(&self, proposal_id: u128) -> Option<(Decimal, Decimal)> {
            self.proposals
                .get(&proposal_id)
                .map(|p| (p.votes_for, p.votes_against))
        }

        pub fn buy_insider_pass_token(&mut self, mut payment: Bucket) -> (Bucket, Bucket) {
            // take our price in XRD out of the payment
            // if the caller has sent too few, or sent something other than XRD, they'll get a runtime error
            let our_share = payment.take(self.token_price);
            self.collected_xrd.put(our_share);

            // we could have simplified the above into a single line, like so:
            // self.collected_xrd.put(payment.take(self.price));

            // return a tuple containing a gumball, plus whatever change is left on the input payment (if any)
            // if we're out of gumballs to give, we'll see a runtime error when we try to grab one
            (self.insider_pass.take(1), payment)
        }

        pub fn set_price(&mut self, price: Decimal) {
            self.token_price = price
        }

        pub fn withdraw_earnings(&mut self) -> Bucket {
            self.collected_xrd.take_all()
        }

        pub fn get_my_created_proposals(&self, creator: ComponentAddress) -> Vec<(u128, Proposal)> {
            self.proposals
                .iter()
                .filter(|(_, proposal)| proposal.creator == creator)
                .map(|(&id, proposal)| {
                    (
                        id,
                        Proposal {
                            description: proposal.description.clone(),
                            votes_for: proposal.votes_for,
                            votes_against: proposal.votes_against,
                            creator: proposal.creator,
                        },
                    )
                })
                .collect()
        }

        pub fn get_all_proposals(&self) -> Vec<(u128, Proposal)> {
            self.proposals
                .iter()
                .map(|(&id, proposal)| {
                    (
                        id,
                        Proposal {
                            description: proposal.description.clone(),
                            votes_for: proposal.votes_for,
                            votes_against: proposal.votes_against,
                            creator: proposal.creator,
                        },
                    )
                })
                .collect()
        }

        //function I will make :
        //get_all_proposals
        //store_information_of_proposal_creator
        //condition check - Address having INSIDER PASS should be able to create proposal, cast a vote
    }
}

//instantiate
//resim call-function package_sim1pk6e5spka5nump4mjj5nhk0cq85xxmyw9cqxq2vn3kfg3jy0yugg80 Dao instantiate_dao

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
