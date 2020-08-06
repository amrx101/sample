use rumqtt::{MqttClient, MqttOptions, QoS};
use std::{thread, time::Duration};
use rand::Rng;

fn main() {

    /// place the  certificates  in this directory itself.
    let ca = include_bytes!("broker-ca.pem").to_vec();x
    let client = include_bytes!("new_c.cert.pem").to_vec();
    let client_key = include_bytes!("new_c.key.pem").to_vec();

    /// change localhost to actual domain.
    let mqtt_options = MqttOptions::new(
        "test-pubsub1", "localhost", 443
    ).
    set_ca(ca).
    set_client_auth(client, client_key);

    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
      
    mqtt_client.subscribe("hello/world", QoS::AtLeastOnce).unwrap();
    let sleep_time = Duration::from_secs(1);
    let mut s = String::new();

    for _ in 0..400 {
        s.push_str("hello world");
    }
    thread::spawn(move || {
        for _ in 0..100 {
            // let payload = generate_payload(5000 );
            let payload = s.as_bytes();
            thread::sleep(sleep_time);
            mqtt_client.publish("hello/world", QoS::AtLeastOnce, false, payload).unwrap();
        }
    });

    for notification in notifications {
        println!("{:?}", notification)
    }
}

fn generate_payload(payload_size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let payload: Vec<u8> = (0..payload_size).map(|_| rng.gen_range(0, 255)).collect();
    payload
}