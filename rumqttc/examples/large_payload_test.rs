use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

fn main() {
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "broker.emqx.io", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_max_packet_size(10 * 1024, 128 * 1024);
    mqttoptions.set_clean_session(false);

    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt/#", QoS::AtMostOnce).unwrap();
    let mut client2 = client.clone();
    thread::spawn(move || {
        [10, 128, 256, 65535, 256, 128].iter().for_each(|count| {
            let result = client.publish(
                "hello/rumqtt/short",
                QoS::AtLeastOnce,
                false,
                vec![1; *count as usize],
            );
            match result {
                Ok(_) => {}
                Err(e) => println!("Publish Error: {e}"),
            }
        });
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        [16, 16, 16, 16, 16, 16, 16].iter().for_each(|count| {
            let result = client2.publish(
                "hello/rumqtt/short",
                QoS::AtLeastOnce,
                false,
                vec![1; *count as usize],
            );
            match result {
                Ok(_) => {}
                Err(e) => println!("Publish Error: {e}"),
            }
        });
    });

    for (_i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(event) => match event {
                Event::Incoming(Incoming::Publish(publish)) => {
                    println!("Publish: {publish:?}")
                }
                _ => {}
            },
            Err(e) => {
                println!("Recv Error: {e}")
            }
        }
    }
}
