use std::cell::RefCell;
use sha2::{Sha512,Digest};
use hex;

thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
    //static USER_NAME: RefCell<Vec<String>> = RefCell::default();
    //static PASSWD: RefCell<Vec<String>> = RefCell::default();
    //static USER_PID: RefCell<Vec<String>> = RefCell::default();
    static USER_DATA: RefCell<Vec<(String,String)>> = RefCell::default();
}

//Non existing component: (left for future purposes)

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

//end
//Posts components:

#[ic_cdk::update]
fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy| {
        let mut mutable_wpisy = wpisy.borrow_mut();
        mutable_wpisy.push(wpis);
    });
}

#[ic_cdk::query]
fn pobierz_wpisy() -> Vec<String>{
    WPISY.with(|wpisy| wpisy.borrow().clone())
}

//end
//Login components
#[ic_cdk::update]
fn add_user(usr: String,passwd: String){
    //processing data:
    let mut hashed_passwd = hash(passwd);
    let data: (String, String) = (usr, hashed_passwd);
    //as user_ID is used position on the blockchain
    USER_DATA.with(|users| {
        let mut allcontent = users.borrow_mut();
        allcontent.push(data);
    });
}

fn hash(passwd: String) -> String{
    let mut hasher = Sha512::new();
    hasher.update(passwd);
    let res = hasher.finalize();
    hex::encode(res)
}

//1 - user with this name already exists
//0 - there is no user with this name inside blockchain
#[ic_cdk::query]
fn check_if_username_already_exist(user_to_check: String){
    let mut array = test_list_passwords();
    for Usr_Nm in array.iter(){
        if( Usr_Nm[0] == user_to_check){
            return 1;
        }
    }
    return 0;
}

//1 - everything has gone perfectly
//2 - username is okay but passwd is wrong
//3 - there is no such username inside blockchain
#[ic_cdk::query]
fn check_password(user_to_check: String, passwd_to_check: String){
    let mut array = test_list_passwords();
    for Usr_Nm in array.iter(){
        if( Usr_Nm[0] == user_to_check){
            //passwd before checking needs to be hashed
            let hashed_passwd_to_check = hash(passwd_to_check);
            if( hashed_passwd_to_check == Usr_Nm[1]){
                return 1;
            }
            else{
                return 2;
            }
        }
    }
    return 3;    
}

//end
//testing section:
//0 element is representing username, other passwd
#[ic_cdk::query]
fn test_list_passwords() -> Vec<(String,String)>{
    USER_DATA.with(|data| data.borrow().clone())
}
//end
//Functions crucial for all fns to work:

ic_cdk::export_candid!();

//end