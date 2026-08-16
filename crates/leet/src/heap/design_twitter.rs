// lc 355 - Design Twitter

use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Twitter {
    time: i32,
    tweets: HashMap<i32, Vec<(i32, i32)>>, // userId -> [(time, tweetId)]
    follows: HashMap<i32, HashSet<i32>>,    // followerId -> {followeeIds}
}

impl Twitter {
    pub fn new() -> Self {
        Twitter {
            time: 0,
            tweets: HashMap::new(),
            follows: HashMap::new(),
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        self.tweets.entry(user_id).or_default().push((self.time, tweet_id));
    }

    /// Merge k sorted lists using a max-heap.
    /// Each entry: (time, tweetId, userId, idx) where idx is position from the end.
    pub fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<(i32, i32, i32, usize)> = BinaryHeap::new();

        // Collect followees + self (deduplicated)
        let mut user_set: HashSet<i32> = self
            .follows
            .get(&user_id)
            .cloned()
            .unwrap_or_default();
        user_set.insert(user_id);
        let users: Vec<i32> = user_set.into_iter().collect();

        // Seed heap with the most recent tweet from each user
        for &uid in &users {
            if let Some(tweets) = self.tweets.get(&uid) {
                if !tweets.is_empty() {
                    let idx = tweets.len() - 1;
                    let (time, tid) = tweets[idx];
                    heap.push((time, tid, uid, idx));
                }
            }
        }

        let mut feed = Vec::with_capacity(10);
        while let Some((_, tid, uid, idx)) = heap.pop() {
            feed.push(tid);
            if feed.len() == 10 {
                break;
            }
            if idx > 0 {
                let next_idx = idx - 1;
                let (time, next_tid) = self.tweets[&uid][next_idx];
                heap.push((time, next_tid, uid, next_idx));
            }
        }
        feed
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_default().insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.follows.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
    }

    #[test]
    fn test_empty_feed() {
        let mut twitter = Twitter::new();
        assert_eq!(twitter.get_news_feed(1), Vec::<i32>::new());
    }

    #[test]
    fn test_max_10_tweets() {
        let mut twitter = Twitter::new();
        for i in 1..=15 {
            twitter.post_tweet(1, i);
        }
        let feed = twitter.get_news_feed(1);
        assert_eq!(feed.len(), 10);
        // Most recent 10 tweets in reverse chronological order
        assert_eq!(feed, vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6]);
    }

    #[test]
    fn test_follow_self() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 100);
        twitter.follow(1, 1); // follow self should not duplicate
        let feed = twitter.get_news_feed(1);
        assert_eq!(feed, vec![100]); // no duplicate
    }

    #[test]
    fn test_unfollow_non_followee() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 10);
        twitter.unfollow(1, 2); // should not panic
        assert_eq!(twitter.get_news_feed(1), vec![10]);
    }

    #[test]
    fn test_multiple_users_interleaved() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 1);
        twitter.post_tweet(2, 2);
        twitter.post_tweet(1, 3);
        twitter.post_tweet(2, 4);
        twitter.post_tweet(3, 5);

        twitter.follow(1, 2);
        twitter.follow(1, 3);

        let feed = twitter.get_news_feed(1);
        // All tweets in reverse chronological: 5, 4, 3, 2, 1
        assert_eq!(feed, vec![5, 4, 3, 2, 1]);
    }
}
