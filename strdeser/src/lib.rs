use strdesermac::DeserializeStruct;

pub trait Deserializable {
    fn deserialize_struct(fields: Vec<(String, String, String)>) -> Self;
}

pub struct Deserializer;

impl Deserializer {
    pub fn blabla<T>(value: Vec<(String, String, String)>) -> T 
    where
        T: Deserializable,
    {
        T::deserialize_struct(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        #[derive(DeserializeStruct)]
        pub struct User {
            pub id: i32,
            pub nickname: String,
            pub email: String,
            pub role: String,
            pub status: bool
        }

        let tuple_vec = vec![("id".to_string(), "5".to_string(), "i32".to_string()),
                                                            ("nickname".to_string(), "necoo33".to_string(), "String".to_string()),
                                                            ("email".to_string(), "arda_etiman_799@windowslive.com".to_string(), "String".to_string()),
                                                            ("role".to_string(), "user".to_string(), "String".to_string()),
                                                            ("status".to_string(), "true".to_string(), "bool".to_string())];

        let deserialized_user = User::deserialize_struct(tuple_vec);
        
        let declared_user = User {
            id: 5,
            nickname: "necoo33".to_string(),
            email: "arda_etiman_799@windowslive.com".to_string(),
            role: "user".to_string(),
            status: true
        };

        assert_eq!(declared_user.id, deserialized_user.id);
        assert_eq!(declared_user.nickname, deserialized_user.nickname);
        assert_eq!(declared_user.email, deserialized_user.email);
        assert_eq!(declared_user.role, deserialized_user.role);
        assert_eq!(declared_user.status, deserialized_user.status);
    }
}
