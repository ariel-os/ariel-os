#![no_main]
#![no_std]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    // async dummy to make linker happy (see https://github.com/ariel-os/ariel-os/issues/2421)
    #[init]
    async fn init() {}

    #[test]
    fn suit_vendor_id() {
        // `uuidgen --sha1 --namespace @dns --name "ariel-os.org"`
        assert_eq!(
            env!("CONFIG_SUIT_VENDOR_ID"),
            "a3f8a89a-2d3a-56cc-8c2c-f46495242647"
        );
    }

    #[test]
    fn suit_class_id() {
        // `uuidgen --sha1 --namespace "a3f8a89a-2d3a-56cc-8c2c-f46495242647" --name "board:native"`
        assert_eq!(
            env!("CONFIG_SUIT_CLASS_ID"),
            "9a16e77d-9f00-5128-90fe-c458bbf5d0e6"
        );
    }
}
