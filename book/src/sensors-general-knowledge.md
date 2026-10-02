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
<svg xmlns="http://www.w3.org/2000/svg" width="704" height="80" class="svgbob"><style>.svgbob line, .svgbob path, .svgbob circle, .svgbob rect, .svgbob polygon {
    stroke: black;
    stroke-width: 2;
    stroke-opacity: 1;
    fill-opacity: 1;
    stroke-linecap: round;
    stroke-linejoin: miter;
}
.svgbob text {
    white-space: pre;
    fill: black;
    font-family: Iosevka Fixed, monospace;
    font-size: 14px;
}
.svgbob rect.backdrop {
    stroke: none;
    fill: white;
}
.svgbob .broken {
    stroke-dasharray: 8;
}
.svgbob .filled {
    fill: black;
}
.svgbob .bg_filled {
    fill: white;
    stroke-width: 1;
}
.svgbob .nofill {
    fill: white;
}
.svgbob .end_marked_arrow {
    marker-end: url(#arrow);
}
.svgbob .start_marked_arrow {
    marker-start: url(#arrow);
}
.svgbob .end_marked_diamond {
    marker-end: url(#diamond);
}
.svgbob .start_marked_diamond {
    marker-start: url(#diamond);
}
.svgbob .end_marked_circle {
    marker-end: url(#circle);
}
.svgbob .start_marked_circle {
    marker-start: url(#circle);
}
.svgbob .end_marked_open_circle {
    marker-end: url(#open_circle);
}
.svgbob .start_marked_open_circle {
    marker-start: url(#open_circle);
}
.svgbob .end_marked_big_open_circle {
    marker-end: url(#big_open_circle);
}
.svgbob .start_marked_big_open_circle {
    marker-start: url(#big_open_circle);
}<!--separator--></style><defs><marker id="arrow" viewBox="-2 -2 8 8" refX="4" refY="2" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><polygon points="0,0 0,4 4,2 0,0"></polygon></marker><marker id="diamond" viewBox="-2 -2 8 8" refX="4" refY="2" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><polygon points="0,2 2,0 4,2 2,4 0,2"></polygon></marker><marker id="circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="2" class="filled"></circle></marker><marker id="open_circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="2" class="bg_filled"></circle></marker><marker id="big_open_circle" viewBox="0 0 8 8" refX="4" refY="4" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><circle cx="4" cy="4" r="3" class="bg_filled"></circle></marker></defs><rect class="backdrop" x="0" y="0" width="704" height="80"></rect><rect x="4" y="8" width="80" height="48" class="solid nofill" rx="4"></rect><text x="18" y="28" >Sensing</text><text x="18" y="44" >element</text><rect x="124" y="8" width="72" height="48" class="solid nofill" rx="4"></rect><text x="138" y="28" >Analog</text><text x="146" y="44" >AAF</text><rect x="236" y="8" width="48" height="48" class="solid nofill" rx="4"></rect><text x="250" y="28" >ADC</text><rect x="324" y="8" width="80" height="48" class="solid nofill" rx="4"></rect><text x="338" y="28" >Digital</text><text x="354" y="44" >AAF</text><rect x="444" y="8" width="112" height="48" class="solid nofill" rx="4"></rect><text x="458" y="28" >Downsampler</text><rect x="596" y="8" width="96" height="48" class="solid nofill" rx="4"></rect><text x="618" y="28" >Digital</text><text x="610" y="44" >interface</text><line x1="96" y1="24" x2="104" y2="24" class="solid"></line><polygon points="104,20 112,24 104,28" class="filled"></polygon><line x1="208" y1="24" x2="216" y2="24" class="solid"></line><polygon points="216,20 224,24 216,28" class="filled"></polygon><line x1="296" y1="24" x2="304" y2="24" class="solid"></line><polygon points="304,20 312,24 304,28" class="filled"></polygon><line x1="416" y1="24" x2="424" y2="24" class="solid"></line><polygon points="424,20 432,24 424,28" class="filled"></polygon><line x1="568" y1="24" x2="576" y2="24" class="solid"></line><polygon points="576,20 584,24 576,28" class="filled"></polygon></svg>
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
However, in ISO 5725-1, accuracy instead encompasses *both* trueness and [precision].
Repeating the measurement does not help improve the trueness, but some systematic errors may be eliminated through calibration.
In the context of sensors, a common source of inaccuracy is temperature: that is why [many IC sensors embed temperature sensing elements] to allow for temperature compensation, either in the sensor itself or in software.
