use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Voter {
    name:String,
    president_vote:String,
    senator_vote:String,
    mp_vote:String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct App {
    voters:Vec<Voter>,
    president_vote:Vec<String>,
    senator_vote:Vec<String>,
    mp_vote:Vec<String>,
}

impl Default for App{
    fn default() -> self{
    return App{
        voters : vec![]
        president : vec![]
        senator : vec![]
        mp : vec![]
    }
   }
}

#[near_bindgen]
impl App {
    pub fn vote(&mut self, name_vote:String, president_candidate:String, sanator_candidate:String, mp_candidate:String){
      
        let mut candidate_president_option: Option<String> = None;
        let mut candidate_sanator_option: Option<String> = None;
        let mut candidate_mp_option: Option<String> = None;

        for elem in self.president.iter(){
            if elem.eq(&president_candidate){
                candidate_president_option = Some(elem.clone());
            }
        }
        for elem in self.senator.iter(){
            if elem.eq(&senator_candidate){
                candidate_senator_option = Some(elem.clone());
            }
        }
        for elem in self.mp.iter(){
            if elem.eq(&mp_candidate){
                candidate_mp_option = Some(elem.clone());
            }
        }
        if candidate_president_option == None {
            env::log_str("President vote name not found!");
        }else if candidate_sanator_option == None{
            env::log_str("Senator vote name not found!");
        }else if candidate_mp_option == None{
            env::log_str("Mp vote name not found");
        }else{
            let vt:Voter = Voter{
                name:name_vote,
                president_vote:candidate_president_option.unwrap(),
                senator_vote:candidate_senator_option.unwrap(),
                mp_vote:candidate_mp_option.unwrap(),
            };
            self.voters.push(vt)
        }
    }
    pub fn register_candidate(&mut self, name:String, position:String){
        if position == "president".to_string(){
            self.president.push(name);
        }else if position == "senator".to_string(){
            self.senator.push(name);
        }else if position == "mp".to_string(){
            self.mp.push(name);
        }else env::log_str("position not known!");
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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn register_candidate(){
     let mut app: App = App::default();
     app.register_candidate(name:"Steve".to_string(), position:"president".to_string());
     assert_eq!(app.President.len(),1)
    }
    #[test]
    fn vote(){
    let mut app: App = App::default();
        app.register_candidate(name:"Steve".to_string(), position:"president".to_string());
        app.register_candidate(name:"James".to_string(), position:"senator".to_string());
        app.register_candidate(name:"Brian".to_string(), position:"mp".to_string());

        assert_eq!(app.president.len(),1);
        assert_eq!(app.senator.len(),1);
        assert_eq!(app.mp.len(),1);

        app.vote(name_vote:"Steve".to_string, president_candidate:"Steve".to_string, name_vote:"James".to_string,
         senator_candidate:"James".to_string, name_vote:"Brian".to_string, mp_candidate:"Brian".to_string)

         assert_eq!(app.voters.len(),1);
    }
}
