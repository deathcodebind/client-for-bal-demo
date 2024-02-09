#![allow(clippy::all, warnings)]
pub struct signIn;
pub mod sign_in {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "signIn";
    pub const QUERY : & str = "query signIn($account: String!, $password: String!) {\n    signIn(account: $account, password: $password)\n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub account: String,
        pub password: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "signIn")]
        pub sign_in: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for signIn {
    type Variables = sign_in::Variables;
    type ResponseData = sign_in::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: sign_in::QUERY,
            operation_name: sign_in::OPERATION_NAME,
        }
    }
}
