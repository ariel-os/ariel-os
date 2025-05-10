# Ariel OS Roadmap

This document contains the roadmap for Ariel OS.
It provides a high-level overview of the project's strategy
and what the project wants to achieve in the next years.

Please take a look at the [contributing guidelines]
when you want to start contributing to one of these goals.

## Development tools & Documentation

### Extensive Laze documentation

As Laze is a core component of the Ariel OS workflow,
extensive documentation for both users and developers is required.
The current documentation of Laze is lacking, with multiple blank pages in the
manual.

## OS functionality

### Common Sensor Interface

A consistent access method for sensors provides a convenient and portable way
for users to access readings.
This allows for generic access to different sensors and makes it easy to export
these sensor readings via different protocols.

See also: https://github.com/ariel-os/ariel-os/pull/474

### Over the air updates

OTA updates are required to fix bugs, including vulnerabilities, on devices
deployed in the field.

### Stack usage insight

Insights and metrics on stack usage in Ariel OS provides the developer with
valuable information on memory usage and where it can be decreased.

In the future this can also be extended with automated tuning and verification.

## Hardware Support

### Native hosted mode

Running Ariel OS as native process on the developers computer without requiring
extra hardware provides a useful way to test and debug OS modules.

See also: 
 - https://github.com/ariel-os/ariel-os/issues/617
 - https://github.com/ariel-os/ariel-os/pull/647

### Stack overflow detection

Detecting stack overflows in threads protect against vulnerabilities and other
bugs around stack overflows.

### Sandboxed thread mode

Fully sandboxing a thread via hardware mechanisms allow for isolated threads.
This improves the resiliency against memory bugs and can protect critical OS
threads against potentially untrusted threads.

## RIOT-c support

Improving compatibility with RIOT APIs provide cross OS compatibility.

## Basic (Layer<5) networking support

### Bluetooth

Bluetooth provides a low power mechanism via which to interface
with other computers such as smartphones.
It allows for exposing multiple standardized functionalities
such as sensors and human interface devices.

### IEEE 802.15.4 / 6LoWPAN

This provides a standardized multi-hop network mechanism
geared towards low power and low throughput devices.

### LoRa

LoRa support provides a mechanism through which simple low power sensor devices
can be deployed in the field and report their measurements to a central
authority.

## Extended (Layer>4) networking + security 

### MQTT client

MQTT is a popular publish-subscribe protocol for communicating with constrained
devices. One of the users is [Home-Assistant] where it is used extensively for
provisioning and sensor/actuator interaction

### TLS / DTLS

TLS and DTLS integration provide easy to use cryptographic primitives for both
Ariel OS modules and outside modules provided by developers. Integration
supports this by making it as easy as possible to add security to communication
protocols.

## Device Life cycle

### On-boarding mechanism

An on-boarding mechanism to enter a device into a ecosystem helps
developers with orchestrating multiple devices in a larger multi-device setup.
Multiple mechanisms are possible for on-boarding a device

#### TOFU

Trust On First Use is an authentication scheme where a user must manually verify
the device on first use. Its security level is not ideal, but it is simple to use
across a wide range of device flashing mechanisms.

#### Zero-Touch

To avoid the security implications of TOFU, the existing connection between the
programmer and the device can be utilized in some cases to already establish
some mutual trust. Where available, this provides an even simpler onboarding
experience than TOFU.

#### Explicit configuration

The on-boarding mechanism might not be completely defined at compile time.
Exposing a configuration mechanism through which the device can be explicitly
on-boarded in a setup.

See also: [This fedi thread](https://chaos.social/@chrysn/112803944916314295)

## Extend embassy integration

### Low power

Low-power support enables the use of Ariel OS on battery-powered devices.

### Multiple Timers

## CI

### Hardware-in-the-Loop testing

HiL testing can catch hardware-related errors during developments.

## Formal Verification

Formal verification to prove panic free operation increases reliability and
provides strong guarantees for users of Ariel OS.

see also: https://github.com/ariel-os/ariel-os/issues/140

[contributing guidelines]: ./CONTRIBUTING.md
[Home-Assistant]: https://github.com/home-assistant/core/
