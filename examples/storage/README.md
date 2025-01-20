# storage

## About

This application demonstrates use of the
[ariel-os-storage](https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/storage/index.html)
API.

The example writes a number of key-value pairs to storage and retrieves them.
Different types of values are written to storage to demonstrate the capablities.

Values are also retrieved with different types as they were stored with,
to show how values are decoded when different types are used
between `insert` and `get`.
While doing so won’t cause unsafety, it might return garbage data, or panic.

Note: The application is not stateless, as it writes to flash.

## How to run

In this folder, run

    laze build -b nrf52840dk run

The application will store and read back a value to the flash.

When the maximum number of cycles has been reached
and the application exits early to avoid wearing out the flash,
the `flash-erase-all` laze task can be used to completely erase
the flash memory:

    laze build -b nrf52840dk flash-erase-all

## Expected Output

When run, this example shows the following output:

    INFO  storage: using flash range 0xe000..0x10000
    INFO  storage: initializing
    INFO  Start storage example
    INFO  no counter value in storage. Is this the first time running this example?
    INFO  
    INFO  Old 'another_counter' value at 0
    INFO  
    INFO  Storing "string_key": "string_value" into storage
    INFO  got heapless string value: "string_value"
    INFO  Attempting to retrieve string value as ArrayString: [73, 74, 72, 69, 6e, 67, 5f, 76, 61, 6c, 75, 65]
    INFO  
    INFO  Storing cfg object MyConfig { val_one: "some value", val_two: 99 } as struct
    INFO  got cfg object: MyConfig { val_one: "some value", val_two: 99 }
    INFO  Attempting to retrieve cfg as ArrayVec: [73, 6f, 6d, 65, 20, 76, 61, 6c, 75, 65]
    INFO  Attempting to retrieve cfg as array: [0a, 73, 6f, 6d, 65, 20, 76, 61, 6c, 75]
    INFO  
    INFO  Storing raw bytes [00, 01, 02, 03, 04]
    INFO  got bytes as array: [00, 01, 02, 03, 04]
    INFO  Attempting to retrieve bytes as heapless vec arr: []
    INFO  
    INFO  Exit storage example

When the initial counter reached the maximum number of executions, the following
output is produced:

    INFO  storage: using flash range 0xe000..0x10000
    INFO  Start storage example
    INFO  got counter value 11 from storage
    INFO  counter value > 10, aborting test to save flash cycles
