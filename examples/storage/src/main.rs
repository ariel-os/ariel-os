#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

use ariel_os::debug::{
    exit,
    log::{defmt, info},
    ExitCode,
};

// Imports for using [`ariel_os::storage`]
use ariel_os::storage;
use serde::{Deserialize, Serialize};

/// Example object.
///
/// The serde Serialize / Deserialize traits are required for storage
#[derive(Serialize, Deserialize, Debug, defmt::Format)]
struct MyConfig {
    val_one: heapless::String<64>,
    val_two: u64,
}

#[ariel_os::task(autostart)]
async fn main() {
    info!("Start storage example");

    // Storing a primitive type (e.g., u32)
    let value: Option<u32> = storage::get("counter").await.unwrap();
    let value = if let Some(value) = value {
        info!("got counter value {} from storage", value);
        value
    } else {
        info!("no counter value in storage. Is this the first time running this example?");
        0
    };

    if value > 10 {
        info!("counter value > 10, aborting test to save flash cycles");
        exit(ExitCode::SUCCESS);
    }
    info!("");

    storage::insert("counter", value + 1).await.unwrap();

    // By getting the Storage mutex directly, changing e.g., a counter,
    // can be done atomically w.r.t. concurrent access from the same firmware:
    {
        let mut s = storage::lock().await;
        let value: Option<u32> = s.get("another_counter").await.unwrap();
        let value = value.unwrap_or_default();
        s.insert("another_counter", value + 1).await.unwrap();
        info!("Old 'another_counter' value at {}", value);
    }
    info!("");

    // Storing a string value
    // For insertion, a literal can be used.
    info!("Don't try this in your code!");
    info!("Storing \"string_key\": \"string_value\" into storage");
    storage::insert("string_key", "string_value").await.unwrap();

    // Retrieve a string value
    if let Some(string) = storage::get::<heapless::String<64>>("string_key")
        .await
        .unwrap()
    {
        info!("got heapless string value: \"{}\"", string);
    }
    if let Some(string) = storage::get::<arrayvec::ArrayString<64>>("string_key")
        .await
        .unwrap()
    {
        // no `defmt::Format` for arrayvec, so just print length
        info!(
            "Attempting to retrieve string value as ArrayString: {:02x}",
            string.as_bytes()
        );
    }
    info!("");

    // Storing an object
    let cfg = MyConfig {
        val_one: heapless::String::<64>::try_from("some value").unwrap(),
        val_two: 99,
    };
    info!("Storing cfg object {} as struct", cfg);
    storage::insert("my_config", cfg).await.unwrap();

    // Getting an object
    // Type used for `get()` needs to match what was used for `insert()`.
    let cfg: Option<MyConfig> = storage::get("my_config").await.unwrap();
    if let Some(cfg) = cfg {
        info!("got cfg object: {:?}", cfg);
    }

    // Getting a value as raw bytes probably does not return what you want due
    // to the way postcard works
    let cfg_array: Option<arrayvec::ArrayVec<u8, 256>> = storage::get("my_config").await.unwrap();
    if let Some(cfg) = cfg_array.as_ref() {
        info!(
            "Attempting to retrieve cfg as ArrayVec: {:02x}",
            cfg.as_slice()
        );
    }

    // Same for byte arrays
    let cfg_array: Option<[u8; 10]> = storage::get("my_config").await.unwrap();
    if let Some(cfg) = cfg_array.as_ref() {
        info!("Attempting to retrieve cfg as array: {:02x}", cfg);
    }
    info!("");

    // raw bytes
    let bytes: [u8; 5] = [0, 1, 2, 3, 4];
    info!("Storing raw bytes {:02x}", bytes);
    storage::insert("some_raw_bytes", bytes).await.unwrap();

    let bytes: Option<[u8; 5]> = storage::get("some_raw_bytes").await.unwrap();
    if let Some(bytes) = bytes.as_ref() {
        info!("got bytes as array: {:02x}", bytes);
    }

    let bytes: Option<heapless::Vec<u8, 256>> = storage::get("some_raw_bytes").await.unwrap();
    if let Some(bytes) = bytes.as_ref() {
        info!(
            "Attempting to retrieve bytes as heapless vec arr: {:02x}",
            bytes
        );
    }
    info!("");

    info!("Accessing an object by composite key");
    let key1: Option<usize> = storage::get(("key", 1)).await.unwrap();
    info!("Original element with key (\"key\", 1) was: {:?}", key1);
    if key1 != Some(42) {
        info!("Storing 42 there.");
        storage::insert(("key", 1), 42usize).await.unwrap();
    }
    info!("");

    info!("Exit storage example");

    exit(ExitCode::SUCCESS);
}
