# General Knowledge on Sensors

This page aims to present relevant general knowledge and background on MEMS sensors and other IC sensors, as well as on signal processing and metrology, to make it easier to write sensor drivers and to use them.

### Digital-Output IC Sensors

Digital-output IC sensors are mixed-signal integrated circuits that measure an analog physical quantity and expose the result as digital values.
Many of such sensors are micro-electromechanical sensors (MEMS) sensors.
MEMS technology allows them to be very small in size, and to fit both the sensing element and the digital logic into a single package.
Because MEMS sensors are sensitive to temperature, they often comprise temperature sensing elements for compensation, even if their primary function is not to measure temperature.
These temperature-sensing elements may have an accuracy of only a few degrees, but that is sufficient for temperature compensation.

Digital-output IC sensors use, in the general case, the following architecture:

<!--
.---------.    .--------.    .-----.    .---------.    .-------------.    .-----------.
| Sensing | -> | Analog | -> | ADC | -> | Digital | -> | Downsampler | -> |  Digital  |
| element |    |  AAF   |    |     |    |   AAF   |    |             |    | interface |
'---------'    '--------'    '-----'    '---------'    '-------------'    '-----------'
-->
<figure>
    <!-- The SVG is included inline so its content can be announced by screen readers. -->
    <!-- Rendered from the diagram above with Svgbob https://github.com/ivanceras/svgbob -->
    {{#include ../static/sensor-architecture.svg }}
    <figcaption style="text-align: center">
        Typical architecture of a sensor<br>
        (AAF: anti-aliasing filter; ADC: analog-to-digital converter)
    </figcaption>
</figure>

<!-- NOTE: For instance, ADXL362 has a variable sample rate, and adjustable analog AAF's cutoff frequency. -->
Some sensors' datasheets do describe their architecture, but sometimes omit some elements, especially the filters.

### Anti-Aliasing Filters and Bandwidth

In continuous mode, the rate at which the sensor's *digital interface* refreshes its values is called the output data rate (ODR) and is generally configurable.
Depending on the sensor, the ODR may only downsample the *digital* output, or it may change the sample rate of the ADC.
If the sample rate of the ADC is variable, there must generally be an *analog* low-pass filter (LPF) upstream of it, required to avoid aliasing, whose cutoff frequency must be adjusted based on the sample rate.
In some cases, the low-pass filter may not be an electronic filter: for instance the thermal inertia of a temperature sensor's package acts as a low-pass filter and might be sufficient.
In addition, if there is a digital downsampler, that reduces the rate of digital samples output by the sensor, there must also be a *digital* low-pass filter to avoid aliasing.

> [!NOTE]
> A downsampler is also called a decimator, which may or may not also include an anti-aliasing filter.

The anti-aliasing filters (AAF) are required because, if the signal contains frequencies higher than half the sampling rate (i.e., higher than the Nyquist frequency), aliases of these will appear as extraneous low frequencies in the output.

Being low-pass filters, these filters also remove high-frequency components, which may be desired if they constitute noise.
On the other hand, if the signal is high bandwidth, i.e., if high frequencies are actually desired, both the cutoff frequencies of these filters and the sampling rate must be kept high enough, to preserve these high frequencies through the chain, according to the Nyquist–Shannon sampling theorem.

### Precision, Effective Resolution, and Oversampling

Precision refers to how close measurement results of a sensor are to each other, for a given true value.
It characterizes random errors marring the measurement results.
It is commonly modeled as noise in datasheets.

A related concept is the effective resolution, which is essentially the number of bits of the output that are not marred by random errors.
It is not to be confused with just "resolution," which is simply the number of bits of the output.
For instance, a 12-bit ADC always generates 12 bits, but fewer than that are likely noise-free.

To improve the precision of sensors, a common technique is oversampling, which involves taking repeated measurements and averaging them, to cancel out some of the noise and therefore increase the effective resolution.
Many sensors support built-in oversampling, and can be set to automatically trigger multiple measurements and average them.
Oversampling can also be implemented in software, on the microcontroller.
A potential drawback of averaging is that it behaves as a low-pass filter, and therefore reduces the sensor bandwidth (i.e., its ability to measure high frequencies), which may be undesired.

### Sensitivity

Sensitivity, as commonly used in datasheets, is the smallest change in value that can be reflected on the output.
It is related to the resolution: a higher resolution allows for a higher sensitivity.
As the resolution (i.e., the number of bits in the output) is generally fixed, sensors often have a setting to reduce the sensitivity to allow outputting a greater range of values.
This may be implemented in the analog domain using a programmable-gain amplifier (PGA) upstream of the ADC.
A better term for this meaning of sensitivity would be *responsivity*.

> [!NOTE]
> Sensitivity is also sometimes defined as taking precision into account, in which case a higher *effective* resolution is required for a higher sensitivity.

### Trueness and Accuracy

Trueness has been introduced by ISO 5725-1 to refer to how close the mean of measurement results of a sensor is to the true value.
It characterizes systematic errors and is generally described as an offset in datasheets.
Trueness is also commonly referred to as accuracy.
However, in ISO 5725-1, accuracy instead encompasses *both* trueness and [precision](#precision-effective-resolution-and-oversampling).
Repeating the measurement does not help improve the trueness, but some systematic errors may be eliminated through calibration.
In the context of sensors, a common source of inaccuracy is temperature: that is why [many IC sensors embed temperature sensing elements](#digital-output-ic-sensors) to allow for temperature compensation, either in the sensor itself or in software.
