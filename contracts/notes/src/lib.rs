#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Cat {
    pub id: u64,
    pub name: String,
    pub breed: String,
    pub description: String,
    pub image_url: String, // Menyimpan link Gambar/GIF kucing
}

// Key storage yang benar
const CAT_DATA: Symbol = symbol_short!("CAT_DATA");

#[contract]
pub struct CatAdoptionContract;

#[contractimpl]
impl CatAdoptionContract {

    // MEMPERBAIKI: Mengubah &NOTE_DATA menjadi &CAT_DATA
    pub fn get_all_cats(env: Env) -> Vec<Cat> {
        env.storage().instance().get(&CAT_DATA).unwrap_or(Vec::new(&env))
    }

    pub fn register_cat(
        env: Env, 
        name: String, 
        breed: String, 
        description: String, 
        image_url: String, 
    ) -> String {
        let mut cat_list: Vec<Cat> = env.storage().instance().get(&CAT_DATA).unwrap_or(Vec::new(&env));
        
        // Menghasilkan ID acak menggunakan PRNG Soroban
        let random_id = env.prng().gen_range(100_000..1000_000);
        
        let new_cat = Cat {
            id: random_id,
            name,
            breed,
            description,
            image_url,
        };
        
        cat_list.push_back(new_cat);
        env.storage().instance().set(&CAT_DATA, &cat_list);
        
        String::from_str(&env, "Meow berhasil didaftarkan dengan gambar!")
    }

    pub fn adopt_cat(env: Env, id: u64) -> String {
        let mut cat_list: Vec<Cat> = env.storage().instance().get(&CAT_DATA).unwrap_or(Vec::new(&env));
        
        let mut found_index: Option<u32> = None;

        // Mencari indeks kucing yang cocok
        for i in 0..cat_list.len() {
            if cat_list.get(i).unwrap().id == id {
                found_index = Some(i);
                break; // Hentikan loop jika sudah ketemu
            }
        }

        // Eksekusi penghapusan di luar loop (Lebih aman dan bersih)
        if let Some(index) = found_index {
            cat_list.remove(index);
            env.storage().instance().set(&CAT_DATA, &cat_list);
            String::from_str(&env, "Selamat! Meow berhasil diadopsi.")
        } else {
            String::from_str(&env, "Maaf, data Meow tidak ditemukan.")
        }
    }
}