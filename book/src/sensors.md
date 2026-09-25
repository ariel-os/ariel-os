# Sensors

<!-- NOTE: Should be kept semantically in sync with the homepage of `ariel-os-sensors` and `ariel_os::sensors`. -->
Ariel OS introduces a sensor API and a matching ecosystem of sensor drivers.
This sensor API has two main goals:

- Providing a unified way of accessing the readings from all registered sensor driver instances in a homogeneous way.
- Making it easy and as transparent as possible to substitute a specific sensor device by a similar one from the same category.

## Definitions

The sensor API and this documentation rely on the following definitions:

<!-- NOTE: Should be kept semantically in sync with the homepage of `ariel-os-sensors` and `ariel_os::sensors`. -->
- A *sensor device* is a device measuring one or multiple physical quantities and reporting them as one or more digital values—we call these values *samples*.
- Sensor devices measuring the same physical quantity are said to be part of the same *sensor category*.
  A sensor device may be part of multiple sensor categories.
- A *measurement* is the physical operation of measuring one or several physical quantities.
- A *reading* is the digital result returned by a sensor device after carrying out a measurement.
  Samples of different physical quantities can therefore be part of the same reading.
- A *sensor driver* refers to a sensor device as exposed by the sensor abstraction layer.
- A *sensor driver instance* is an instance of a sensor driver.

> [!NOTE]
> Currently, existing sensor drivers are mainly focused on sensor devices with a digital serial interface (e.g., [I2C][i2c-book], [SPI][spi-book]), but this is not a limitation of the sensor API: it should also accommodate analog sensor devices that require readings of the internal ADC for instance.

## Ariel OS Sensor API

The core of the sensor API is the dyn-compatible `ariel_os_sensors::Sensor` trait.
It defines a single interface that sensor drivers implement, to allow homogeneous access to their readings.
The main two methods are [`Sensor::trigger_measurement()`][ariel-os-sensors-mod-sensor-trigger-measurement-rustdoc] and [`Sensor::wait_for_reading()`][ariel-os-sensors-mod-sensor-wait-for-reading-rustdoc] which, together, allow triggering a measurement and awaiting its reading, in a polling fashion: the measurement is explicitly triggered, but [`Sensor::wait_for_reading()`][ariel-os-sensors-mod-sensor-wait-for-reading-rustdoc] returns a `Future`, which completes when the reading is available (which may take some time depending on the sensor device).

> [!NOTE]
> In the future, the API should be expanded to provide abstractions over sensor device interrupts, specific to each sensor category.

Additionally, a registry exposes sensor driver instances that have been registered in the application, providing a single access point to them, and allowing for instance to iterate at runtime over all the sensor driver instances registered in the application.

## Using Sensor Drivers through the API

This section covers the usage of existing sensor drivers.
<!-- To implement a new sensor driver, see [the dedicated section]. -->

### Initializing the Sensor Driver

To use a sensor driver implementing Ariel OS sensor API, the sensor driver's package must first be added as a dependency.
Then, the sensor driver must be instantiated, and the instance registered into the registry.
Additionally, sensor drivers typically require an async runner that needs to run continuously to process measurement triggers.

There is not currently an explicitly defined interface for sensor *initialization*, even if existing sensor drivers try to follow a common one.
For this reason, see the sensor-related examples to learn how to initialize the existing sensor drivers.

> [!NOTE]
> Sensor initialization is planned to be later abstracted over and automatically generated from [SBD files][sbd-book].
> The initialization of the existing sensor drivers may need to be significantly overhauled when doing so.

### Accessing the Sensor Driver Instance

Sensor driver instances are accessed through the registry: [`ariel_os::sensors::REGISTRY`][ariel-os-sensors-mod-registry-rustdoc].
Its [`sensors()`][ariel-os-sensors-mod-registry-sensors-rustdoc] method returns an iterator over all registered sensor driver instances.
If a specific sensor driver instance is desired, the iterator can be filtered based on the sensor categories, obtained with [`Sensor::categories()`][ariel-os-sensors-mod-sensor-categories-rustdoc], and on the sensor driver *instance*'s label, obtained with [`Sensor::label()`][ariel-os-sensors-mod-sensor-label-rustdoc].
The label is defined when instantiating the sensor driver and allows disambiguating between multiple identical sensor devices: the label can for instance indicate where the sensor device is located or what its function is.

### Triggering Measurements and Obtaining Readings

After obtaining a reference to the sensor driver instance, a measurement can be triggered and then awaited.
There is no bound on how long a measurement can take, so [`Sensor::wait_for_reading()`][ariel-os-sensors-mod-sensor-wait-for-reading-rustdoc] returns a `Future` that completes when the reading is ready.
The split between [`Sensor::trigger_measurement()`][ariel-os-sensors-mod-sensor-trigger-measurement-rustdoc] and [`Sensor::wait_for_reading()`][ariel-os-sensors-mod-sensor-wait-for-reading-rustdoc] makes it possible to trigger parallel measurements while having to await the reading of only one sensor driver instance at a time, avoiding the need for allocating multiple `Future`s.
Note that sensor drivers may internally implement time-based polling instead of relying on sensor device interrupts to determine when samples are available.

When [`Sensor::wait_for_reading()`][ariel-os-sensors-mod-sensor-wait-for-reading-rustdoc] successfully completes, it returns [`Samples`][ariel-os-sensors-mod-sensor-mod-samples-rustdoc], which exposes [`Reading::samples()`][ariel-os-sensors-mod-sensor-mod-samples-samples-rustdoc], allowing to obtain an iterator over pairs of [`ReadingChannel`][ariel-os-sensors-mod-sensor-mod-readingchannel-rustdoc]s and [`Sample`][ariel-os-sensors-mod-sensor-mod-sample-rustdoc]s.
Taking into account the [`ReadingChannel`][ariel-os-sensors-mod-sensor-mod-readingchannel-rustdoc]s is essential to make sense of the [`Sample`][ariel-os-sensors-mod-sensor-mod-sample-rustdoc]s: most importantly, they contain [the scaling value][ariel-os-sensors-mod-sensor-mod-sample-scaling-rustdoc] to use with the [`Sample`][ariel-os-sensors-mod-sensor-mod-sample-rustdoc].
Additionally, [`ReadingChannel`][ariel-os-sensors-mod-sensor-mod-readingchannel-rustdoc]'s [`Label`][ariel-os-sensors-mod-sensor-mod-label-rustdoc]s allow associating each [`Sample`][ariel-os-sensors-mod-sensor-mod-sample-rustdoc] to the exact physical quantity it relates to, and filtering them if needed.
The [`ReadingChannel`][ariel-os-sensors-mod-sensor-mod-readingchannel-rustdoc]s of a sensor driver are fixed and can be obtained without triggering a measurement using [`Sensor::reading_channels()`][ariel-os-sensors-mod-sensor-readingchannels-rustdoc].

Separately from the [`ReadingChannel`][ariel-os-sensors-mod-sensor-mod-readingchannel-rustdoc]s, each [`Sample`][ariel-os-sensors-mod-sensor-mod-sample-rustdoc] comes with [`SampleMetadata`][ariel-os-sensors-mod-sensor-mod-samplemetadata-rustdoc], which provides important auxiliary information about the sample, which may change between individual measurements.
For instance, the reading channel may be disabled, either temporarily or by configuration, in which case the sample's value does not have meaning.
Additionally, it also provides the sample's accuracy, which the sensor driver may determine for each individual sample.

The API is designed not to use floats, so that it can easily be used on the smallest microcontrollers.
Floats can still be constructed and used if needed, especially for easily displaying values.

[i2c-book]: ./i2c.md
[spi-book]: ./spi.md
[ariel-os-sensors-mod-registry-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/static.REGISTRY.html
[ariel-os-sensors-mod-registry-sensors-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/registry/struct.Registry.html#method.sensors
[ariel-os-sensors-mod-sensor-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/trait.Sensor.html
[ariel-os-sensors-mod-sensor-trigger-measurement-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/trait.Sensor.html#tymethod.trigger_measurement
[ariel-os-sensors-mod-sensor-wait-for-reading-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/trait.Sensor.html#tymethod.wait_for_reading
[ariel-os-sensors-mod-sensor-categories-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/trait.Sensor.html#tymethod.categories
[ariel-os-sensors-mod-sensor-label-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/trait.Sensor.html#tymethod.label
[ariel-os-sensors-mod-sensor-mod-samples-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/sensor/struct.Samples.html
[ariel-os-sensors-mod-sensor-mod-sample-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/sensor/struct.Sample.html
[ariel-os-sensors-mod-sensor-mod-sample-scaling-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/sensor/struct.Sample.html#scaling
[ariel-os-sensors-mod-sensor-mod-readingchannel-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/sensor/struct.ReadingChannel.html
[ariel-os-sensors-mod-sensor-mod-samples-samples-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/sensor/struct.Samples.html#method.samples
[ariel-os-sensors-mod-sensor-readingchannels-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/trait.Sensor.html#tymethod.reading_channels
[ariel-os-sensors-mod-sensor-mod-label-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/enum.Label.html
[ariel-os-sensors-mod-sensor-mod-samplemetadata-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/sensors/sensor/enum.SampleMetadata.html
[sbd-book]: ./structured-board-descriptions.md
