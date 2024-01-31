use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::CanFrame;
use pcan_basic::socket::{Baudrate, MessageType, SendCan};
use std::time::Duration;
use std::thread;

fn main() {
    let usb_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud250K) {
        Ok(socket) => socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    // vcu_status_pkt_1
    let vehicle_state_id = 0x18EFF4FA;
    // let value_parked = vec![0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    // parked = 0x1A
    let value_parked = vec![0x1A];

    let can_frame = CanFrame::new(vehicle_state_id, MessageType::Extended, &value_parked).unwrap();
    let result = usb_socket.send(can_frame);
    if result.is_ok() {
        println!("parked sent!");
    }

    // vcu_status_pkt_1
    let vehicle_state_id = 0x18EFF4FA;
    // charging = 0x32
    let value_parked = vec![0x32];
    
    let can_frame = CanFrame::new(vehicle_state_id, MessageType::Extended, &value_parked).unwrap();
    let result = usb_socket.send(can_frame);
    if result.is_ok() {
        println!("charge sent!");
    }

    let vcu_status_5_id = 0x18EBF4FA;
    // SOC_limit = 100
    // VA_limit = 2200
    let value_batt = vec![0x64 & 0x7F, 0x98, 0x08, 0x00];
    let can_frame = CanFrame::new(vcu_status_5_id, MessageType::Extended, &value_batt).unwrap();
    let result = usb_socket.send(can_frame);
    if result.is_ok() {
        println!("charging sent!");
    }
    
    // let vcu_status_3_id = 0x18EDF4FA;
    // target_charge_mins_rem =
    // target_charge_range_km =
    // target_charge_hours_rem =
    // range_km = 
    // display_SOC =

    // let mut value_batt = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    // for i in 0..=100 {
        // println!("{}", i);
        // value_batt = vec![0x00, 0x02 - i*(2/100), 0x3B - i*(59/100), 0x96, 0x64, 0x00, 0x00, 0x00];
        // SOC_limit = 100
        // VA_limit = 2200
        // value_batt = vec![0x64 & 0x7F, 0x98, 0x08, 0x00];
        // let can_frame = CanFrame::new(vcu_status_5_id, MessageType::Extended, &value_batt).unwrap();
        // let result = usb_socket.send(can_frame);
        // if result.is_ok() {
        //     println!("charging sent!");
        // }
        // thread::sleep(Duration::from_millis(500));
    // }

}
