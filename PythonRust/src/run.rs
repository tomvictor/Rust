use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;
use std::thread;

pub fn run(){
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "mqtt.eclipseprojects.io", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    
    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
    thread::spawn(move || for i in 0..10 {
       client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).unwrap();
       thread::sleep(Duration::from_millis(1000));
    });
    
    // Iterate to poll the eventloop for connection progress
    for (i, notification) in connection.iter().enumerate() {
        println!("Notification = {:?}", notification);
    }
}
