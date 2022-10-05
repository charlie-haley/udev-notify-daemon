extern crate notify_rust;
extern crate tokio_udev;
extern crate futures_util;

use futures_util::future::ready;
use futures_util::stream::StreamExt;
use std::convert::TryInto;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder, EventType};
use udev::{Event};
use notify_rust::{Notification, NotificationHandle};

use anyhow::{Result, bail};

#[tokio::main]
async fn main() {
    let builder = MonitorBuilder::new()
        .expect("Couldn't create builder")
        .match_subsystem_devtype("usb", "usb_device")
        .expect("Failed to add filter for USB devices");

    let monitor: AsyncMonitorSocket = builder
        .listen()
        .expect("Couldn't create MonitorSocket")
        .try_into()
        .expect("Couldn't create AsyncMonitorSocket");

    monitor
        .for_each(|event| {
            if let Ok(event) = event {
                match event.event_type() {
                    EventType::Add => {
                        let mut msg = get_property_value(&event, "ID_MODEL_FROM_DATABASE");
                        if msg.is_err() {
                            msg = get_property_value(&event, "ID_MODEL");
                        }
                        notify("USB Device connected", &msg.unwrap(), 5000).expect("failed to send dbus notification");
                    },
                    EventType::Remove => {
                        notify("USB Device removed", "", 5000).expect("failed to send dbus notification");
                    },
                    _ => {}
                }
            }
            ready(())
        })
        .await
}

fn notify(summary: &str, body: &str, timeout: i32) -> Result<NotificationHandle, notify_rust::error::Error>{
    return Notification::new()
    .summary(summary)
    .body(body)
    .timeout(timeout)
    .show();
}

fn get_property_value(event: &Event, name: &str) ->  Result<String>{
    for property in event.properties() {
        if property.name() == name {
            let result = property.value().to_str().unwrap().to_string();
            return Ok(result);
        }
    };
    bail!("property {} not found on device", name);
}
