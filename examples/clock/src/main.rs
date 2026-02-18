#![no_main]
#![no_std]

use embassy_futures::join::join;
use embassy_futures::select::select3;
use trouble_host::prelude::*;

use ariel_os::debug::log::*;

use ::defmt::{self, Format};

/// A representation of current time.
///
/// This should, as this develops, move towards a more universal format than the current BLE
/// inspired format, but right now, this will do. (A likely change is that it'll go to some integer
/// time stamp; formatting can then still save the costly divisions by memoization).
#[derive(Debug)]
struct CurrentTime {
    current_time: [u8; 10],
    time_offset: [u8; 2],
}

impl Format for CurrentTime {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(
            f,
            "Time {:04}-{:02}-{:02} {:02}:{:02}:{:02}+{}/256 at time zone {}",
            // As soon as we move to a better CurrentTime type, this parsing should happen somewhere
            // else.
            u16::from_le_bytes(self.current_time.as_chunks().0[0]),
            self.current_time[2],
            self.current_time[3],
            self.current_time[4],
            self.current_time[5],
            self.current_time[6],
            // ignoring day of week
            self.current_time[8],
            self.time_offset,
        )
    }
}

#[derive(Format)]
enum BleGetTimeError<BleErr> {
    /// The service or any needed characteristic were not present.
    NotPresent,
    /// A high-level error occurred (such as a characteristic not having the right length)
    HighLevelError,
    /// A BLE error occurred
    Ble(BleHostError<BleErr>),
}

impl<T> From<BleHostError<T>> for BleGetTimeError<T> {
    fn from(value: BleHostError<T>) -> Self {
        BleGetTimeError::Ble(value)
    }
}

/// Collection of GATT services offered by the device
// FIXME: Can we gather OS-wide which services we want to provide?
#[gatt_server]
struct Server {
    _dev_info: DevInfo,
}

/// Device Information Service
// FIXME: This is one of the services that the OS should offer to throw in.
#[gatt_service(uuid = service::DEVICE_INFORMATION)]
struct DevInfo {
    // Manufacturer Name String
    #[characteristic(uuid = "2a29", read, value = *b"Ariel OS")]
    manufacturer: [u8; 8],
}

#[ariel_os::task(autostart)]
async fn main() {
    let stack = ariel_os::ble::ble_stack().await;

    let Host {
        peripheral, runner, ..
    } = stack.build();

    // Actually, we don't expect either to terminate (if anything, they panic, but that should(!)
    // only be due to implementation errors, not due to to anything a peer can do).
    join(ble_task(runner), ble_loop(peripheral, stack)).await;
}

// FIXME: The OS should run this, ideally as an own task (which only Ariel can do, as it can name
// the type of the runner)
async fn ble_task<C: Controller>(mut runner: Runner<'_, C>) -> !
where
    C::Error: Format,
{
    loop {
        if let Err(e) = runner.run().await {
            panic!("[ble_task] error: {:?}", e);
        }
        debug!("[ble_task] Terminated successfully; restarting.");
    }
}

async fn ble_loop<'c, C: Controller>(
    mut peripheral: Peripheral<'c, C>,
    stack: &'c Stack<'c, C>,
) -> !
where
    C::Error: Format,
{
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "Ariel Clock",
        appearance: &appearance::CLOCK,
    }))
    .unwrap();

    loop {
        match advertise("Ariel Clock", &mut peripheral).await {
            Ok(conn) => {
                let servconn = conn
                    .clone()
                    .with_attribute_server(&server)
                    .expect("It's not C dependent so probably no user-data-dependent error?");
                let servertask = gatt_events_task(&servconn);
                let client = GattClient::<'_, _, 1>::new(&stack, &conn).await.unwrap();
                let client_task = client.task();
                // FIXME: Refactor to make clock run all the time (but right now we only print on
                // update anyway)
                let closed = select3(servertask, client_task, async {
                    match look_for_time(&client).await {
                        Ok(time) => {
                            println!("Got time: {:?}", time);
                            println!(
                                "For us, that was {:?}s uptime",
                                ariel_os::time::Instant::now().as_secs()
                            );
                        }
                        Err(BleGetTimeError::NotPresent) => debug!("Peer did not offer CTS"),
                        Err(BleGetTimeError::HighLevelError) => {
                            warn!("Peer implemented CTS erroneously")
                        }
                        Err(BleGetTimeError::Ble(_)) => {
                            warn!("BLE error produced while attempting to read time")
                        }
                    }
                    // Yeah the clock task is done, but if we returned here, we'd have to take care
                    // of shutting down.
                    core::future::pending::<()>().await;
                    Ok::<_, BleHostError<C::Error>>(())
                })
                .await;
                match closed {
                    embassy_futures::select::Either3::First(Err(e)) => {
                        warn!("[adv] Error from GATT server task: {:?}", e)
                    }
                    embassy_futures::select::Either3::Second(Err(e)) => {
                        warn!("[adv] Error from GATT client task: {:?}", e)
                    }
                    embassy_futures::select::Either3::Third(Err(e)) => {
                        warn!("[adv] Error from GATT client program: {:?}", e)
                    }
                    _ => (),
                }
            }
            Err(e) => panic!("[adv] error: {:?}", e),
        }
    }
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, 'b, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
) -> Result<Connection<'a>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            // FIXME: Does this even make sense to advertise? Can we advertise "please connect so I
            // can run Current Time Service"?
            AdStructure::ServiceUuids16(&[service::DEVICE_INFORMATION.to_le_bytes()]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )
    .expect("Constant size array too small");
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..],
                scan_data: &[],
            },
        )
        .await?;
    debug!("[adv] advertising");
    let conn = advertiser.accept().await?;
    debug!("[adv] connection established");
    Ok(conn)
}

/// Process incoming events.
///
/// This does not apply any business logic, it merely ensures that the events are accepted and thus
/// responses are sent.
async fn gatt_events_task(conn: &GattConnection<'_, '_>) -> Result<(), Error> {
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                debug!("[gatt] disconnected: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => match event {
                Ok(event) => {
                    // All our own GATT server properties are static-only, we don't have actual
                    // handling to do.
                    //
                    // (More realistic clocks might actually have BLE configurable properties, such
                    // as volume control for any bell tower sounds, or maybe brightness/contrast if
                    // we pretend that the Electronic Shelf Label service applies).

                    match event.accept() {
                        Ok(reply) => {
                            reply.send().await;
                        }
                        Err(e) => warn!("[gatt] error sending response: {:?}", e),
                    }
                }
                Err(e) => warn!("[gatt] error processing event: {:?}", e),
            },
            _ => {}
        }
    }
    trace!("[gatt] task finished");
    Ok(())
}

/// A client implementation of the Current Time Service
///
/// It is yet to be evaluated whether using the Device Time Service might be preferable, of if they
/// should be combined. The Current Time Service appears to be more widespread at the moment,
/// whereas the Device Time Service is more featureful. The latter can even change the time
/// representation that is to be rendered, it has a more straightforward time+zone representation
/// of 32bit seconds and time zone and DST in the same message, but also has its own weirdnesses
/// (including different epochs indicated by flag bits). There is a mechanism for a
/// DST_Offset_Update around its DTCP, but it is unclear whether that allows scheduling updates for
/// the future (and if not, the Next DST Change feature would still be needed).
async fn look_for_time<'a, C: Controller>(
    client: &GattClient<'a, C, 1>,
) -> Result<CurrentTime, BleGetTimeError<C::Error>> {
    debug!("[cts] Connection available, looking for Current Time Service");

    // not const for lack of into constness or other const conversion
    let current_time_service: Uuid = Uuid::new_short(service::CURRENT_TIME.into());
    let services = client.services_by_uuid(&current_time_service).await?;

    if let Some(service) = services.iter().next() {
        debug!("[cts] Found a time service");
        let c: Characteristic<u8> = client
            .characteristic_by_uuid(
                &service,
                &Uuid::new_short(characteristic::CURRENT_TIME.into()),
            )
            .await
            .expect("But The Spec Said It's Mandatory");
        let c_local: Characteristic<u8> = client
            .characteristic_by_uuid(
                &service,
                &Uuid::new_short(characteristic::LOCAL_TIME_INFORMATION.into()),
            )
            .await
            .expect("Nah I won't go fully time zone blind");

        debug!("[cts] Found characteristic for time");
        let mut buffer = [0; 10];
        let offset1 = client
            .read_characteristic(&c_local, &mut buffer)
            .await
            .unwrap();
        let offset1: [u8; 2] = buffer[..offset1]
            .try_into()
            .map_err(|_| BleGetTimeError::HighLevelError)?;
        let time = client.read_characteristic(&c, &mut buffer).await.unwrap();
        let time: [u8; 10] = buffer[..time]
            .try_into()
            .map_err(|_| BleGetTimeError::HighLevelError)?;
        let offset2 = client
            .read_characteristic(&c_local, &mut buffer)
            .await
            .unwrap();
        let offset2: [u8; 2] = buffer[..offset2]
            .try_into()
            .map_err(|_| BleGetTimeError::HighLevelError)?;
        if offset1 != offset2 {
            panic!("Won't use this b/c didn't get an atomic answer"); // or try again
        }

        // maybe also query the Next DST Change service, or people will wake up unhappy

        Ok(CurrentTime {
            current_time: time,
            time_offset: offset1,
        })
    } else {
        Err(BleGetTimeError::NotPresent)
    }
}
