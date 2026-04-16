# Glossary

> [!NOTE]
> This glossary does not intend to provide complete definitions of each term.
> Instead, it aims to clarify and emphasize the differences between related terms.
> Links are provided when relevant to learn more about each concept.

<dl>
  <dt id="ariel-os-hals">Ariel OS HALs:</dt>
  <dd>
    Currently the following crates:
    <a href="https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os_esp/index.html"><code>ariel-os-esp</code></a>,
    <a href="https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os_nrf/index.html"><code>ariel-os-nrf</code></a>,
    <a href="https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os_rp/index.html"><code>ariel-os-rp</code></a>, and
    <a href="https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os_stm32/index.html"><code>ariel-os-stm32</code></a>.
  </dd>

  <dt id="arm"><a href="https://en.wikipedia.org/wiki/ARM_architecture_family">ARM (or Arm)</a>:</dt>
  <dd>A family of instruction set architectures.</dd>

  <dt id="arm-holdings"><a href="https://en.wikipedia.org/wiki/Arm_Holdings">Arm (Arm Holdings)</a>:</dt>
  <dd>The company behind the ARM architecture family.</dd>

  <dt id="chip"><a href="https://en.wikipedia.org/wiki/Integrated_circuit">Chip</a>:</dt>
  <dd>An integrated circuit (IC).</dd>

  <dt id="cortex-m"><a href="https://en.wikipedia.org/wiki/ARM_Cortex-M">Cortex-M</a>:</dt>
  <dd>A family of 32-bit processor implementations from Arm.</dd>

  <dt id="esp32"><a href="https://en.wikipedia.org/wiki/ESP32">ESP32</a>:</dt>
  <dd>
    A family of 32-bit microcontrollers from Espressif.
    Its older microcontrollers are based on the Xtensa architecture, while newer ones use RISC-V.
    Can also specifically refer to the eponymous ESP32 microcontroller.
  </dd>

  <dt id="embassy"><a href="https://embassy.dev/">Embassy</a>:</dt>
  <dd>
    A software project developing HALs for multiple microcontroller families (e.g.,
    <a href="https://docs.embassy.dev/embassy-nrf/"><code>embassy-nrf</code></a>,
    <a href="https://docs.embassy.dev/embassy-rp/"><code>embassy-rp</code></a>,
    <a href="https://docs.embassy.dev/embassy-stm32/"><code>embassy-stm32</code></a>),
    along with other components (e.g.,
    <a href="https://docs.rs/embassy-executor/latest/embassy_executor/"><code>embassy-executor</code></a>,
    <a href="https://docs.rs/embassy-time/latest/embassy_time/"><code>embassy-time</code></a>).
  </dd>

  <dt id="embassy-style-hals">Embassy-style HALs:</dt>
  <dd>
    Currently the following crates:
    <a href="https://docs.embassy.dev/embassy-nrf/"><code>embassy-nrf</code></a>,
    <a href="https://docs.embassy.dev/embassy-rp/"><code>embassy-rp</code></a>,
    <a href="https://docs.embassy.dev/embassy-stm32/"><code>embassy-stm32</code></a>,
    <a href="https://docs.espressif.com/projects/rust/esp-hal/latest/"><code>esp-hal</code></a>.
    In particular, these HALs feature <a href="./application.md#obtaining-peripheral-access">peripheral ZSTs</a> modeling compile-time
  exclusive access, that drivers require for instantiation.
  </dd>

  <dt id="esp-hal"><a href="https://docs.espressif.com/projects/rust/esp-hal/latest/">esp-hal</a>:</dt>
  <dd>A HAL for ESP32 microcontrollers, developed by their manufacturer Espressif.</dd>

  <dt id="hal">HAL (Hardware Abstraction Layer):</dt>
  <dd>A software layer that makes specific pieces of hardware (e.g., microcontroller peripherals) easier to use, by hiding some of their details.</dd>

  <dt id="mcu"><a href="https://en.wikipedia.org/wiki/Microcontroller">MCU</a>:</dt>
  <dd>
    A microcontroller.
    Contains a processor, memory, and peripherals to interact with the outside world.
  </dd>

  <dt id="nrf">nRF:</dt>
  <dd>A family of 32-bit microcontrollers developed by Nordic Semiconductor.</dd>

  <dt id="raspberry-pi">Raspberry Pi:</dt>
  <dd>
    Can refer to the <a href="https://en.wikipedia.org/wiki/Raspberry_Pi">family of single-board computers</a>
    or to <a href="https://en.wikipedia.org/wiki/Raspberry_Pi_Holdings">Raspberry Pi Holdings</a>, the company that manufactures
    them as well as RP microcontrollers. Sometimes abbreviated to “RPi.“
  </dd>

  <dt id="risc-v"><a href="https://en.wikipedia.org/wiki/RISC-V">RISC-V</a>:</dt>
  <dd>An open, royalty-free instruction set architecture.</dd>

  <dt id="rp">RP:</dt>
  <dd>A family of 32-bit microcontrollers designed by Raspberry Pi, which includes the RP2040 and RP2350 microcontrollers.</dd>

  <dt id="soc"><a href="https://en.wikipedia.org/wiki/System_on_a_chip">SoC (System on Chip)</a>:</dt>
  <dd>
    Often synonymous with microcontroller in practice, at least in the embedded domain.
    Sometimes used to emphasize the presence of a sizable analog component (usually for radio).
  </dd>

  <dt id="stm32"><a href="https://en.wikipedia.org/wiki/STM32">STM32</a>:</dt>
  <dd>A family of 32-bit microcontrollers developed by STMicroelectronics.</dd>

  <dt id="uart"><a href="https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter">UART (Universal Asynchronous Receiver-Transmitter)</a>:</dt>
  <dd>
    A (microcontroller) peripheral.
    By extension, can also refer to the link-layer protocol it implements.
    Sometimes incorrectly used to refer to serial communication over USB (e.g., instead of <a href="https://en.wikipedia.org/wiki/USB_communications_device_class#Abstract_Control_Model">USB CDC-ACM</a>).
  </dd>

  <dt id="usb-cdc-acm"><a href="https://en.wikipedia.org/wiki/USB_communications_device_class#Abstract_Control_Model">USB CDC-ACM</a>:</dt>
  <dd>A standard protocol implementing serial communication over USB.</dd>

  <dt id="usb-cdc-ncm"><a href="https://en.wikipedia.org/wiki/Ethernet_over_USB#Protocols">USB-CDC-NCM</a>:</dt>
  <dd>A standard protocol implementing Ethernet over USB.</dd>

  <dt id="xtensa"><a href="https://en.wikipedia.org/wiki/Tensilica#Xtensa_configurable_cores">Xtensa</a>:</dt>
  <dd>A family of instruction set architectures, used in some ESP32 microcontrollers.</dd>
</dl>
