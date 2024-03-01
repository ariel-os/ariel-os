# Manifesto

We must (re)write the foundations of system software on more solid ground. Even [POTUS roots for it](https://www.whitehouse.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf)! For the Internet of Things, our cyberphysical world under continuous development, **we  need stronger cybersecurity**. 

So what does it mean for the main embedded operating systems targeting networked microcontrollers (Zephyr, FreeRTOS, or [RIOT](https://github.com/RIOT-OS/RIOT/)) which are primarily written in C? 

Well, how about using instead a memory-safe language, [Rust](https://www.rust-lang.org/), as foundation. And for good measure, how about formal verification for the critical modules cherry-picked incrementally, along the way. Implementing the above guidelines is exactly the mission of [RIOT-rs](https://github.com/future-proof-iot/RIOT-rs): collaboration to produce open source re-write(s) of RIOT core modules providing a basis for next-level cybersecurity in IoT.

Curious about more details on principles driving RIOT-rs development? Here you go. 

*Southbound*, RIOT-rs builds on top of [Embassy](https://github.com/embassy-rs/embassy) which provides an awesome open source HAL for a large variety of low-power IoT hardware, written in Rust. 

*Northbound*, RIOT-rs provides high-level APIs working nicely across all supported hardware (work-in-progress) for generic sensors/actuators interaction (such as [SAUL](https://api.riot-os.org/group__drivers__saul.html)) and for generic network I/O (such as [sock](https://api.riot-os.org/group__net__sock.html)). 

*At the core*, RIOT-rs provides a programming framework for both async and blocking (threads with priorities) paradigms and convenient scaffolding to bundle various the libraries and 3rd party modules you need. 

True to the spirit of RIOT development, RIOT-rs aims at a level of integration and **code portability** such that ~95% of the code is reused as-is on all supported hardware. The targeted **memory footprint** (RAM & Flash) is measured in tens of kBytes. The targeted **power consumption** levels enable applications lasting 1+ years on a small battery.

Last but not least the CI process for each pull-request to RIOT-rs includes extensive and rigorous tests which are automated for all supported configuration and hardware. And for selected modules, a **formal verification workflow** is used based on [hax](https://github.com/hacspec/hax), running directly on the functional Rust.

Long story short: RIOT-rs embodies the developing love affair between RIOT and embedded Rust, fostering stronger IoT cybersecurity. This is a joyous open source community, so... you're welcome to [join us](https://github.com/future-proof-iot/RIOT-rs)! 
