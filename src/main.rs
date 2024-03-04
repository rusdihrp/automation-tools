use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::CanFrame;
use pcan_basic::socket::{Baudrate, MessageType, SendCan};
use std::time::Duration;
use std::thread;

#[tokio::main]
async fn main() {
    //UsbBus::can0
    let usb_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud250K) {
        Ok(socket) => socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    // open parked page
    // vcu_status_pkt_1
    let vehicle_state_id = 0x18EFF4FA;
    let value_parked = vec![0x1A];

    let bms_status_pkt_1_id = 0x18EFFAF8;

    let value_current = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let can_frame = CanFrame::new(bms_status_pkt_1_id, MessageType::Extended, &value_current).unwrap();
    let result = usb_socket.send(can_frame);
    if result.is_ok() {
        println!("bms_current_A sent!");
    }
    thread::sleep(Duration::from_millis(50));

    let bms_status_pkt_2_id = 0x18EEFAF8;

    let value_lowest_cell = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x74, 0x0E];
    let can_frame = CanFrame::new(bms_status_pkt_2_id, MessageType::Extended, &value_lowest_cell).unwrap();
    let result = usb_socket.send(can_frame);
    if result.is_ok() {
        println!("bms_lowest_cell sent!");
    }
    thread::sleep(Duration::from_millis(50));

    // obc_status_pkt_1
    let obc_status_pkt_1_id = 0x18FFF8F2;

    let mut value_volt;
    let mut can_frame;
    let mut result; 
    // loop {
    for i in 0..=50 {
        value_volt = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xDC, 0x00];
        value_volt[7] = (value_volt[7] as u8).wrapping_add(i as u8) as u8;
        can_frame = CanFrame::new(obc_status_pkt_1_id, MessageType::Extended, &value_volt).unwrap();
        result = usb_socket.send(can_frame);
        if result.is_ok() {
            println!("charging limit sent!");
        }
        thread::sleep(Duration::from_millis(50));
    }
    thread::sleep(Duration::from_millis(8000));

    can_frame = CanFrame::new(vehicle_state_id, MessageType::Extended, &value_parked).unwrap();
    result = usb_socket.send(can_frame);
    if result.is_ok() {
        println!("parked sent!");
    }
    thread::sleep(Duration::from_millis(500));

}
