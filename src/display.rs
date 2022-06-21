use ldap3::{LdapResult, ResultEntry};

pub fn display_results(r: Vec<ResultEntry>) {
    println!("{:?}", r);
}