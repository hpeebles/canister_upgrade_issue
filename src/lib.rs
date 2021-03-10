use ic_cdk::storage;
use ic_cdk_macros::*;

#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((storage::get::<u32>(),)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    //panic!("here!");

    let (from_stable_storage,): (u32,) = storage::stable_restore().unwrap();

    let value: &mut u32 = storage::get_mut();

    *value = from_stable_storage;
}