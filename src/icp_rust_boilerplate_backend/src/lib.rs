#[macro_use]

extern crate serde;

use candid::{Decode, Encode};

use ic_cdk::api::time;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};

use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};

use std::{borrow::Cow, cell::RefCell};



type Memory = VirtualMemory<DefaultMemoryImpl>;

type IdCell = Cell<u64, Memory>;



#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]

struct Lele {
    id: u64,
    variety: String,
    age: u64,
    weight: u64,
    image_url: String,
    created_at: u64,
    updated_at: Option<u64>,
}



// a trait that must be implemented for a struct that is stored in a stable struct

impl Storable for Lele {

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {

        Cow::Owned(Encode!(self).unwrap())

    }



    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {

        Decode!(bytes.as_ref(), Self).unwrap()

    }

}



// another trait that must be implemented for a struct that is stored in a stable struct

impl BoundedStorable for Lele {

    const MAX_SIZE: u32 = 1024;

    const IS_FIXED_SIZE: bool = false;

}



thread_local! {

        static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(

            MemoryManager::init(DefaultMemoryImpl::default())

        );

    

        static ID_COUNTER: RefCell<IdCell> = RefCell::new(

            IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)

                .expect("Cannot create a counter")

        );

    

        static STORAGE: RefCell<StableBTreeMap<u64, Lele, Memory>> =

            RefCell::new(StableBTreeMap::init(

                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))

        ));

    }



#[derive(candid::CandidType, Serialize, Deserialize, Default)]

struct LelePayload {
    variety: String,
    age: u64,
    weight: u64,
    image_url: String,
}



#[ic_cdk::query]

fn get_lele(id: u64) -> Result<Lele, Error> {

    match _get_lele(&id) {

        Some(lele) => Ok(lele),

        None => Err(Error::NotFound {

            msg: format!("a lele with id={} not found", id),

        }),

    }

}



#[ic_cdk::update]

fn add_lele(lele: LelePayload) -> Option<Lele> {

    let id = ID_COUNTER

        .with(|counter| {

            let current_value = *counter.borrow().get();

            counter.borrow_mut().set(current_value + 1)

        })

        .expect("cannot increment id counter");

    let lele = Lele {
        id,
        variety: lele.variety,
        age: lele.age,
        weight: lele.weight,
        image_url: lele.image_url,
        created_at: time(),
        updated_at: None,
    };

    do_insert(&lele);

    Some(lele)

}



#[ic_cdk::update]

fn update_lele(id: u64, payload: LelePayload) -> Result<Lele, Error> {

    match STORAGE.with(|service| service.borrow().get(&id)) {

        Some(mut lele) => {
            lele.variety = payload.variety;

            lele.image_url = payload.image_url;

            lele.weight = payload.weight;

            lele.age = payload.age;

            lele.updated_at = Some(time());

            do_insert(&lele);

            Ok(lele)

        }

        None => Err(Error::NotFound {

            msg: format!(

                "couldn't update a lele with id={}. lele not found",

                id

            ),

        }),

    }

}



// helper method to perform insert.

fn do_insert(lele: &Lele) {

    STORAGE.with(|service| service.borrow_mut().insert(lele.id, lele.clone()));

}



#[ic_cdk::update]

fn delete_lele(id: u64) -> Result<Lele, Error> {

    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {

        Some(lele) => Ok(lele),

        None => Err(Error::NotFound {

            msg: format!(

                "couldn't delete a lele with id={}. lele not found.",

                id

            ),

        }),

    }

}



#[derive(candid::CandidType, Deserialize, Serialize)]

enum Error {

    NotFound { msg: String },

}



// a helper method to get a lele by id. used in get_lele/update_lele

fn _get_lele(id: &u64) -> Option<Lele> {

    STORAGE.with(|service| service.borrow().get(id))

}



// need this to generate candid

ic_cdk::export_candid!();