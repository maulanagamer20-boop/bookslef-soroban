#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, String, Vec, Symbol
};

#[contract]
pub struct RatingContract;

// 🔑 storage key
const RATING_DATA: Symbol = Symbol::short("RATE");

// 📦 Struct Rating
#[contracttype]
#[derive(Clone)]
pub struct Rating {
    pub id: u64,
    pub score: u32,      // rating 1 - 5
    pub comment: String,
}

#[contractimpl]
impl RatingContract {

    // 🔍 Ambil semua rating
    pub fn get_ratings(env: Env) -> Vec<Rating> {
        env.storage()
            .instance()
            .get(&RATING_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // ➕ Tambah rating
    pub fn add_rating(env: Env, score: u32, comment: String) -> String {
        if score < 1 || score > 5 {
            return String::from_str(&env, "Rating must be between 1 and 5");
        }

        let mut ratings: Vec<Rating> = env.storage()
            .instance()
            .get(&RATING_DATA)
            .unwrap_or(Vec::new(&env));

        let rating = Rating {
            id: env.prng().gen::<u64>(),
            score,
            comment,
        };

        ratings.push_back(rating);

        env.storage().instance().set(&RATING_DATA, &ratings);

        String::from_str(&env, "Rating added")
    }

    // ❌ Hapus rating
    pub fn delete_rating(env: Env, id: u64) -> String {
        let mut ratings: Vec<Rating> = env.storage()
            .instance()
            .get(&RATING_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..ratings.len() {
            if ratings.get(i).unwrap().id == id {
                ratings.remove(i);
                env.storage().instance().set(&RATING_DATA, &ratings);
                return String::from_str(&env, "Deleted");
            }
        }

        String::from_str(&env, "Rating not found")
    }
}