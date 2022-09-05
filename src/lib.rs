use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
enum Positions {
    President,
    Governer,
    Senator,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Voter {
    name: String,
    account: AccountId,
    timestamp: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Votes {
    position: Positions,
    candidate_name: String,
    voter: AccountId,
    timestamp: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Candidate {
    name: String,
    account: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct App {
    votes: Vector<Votes>,
    voters: Vector<Voter>,
    candidates: LookupMap<Positions, Vec<Candidate>>,
}

impl Default for App {
    fn default() -> Self {
        return App {
            votes: Vector::new(b"r".to_vec()),
            voters: Vector::new(b"r".to_vec()),
            candidates: LookupMap::new(b"r".to_vec()),
        };
    }
}

#[near_bindgen]
impl App {
    pub fn get_voters(&self) -> Vec<Voter> {
        self.voters.to_vec()
    }

    pub fn get_votes(&self) -> Vec<Votes> {
        self.votes.to_vec()
    }

    pub fn register_voter(&mut self, name: String) {
        let voter = Voter {
            name: name,
            account: env::signer_account_id(),
            timestamp: env::block_timestamp(),
        };
        self.voters.push(&voter);
    }

    pub fn register_candidate(&mut self, name: String, position: String) {
        let mut canditates_position: Option<Positions> = None;

        if position == "govener" {
            canditates_position = Some(Positions::Governer);
        } else if position == "senator" {
            canditates_position = Some(Positions::Senator)
        } else if position == "president" {
            canditates_position = Some(Positions::President)
        }

        match canditates_position {
            Some(ps) => {
                let registerd_candidates = self.candidates.get(&ps);

                let candidate = Candidate {
                    name: name,
                    account: env::signer_account_id(),
                };

                match registerd_candidates {
                    Some(mut candiates) => {
                        candiates.push(candidate);
                        self.candidates.insert(&ps, &candiates);
                    }
                    None => {
                        let mut tmp: Vec<Candidate> = vec![];
                        tmp.push(candidate);
                        self.candidates.insert(&ps, &tmp);
                    }
                }
            }
            None => env::panic_str("Postion not known, please provide an appropriate position"),
        }
    }

    pub fn vote(&mut self, candidate_name: String, position: String) {
        let mut canditates_position: Option<Positions> = None;

        if position == "govener" {
            canditates_position = Some(Positions::Governer);
        } else if position == "senator" {
            canditates_position = Some(Positions::Senator)
        } else if position == "president" {
            canditates_position = Some(Positions::President)
        }

        match canditates_position {
            Some(ps) => {
                let registerd_candidates = self.candidates.get(&ps);

                match registerd_candidates {
                    Some(cnd) => {
                        let mut candidat_indx: Option<usize> = None;

                        for (index, elem) in cnd.iter().enumerate() {
                            if elem.name == candidate_name {
                                candidat_indx = Some(index);
                                break;
                            }
                        }

                        match candidat_indx {
                            Some(_the_candidate_index) => {
                                let votes = Votes {
                                    candidate_name: candidate_name,
                                    voter: env::signer_account_id(),
                                    timestamp: env::block_timestamp(),
                                    position: ps,
                                };
                                self.votes.push(&votes);
                            }
                            None => {
                                env::log_str("Te name of the candidate was not found in the system")
                            }
                        }
                    }
                    None => env::log_str("There are no candidates for this position provided "),
                }
            }
            None => env::log_str("Postion not known, please provide an appropriate position"),
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::AccountId;

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn register_candidate() {
        let mut app: App = App::default();
        app.register_candidate("Steve".to_string(), "president".to_string());
        let candidates = app.candidates.get(&Positions::President).unwrap();
        assert_eq!(candidates.len(), 1)
    }

    #[test]
    fn register_voter() {
        let user = AccountId::new_unchecked("steve.testnet".to_string());
        let mut context = get_context(user.clone());
        context.block_timestamp(9999);

        let mut app: App = App::default();
        app.register_voter("Steve".to_string());
        assert_eq!(app.voters.len(), 1)
    }

    #[test]
    fn vote() {
        let user = AccountId::new_unchecked("steve.testnet".to_string());
        let mut context = get_context(user.clone());
        context.block_timestamp(9999);

        let mut app: App = App::default();

        app.register_candidate("Steve".to_string(), "president".to_string());
        let candidates = app.candidates.get(&Positions::President).unwrap();
        assert_eq!(candidates.len(), 1);

        let mut app: App = App::default();
        app.register_voter("Steve".to_string());
        assert_eq!(app.voters.len(), 1);

        app.vote("Steve".to_string(), "president".to_string());

        assert_eq!(app.votes.len(), 1)
    }
}
