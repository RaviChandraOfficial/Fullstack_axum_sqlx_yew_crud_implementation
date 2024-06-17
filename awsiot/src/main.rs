use aws_iot_device_sdk_rust::{async_event_loop_listener, AWSIoTAsyncClient, AWSIoTSettings};
use rumqttc::{self, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use calamine::{open_workbook, Data, Reader, Xlsx};

#[derive(Serialize, Deserialize)]
pub struct Message {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // Specify the path to your Excel file
    let path = "excel path";

    // Open the workbook
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open Excel file");

    // Specify the sheet name
    let sheet_name = "Sheet1"; // Replace with your sheet name

    
    // Read data from the Excel sheet
    let mut excel_data = String::new();

    // Get the range of the specified sheet and extract data
    match workbook.worksheet_range(sheet_name) {
        Ok(range) => {
            println!("Data in '{}':", sheet_name);
            for row in range.rows() {
                for cell in row {
                    print!("{:?}\t", cell);
                    excel_data.push_str(&format!("{:?}\t", cell));
                }
                println!();
            }

            // Provide some statistics
            let total_cells = range.get_size().0 * range.get_size().1;
            let non_empty_cells: usize = range.used_cells().count();
            println!(
                "Found {} cells in '{}', including {} non-empty cells",
                total_cells, sheet_name, non_empty_cells
            );
            // alternatively, we can manually filter rows
            assert_eq!(
                non_empty_cells,
                range.rows()
                    .flat_map(|r| r.iter().filter(|&c| c != &Data::Empty))
                    .count()
            );
        }
        Err(_) => println!("Sheet '{}' does not exist in the workbook", sheet_name),
        Err(e) => println!("Error reading the workbook: {:?}", e),
    }


    let aws_settings = AWSIoTSettings::new(
        "thing name".to_owned(),
        "amazon root ca".to_owned(),
        "certificate".to_owned(),
        "privatekey".to_owned(),
        "amazon end point".to_owned(),
        None,
    );

    let (iot_core_client, eventloop_stuff) = AWSIoTAsyncClient::new(aws_settings).await?;

    iot_core_client.subscribe("test1234".to_string(), QoS::AtMostOnce).await.unwrap();
    iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, "hey").await.unwrap();

    let mut receiver1 = iot_core_client.get_receiver().await;
    let mut receiver2 = iot_core_client.get_receiver().await;

    let recv1_thread = tokio::spawn(async move {
        loop {
            match receiver1.recv().await {
                Ok(event) => {
                    match event {
                        Packet::Publish(p) => {
                            println!("Received message {:?} on topic: {}", p.payload, p.topic)
                        }
                        _ => println!("Got event on receiver1: {:?}", event),
                    }
                }
                Err(_) => (),
            }
        }
    });

    let recv2_thread = tokio::spawn(async move {
        loop {
            match receiver2.recv().await {
                Ok(event) => println!("Got event on receiver2 : {:?}", event),
                Err(_) => (),
            }
        }
    });

    let publish = tokio::spawn(async move {
        loop {
            // let message = Message {
            //     text: "hello santosh".to_string(),
            // };
            // Publish the Excel data to AWS IoT
            let message = Message { text: excel_data.clone() };
            let json_message = serde_json::to_string(&message).unwrap();
            iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, &*json_message).await.unwrap();
        }
    });
    
    let listen_thread = tokio::spawn(async move {
        async_event_loop_listener(eventloop_stuff).await.unwrap();
    });

    tokio::join!(
        recv1_thread, 
        // recv2_thread, 
        // listen_thread,
        //  publish
        );

    Ok(())
}


