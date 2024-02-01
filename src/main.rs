use pcan_basic::bus::UsbBus;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::CanFrame;
use pcan_basic::socket::{Baudrate, MessageType, SendCan};
use std::time::Duration;
use std::thread;

// #[deny(overflowing_literals)]
#[tokio::main]
async fn main() {
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
    // let value_parked = vec![0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    // parked = 0x1A
    let value_parked = vec![0x1A];

    // let can_frame = CanFrame::new(vehicle_state_id, MessageType::Extended, &value_parked).unwrap();
    // let result = usb_socket.send(can_frame);
    // if result.is_ok() {
    //     println!("parked sent!");
    // }
    // thread::sleep(Duration::from_millis(500));

    // obc_
    // let obc_status_pkt_1_id = 0x18FFF2FA;

    // let value_volt = vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    // let can_frame = CanFrame::new(obc_status_pkt_1_id, MessageType::Extended, &value_volt).unwrap();
    // let result = usb_socket.send(can_frame);
    // if result.is_ok() {
    //     println!("charging limit sent!");
    // }
    // thread::sleep(Duration::from_millis(500));

    // obc_status_pkt_1
    let obc_status_pkt_1_id = 0x18FFF8F2;

    let mut value_volt;
    let mut can_frame;
    let mut result; 
    // loop {
    for i in 0..=2000 {
        value_volt = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xDC, 0x00];
        value_volt[7] = (value_volt[7] as u8).wrapping_add(i as u8) as u8;
        can_frame = CanFrame::new(obc_status_pkt_1_id, MessageType::Extended, &value_volt).unwrap();
        result = usb_socket.send(can_frame);
        if result.is_ok() {
            println!("charging limit sent!");
        }
        thread::sleep(Duration::from_millis(50));
    }


    // thread::sleep(Duration::from_millis(6000));

    // // open charge page
    // // vcu_status_pkt_1
    // // let vehicle_state_id = 0x18EFF4FA;
    // // charging = 0x32
    // let value_parked = vec![0x32];
    
    // can_frame = CanFrame::new(vehicle_state_id, MessageType::Extended, &value_parked).unwrap();
    // result = usb_socket.send(can_frame);
    // if result.is_ok() {
    //     println!("charge sent!");
    // }

    // // simulate charging
    // let vcu_status_5_id = 0x18EBF4FA;
    // // SOC_limit = 100
    // // VA_limit = 2200
    // let mut value_status_5 = vec![0x64 & 0x7F, 0x98, 0x08, 0x00];
    // can_frame = CanFrame::new(vcu_status_5_id, MessageType::Extended, &value_status_5).unwrap();
    // result = usb_socket.send(can_frame);
    // if result.is_ok() {
    //     println!("charging limit sent!");
    // }
    
    // let vcu_status_3_id = 0x18EDF4FA;
    // // target_charge_range_km =
    // // target_charge_mins_rem =
    // // target_charge_hours_rem =
    // // range_km = 
    // // display_SOC =

    // let mut value_status_3 = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    // for i in 0..=100 {
    //     // println!("{}", i);
    //     // value_batt = vec![0x00, 0x02 - i*(2/100), 0x3B - i*(59/100), 0x96, 0x64, 0x00, 0x00, 0x00];
    //     value_status_3 = vec![0x00 + i, 0x00 * 150/100, 0x02 - i*(2/100), 0x3B - i*(59/100), 0x96];
    //     can_frame = CanFrame::new(vcu_status_3_id, MessageType::Extended, &value_status_3).unwrap();
    //     result = usb_socket.send(can_frame);
    //     if result.is_ok() {
    //         println!("charging sent!");
    //     }
    //     thread::sleep(Duration::from_millis(500));
    // }
    // thread::sleep(Duration::from_millis(500));

    // //simulate charge finish
    // // store_charge_cable = 1
    // // charge_complete_bool = 1
    // value_status_5 = vec![0x64 & 0x7F, 0x98, 0x08, 0x03];
    // can_frame = CanFrame::new(vcu_status_5_id, MessageType::Extended, &value_status_5).unwrap();
    // result = usb_socket.send(can_frame);
    // if result.is_ok() {
    //     println!("charge finish sent!");
    // }
    // let store_cable = vec![0x35];
    
    // can_frame = CanFrame::new(vehicle_state_id, MessageType::Extended, &store_cable).unwrap();
    // result = usb_socket.send(can_frame);
    // if result.is_ok() {
    //     println!("charge sent!");
    // }

    // can_frame = CanFrame::new(vehicle_state_id, MessageType::Extended, &value_parked).unwrap();
    // result = usb_socket.send(can_frame);
    // if result.is_ok() {
    //     println!("parked sent!");
    // }
    // thread::sleep(Duration::from_millis(500));

}
