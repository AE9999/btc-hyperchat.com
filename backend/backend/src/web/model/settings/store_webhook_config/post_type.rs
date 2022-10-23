
pub type PostTypeValue = i32;

#[derive(PartialEq, Clone, Debug)]
pub enum PostType {
    Undefined,
    ApplicationXWwwFormUrlEncoded,
    Json,
}

impl PostType {
    pub fn from_code(code: PostTypeValue) -> Self {
        match code {
            1 => PostType::ApplicationXWwwFormUrlEncoded,
            2 => PostType::Json,
            _ => PostType::Undefined,
        }
    }


    pub fn to_code(&self) -> PostTypeValue {
        match *self {
            PostType::Undefined => 0,
            PostType::ApplicationXWwwFormUrlEncoded => 1,
            PostType::Json => 2,
        }
    }

    pub fn is_undefined(&self) -> bool {
        match *self {
            PostType::Undefined => true,
            _ => false,
        }
    }

    pub fn is_json(&self) -> bool {
        match *self {
            PostType::Json => true,
            _ => false,
        }
    }
}
