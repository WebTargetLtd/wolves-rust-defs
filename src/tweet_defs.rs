#![allow(dead_code)]

///
/// This struct maps the raw tweets 
/// 
/// 
/// 

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
struct User {
    id: u64,
    id_str: String,
    name: String,
    screen_name: String,
    location: Option<String>,
    url: Option<String>,
    description: Option<String>,
    followers_count: u32,
    friends_count: u32,
    statuses_count: u32,
    created_at: String,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
struct Mention {
    screen_name: String,
    name: String,
    id: u64,
    id_str: String,
    indices: Vec<u32>,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
struct Entities {
    user_mentions: Vec<Mention>,
}

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
pub struct TweetCore {
    text: String,
    retweet_count: u32,
    user: User,

}


#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize, Clone))]
pub struct Tweet{
    created_at: String,
    id: u64,
    id_str: String,
    text: String,
    display_text_range: Option<Vec<u32>>,
    source: String,
    truncated: bool,
    in_reply_to_status_id: Option<u64>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<u64>,
    in_reply_to_user_id_str: Option<String>,
    in_reply_to_screen_name: Option<String>,
    user: User,
    geo: Option<String>,
    coordinates: Option<String>,
    place: Option<String>,
    contributors: Option<String>,
    is_quote_status: bool,
    quote_count: Option<u32>,
    reply_count: Option<u32>,
    retweet_count: u32,
    favorite_count: u32,
    entities: Entities,
    favorited: bool,
    retweeted: bool,
    filter_level: Option<String>,
    lang: Option<String>,
    timestamp_ms: Option<String>,
}

impl Tweet {
    
    pub fn tweet_core(original: Tweet) -> TweetCore {
        TweetCore { text: original.text, retweet_count: original.retweet_count, user: original.user }
    }
}
