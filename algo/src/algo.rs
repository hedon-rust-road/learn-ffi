use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Algo {
    pub r#type: AlgoType,
}

#[derive(Debug, Clone, Copy)]
pub enum AlgoType {
    Blake3,
    Default,
}

impl Algo {
    pub fn new(algo_type: AlgoType) -> Self {
        Self { r#type: algo_type }
    }

    pub fn hash(&self, data: String) -> String {
        match self.r#type {
            AlgoType::Blake3 => {
                let hasher = blake3::hash(data.as_bytes());
                hasher.to_string()
            }
            AlgoType::Default => {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                hasher.finish().to_string()
            }
        }
    }

    pub fn get_name(&self) -> &str {
        match self.r#type {
            AlgoType::Blake3 => "Blake3",
            AlgoType::Default => "Default",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let algo = Algo::new(AlgoType::Default);
        let hash = algo.hash("hello world".to_string());
        assert_eq!(hash, "8170069951894177743");

        let algo = Algo::new(AlgoType::Blake3);
        let hash = algo.hash("hello world".to_string());
        assert_eq!(
            hash,
            "d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24"
        );
    }
}
