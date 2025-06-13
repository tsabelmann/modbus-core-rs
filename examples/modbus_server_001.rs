// use std::{io::{Read, Write}, net::TcpListener, path::Iter};
// use modbus_stack::{decode::request::{ReadHoldingRegistersRequestDecoder, WriteMultipleRegistersRequestDecoder, WriteSingleRegisterRequestDecoder}, encode::response::{ReadHoldingRegistersReponseEncoder, WriteMultipleRegistersReponseEncoder, WriteSingleRegisterReponseEncoder}, register::{IntoRegIter, RegU16, RegU16Array, RegU32, RW}, tcp::{ModbusTcpFrame, ModbusTcpFrameDecoder}, ExceptionCode, PduData};
// use modbus_stack::constants::MODBUS_TCP_FRAME_DATA_LENTGH;
// use modbus_stack::register::RO;


// // trait Model {
// //     fn model_id() -> u16;
// //     fn model_length() -> u16;
// // }


// // trait CommonModel : Model {
// //     fn manufacturer(&self) -> &str;
// //     fn model(&self) -> &str;
// //     fn options(&self) -> &str;
// //     fn version(&self) -> &str;
// //     fn serial_number(&self) -> &str;
// //     fn device_address(&self) -> &str;
// // }

// // trait BatteryBaseModel: Model {

// // }


// // #[derive(Default)]
// // struct ConcreteBatteryBaseModel {
// //     nameplate_charge_capacity: RO<RegU16>,
// //     nameplate_energy_capacity: RO<RegU16>,
// //     nameplate_max_charge_rate: RO<RegU16>,
// //     nameplate_max_discharge_rate: RO<RegU16>,
// //     self_discharge_rate: RO<RegU16>,
// //     nameplate_max_soc: RO<RegU16>,
// //     nameplate_min_soc: RO<RegU16>,
// //     max_reserve_percent: RO<RegU16>,
// //     min_reserve_percent: RO<RegU16>,
// //     state_of_charge: RO<RegU16>,
// //     depth_of_discharge: RO<RegU16>,
// //     state_of_health: RO<RegU16>,
// //     cycle_count: RO<RegU32>,
// //     charge_status: RO<RegU16>,
// //     control_mode: RO<RegU16>,
// //     battery_heartbeat: RO<RegU16>,
// //     contoller_heartbeat: RW<RegU16>,
// //     alarm_reset: RW<RegU16>,
// //     battery_type: RO<RegU16>,
// //     state_of_the_battery_bank: RO<RegU16>,
// //     vendor_battery_bank_state: RO<RegU16>,
// //     warranty_date: RO<RegU32>,
// //     battery_event_1_bitfield: RO<RegU32>,
// //     battery_event_2_bitfield: RO<RegU32>,
// //     vendor_event_bitfield_1: RO<RegU32>,
// //     vendor_event_bitfield_2: RO<RegU32>,
// //     external_byttery_voltage: RO<RegU16>,
// //     max_battery_voltage: RO<RegU16>,
// //     min_battery_voltage: RO<RegU16>,
// //     max_cell_voltage: RO<RegU16>,
// //     max_cell_voltage_strinf: RO<RegU16>,
// //     max_cell_voltage_module: RO<RegU16>,
// //     min_cell_voltage: RO<RegU16>,
// //     min_cell_voltage_strinf: RO<RegU16>,
// //     min_cell_voltage_module: RO<RegU16>,
// //     average_cell_voltage: RO<RegU16>,
// //     total_dc_current: RO<RegU16>,
// //     max_charge_current: RO<RegU16>,
// //     max_discharge_current: RO<RegU16>,
// //     total_power: RO<RegU16>,
// //     inverter_state_request: RO<RegU16>,
// //     battery_power_request: RO<RegU16>,
// //     set_operation: RW<RegU16>,
// //     set_inverter_state: RW<RegU16>
// // }


// // struct RegIter<'a, I>
// // where 
// //     I : Iterator<Item = &'a u16>    
// // {
// //     iter: I
// // }

// // impl<'a, I: Iterator<Item = &'a u16>> RegIter<'a, I> {
// //     pub const fn new(iter: I) -> RegIter<'a, I> {
// //         RegIter { iter }
// //     }
// // }

// // impl<'a, I: Iterator<Item=&'a u16>> Iterator for RegIter<'a, I> {
// //     type Item = I::Item;
// //     fn next(&mut self) -> Option<Self::Item> {
// //         self.iter.next()
// //     }
// // }


// // impl<I> IntoRegIter for ConcreteBatteryBaseModel 
// // where 
// //     I: for<'a> Iterator<Item=&'a u16>,
// //     for<'a> Self: 'a
// // {
// //     type IntoIter<'a> = RegIter<'a, I>
// //     where
// //         Self: 'a;

// //     fn into_reg_iter<'a>(&'a self) -> Self::IntoIter<'a> {
// //         RegIter::new(self.alarm_reset.reg_iter())
// //     }
// // }




// pub const SUN_S: RO::<RegU32> = RO::<RegU32>::new(RegU32::new(0x53_75_6E_53));  
// pub const MODEL_1_ID: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(1));
// pub const MODEL_1_LENGTH: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(68));
// pub const MODEL_1_MANUFACTURER: RO::<RegU16Array<16>> = RO::<RegU16Array<16>>::new(RegU16Array::from_str("NAEXT GmbH"));
// pub const MODEL_1_MODEL: RO::<RegU16Array<16>> = RO::<RegU16Array<16>>::new(RegU16Array::from_str("MultiBMS"));
// pub const MODEL_1_OPTIONS: RO::<RegU16Array<8>> = RO::<RegU16Array<8>>::new(RegU16Array::from_str("NOT"));
// pub const MODEL_1_VERSION: RO::<RegU16Array<8>> = RO::<RegU16Array<8>>::new(RegU16Array::from_str("0.1.0-test"));
// pub const MODEL_1_SERIAL: RO::<RegU16Array<16>> = RO::<RegU16Array<16>>::new(RegU16Array::from_str("SunSpec funktioniert!"));
// pub const MODEL_1_DEVICE_ADDRESS: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(0));
// pub const MODEL_1_PAD: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(0));

// pub const MODEL_END: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(0xFFFF));
// pub const END: RO::<RegU16> = RO::<RegU16>::new(RegU16::new(0));

// pub fn dispatch(starting_address: u16, quantity_of_registers: u16) -> Option<impl Iterator<Item = &'static u16>> {
//     if starting_address < 40000 || starting_address > 40074 {
//         return None;
//     }

//     let diff = starting_address - 40000;
//     if diff + quantity_of_registers > 74 {
//         return None;
//     }

//     // Beginne mit dem ersten Iterator
//     let iter = SUN_S.into_reg_iter();

//     let model_1_id_iter = MODEL_1_ID.into_reg_iter();
//     let model_1_length_iter = MODEL_1_LENGTH.into_reg_iter();
//     let model_1_manufacturer_iter = MODEL_1_MANUFACTURER.into_reg_iter();
//     let model_1_model_iter = MODEL_1_MODEL.into_reg_iter();
//     let model_1_options_iter = MODEL_1_OPTIONS.into_reg_iter();
//     let model_1_version_iter = MODEL_1_VERSION.into_reg_iter();
//     let model_1_serial_iter = MODEL_1_SERIAL.into_reg_iter();
//     let model_1_device_address_iter = MODEL_1_DEVICE_ADDRESS.into_reg_iter();
//     let model_1_pad_iter = MODEL_1_PAD.into_reg_iter();
//     let iter = iter
//         .chain(model_1_id_iter)
//         .chain(model_1_length_iter)
//         .chain(model_1_manufacturer_iter)
//         .chain(model_1_model_iter)
//         .chain(model_1_options_iter)
//         .chain(model_1_version_iter)
//         .chain(model_1_serial_iter)
//         .chain(model_1_device_address_iter)
//         .chain(model_1_pad_iter);

//     // Hänge den letzten an
//     let iter = iter.chain(MODEL_END.into_reg_iter()).chain(END.into_reg_iter());
//     let iter = iter.skip(diff as usize).take(quantity_of_registers as usize);

//     println!("SIZE={}", size_of_val(&iter));
//     Some(iter)
// }

// fn main() -> std::io::Result<()> {
//     let mut buffer = [0u8; MODBUS_TCP_FRAME_DATA_LENTGH];
//     let mut out_buffer = [0u8; MODBUS_TCP_FRAME_DATA_LENTGH];
//     // let mut decoder = ModbusTcpFrameDecoder::new();
//     let listener = TcpListener::bind("0.0.0.0:1502")?;

//     // // accept connections and process them serially
//     // let socket = listener.accept();
//     // let mut stream = match socket {
//     //     Ok((stream, _)) => {
//     //             stream    
//     //         },
//     //     _ => return Ok(())
//     // };

//     for incoming in listener.incoming() {
//         let mut stream = match incoming {
//             Ok(stream) => {
//                     stream    
//                 },
//             _ => continue
//         };

//         println!("Socket created!");
        
//         // reset decoder
//         // decoder.reset();
//         let mut decoder = ModbusTcpFrameDecoder::new();

//         'main: loop {
//             match stream.read(&mut buffer) {
//             Ok(size) => {
//                 // println!("Size: {:?}", size);
//                 // println!("Bytes: {:?}", &out_buffer[..size]);
//                 for val in &buffer[..size] {
//                     if let Ok(mut frame) = decoder.push_data(*val) {
//                         println!("Frame");
//                         println!("Transation-Identifier: {}", frame.transaction_identifier());
//                         println!("Protocol-Identifier: {}", frame.protocol_identifier());
//                         println!("Length: {}", frame.length());
//                         println!("Unit-Identifier: {}", frame.unit_identifier());
//                         println!("Function-Code: {:?}", frame.function_code());

//                         match WriteMultipleRegistersRequestDecoder::new(&frame) {
//                             Ok(req) => {
                                
//                                 println!("<Write-Multiple-Registers-Request>");
//                                 println!("\tStarting-Address: {:04X}", req.starting_address());
//                                 println!("\tQuantity-Of-Register: {}", req.quantity_of_registers());
//                                 println!("\tByte-Count: {}", req.byte_count());
//                                 println!("Bytes: {:02X?}", &buffer[..size]);
                                    
//                                 // for value in req {
//                                 //     println!("\t\tRegister={:04X}", value);
//                                 // }
//                                 // println!();

//                                 let mut out_frame = unsafe {
//                                     ModbusTcpFrame::new_unchecked(&mut out_buffer)
//                                 };
//                                 frame.copy_to(&mut out_frame);
                                

//                                 match WriteMultipleRegistersReponseEncoder::encode(&mut out_frame, req.starting_address(), req.quantity_of_registers()) {
//                                     Ok(_) => {
//                                         println!("<Write-Multiple-Registers-Response>");
//                                         let length = (out_frame.length() + 6) as usize;
//                                         let result = stream.write(&out_buffer[..length]);
//                                     },
//                                     _ => {}
//                                 };
//                                 println!();
//                             },
//                             Err(reason) => {}
//                         };
                        
//                         if let Ok(req) = ReadHoldingRegistersRequestDecoder::new(&frame) {
//                             println!("<Read-Holding-Registers-Request>");
//                             println!("\tStarting-Address: {:04X}", req.starting_address());
//                             println!("\tQuantity-Of-Register: {}", req.quantity_of_registers());

//                             let mut out_frame = unsafe {
//                                 ModbusTcpFrame::new_unchecked(&mut out_buffer)
//                             };
//                             frame.copy_to(&mut out_frame);
                            
//                             let starting_address = req.starting_address();
//                             let quantity_of_registers = req.quantity_of_registers();
                            
//                             if let Some(iter) = dispatch(starting_address, quantity_of_registers) {
//                                 match ReadHoldingRegistersReponseEncoder::encode(&mut out_frame, quantity_of_registers, iter) {
//                                     Ok(_) => {
//                                         println!("<Read-Holding-Registers-Response>");
//                                         let length = (out_frame.length() + 6) as usize;
//                                         let result = stream.write(&out_buffer[..length]);
//                                         // println!("Written-Data: {:?}", result);
//                                         // println!("Bytes: {:?}", &out_buffer[..length]);
//                                     },
//                                     Err(reason) => println!("<Read-Holding-Registers> Not a success - Why? {:?}", reason)
//                                 };
//                             } else {
//                                 match ReadHoldingRegistersReponseEncoder::encode_exception(&mut out_frame, ExceptionCode::IllegalDataAddress) {
//                                     Ok(_) => {
//                                         println!("<Read-Holding-Registers-Response-Exception>");
//                                         let length = (out_frame.length() + 6) as usize;
//                                         let result = stream.write(&out_buffer[..length]);
//                                         println!("Written-Data: {:?}", result);
//                                         println!("Bytes: {:?}", &out_buffer[..length]);
//                                     },
//                                     Err(reason) => println!("<Read-Holding-Registers> Not a success - Why? {:?}", reason)
//                                 };
//                             }
//                             println!();
//                         } else {
//                             // match ReadHoldingRegistersReponseEncoder::encode_exception(&mut out_frame, ExceptionCode::IllegalDataAddress) {
//                             //     Ok(_) => {
//                             //         println!("Exception!");
//                             //         let length = (out_frame.length() + 6) as usize;
//                             //         let result = stream.write(&out_buffer[..length]);
//                             //         println!("Written-Data: {:?}", result);
//                             //         println!("Bytes: {:?}", &out_buffer[..length]);
//                             //     },
//                             //     Err(reason) => println!("Not a success - Why? {:?}", reason)
//                             // };
//                         }

//                         if let Ok(request) = WriteSingleRegisterRequestDecoder::new(&frame) {
//                             println!("<Write-Single-Register>");
//                             println!("\tRegister-Address: {:04X}", request.register_address());
//                             println!("\tRegister-Value: {:04X}", request.register_value());

//                             let mut out_frame = unsafe {
//                                 ModbusTcpFrame::new_unchecked(&mut out_buffer)
//                             };
//                             frame.copy_to(&mut out_frame);

//                             match WriteSingleRegisterReponseEncoder::encode(&mut out_frame, request.register_address(), request.register_value()) {
//                                 Ok(_) => {
//                                     println!("<Write-Single-Register-Resonse>");
//                                     let length = (out_frame.length() + 6) as usize;
//                                     let result = stream.write(&out_buffer[..length]);
//                                     // println!("Written-Data: {:?}", result);
//                                     // println!("Bytes: {:?}", &out_buffer[..length]);
//                                 },
//                                 Err(reason) => {}
//                             };
//                             println!();
//                         }
//                     }
//                 }
//             },
//             Err(err) => println!("Stream-Error: {:?}", err)
//         }
//         let mut test_buffer = [];
//         match stream.read(&mut test_buffer) {
//             Ok(0) => {},
//             _ => {
//                 println!("Destruct Socket");
//                 break 'main;
//             }
//         };
    
//     }
// }
//     Ok(())
// }


use std::ops::{Deref, DerefMut};

use modbus_stack::register::*;
use modbus_stack::register::IntoRegIter;


#[derive(Default)]
struct MultiBmsBatteryBaseModel {
    nameplate_charge_capacity: RO<RegU16>,
    nameplate_energy_capacity: RO<RegU16>,
    nameplate_max_charge_rate: RO<RegU16>,
    nameplate_max_discharge_rate: RO<RegU16>,
    self_discharge_rate: RO<RegU16>,
    nameplate_max_soc: RO<RegU16>,
    nameplate_min_soc: RO<RegU16>,
    max_reserve_percent: RO<RegU16>,
    min_reserve_percent: RO<RegU16>,
    state_of_charge: RO<RegU16>,
    depth_of_discharge: RO<RegU16>,
    state_of_health: RO<RegU16>,
    cycle_count: RO<RegU32>,
    charge_status: RO<RegU16>,
    control_mode: RO<RegU16>,
    battery_heartbeat: RO<RegU16>,
    contoller_heartbeat: RW<RegU16>,
    alarm_reset: RW<RegU16>,
    battery_type: RO<RegU16>,
    state_of_the_battery_bank: RO<RegU16>,
    vendor_battery_bank_state: RO<RegU16>,
    warranty_date: RO<RegU32>,
    battery_event_1_bitfield: RO<RegU32>,
    battery_event_2_bitfield: RO<RegU32>,
    vendor_event_bitfield_1: RO<RegU32>,
    vendor_event_bitfield_2: RO<RegU32>,
    external_byttery_voltage: RO<RegU16>,
    max_battery_voltage: RO<RegU16>,
    min_battery_voltage: RO<RegU16>,
    max_cell_voltage: RO<RegU16>,
    max_cell_voltage_strinf: RO<RegU16>,
    max_cell_voltage_module: RO<RegU16>,
    min_cell_voltage: RO<RegU16>,
    min_cell_voltage_strinf: RO<RegU16>,
    min_cell_voltage_module: RO<RegU16>,
    average_cell_voltage: RO<RegU16>,
    total_dc_current: RO<RegU16>,
    max_charge_current: RO<RegU16>,
    max_discharge_current: RO<RegU16>,
    total_power: RO<RegU16>,
    inverter_state_request: RO<RegU16>,
    battery_power_request: RO<RegU16>,
    set_operation: RW<RegU16>,
    set_inverter_state: RW<RegU16>
}

enum MultiBmsBatteryBaseModelIterState<'a> {
    NameplateChargeCapacity(core::slice::Iter<'a, u16>),
    NameplateEnergyCapacity(core::slice::Iter<'a, u16>),
    NameplateMaxChargeRate(core::slice::Iter<'a, u16>),
    NameplateMaxDischargeRate(core::slice::Iter<'a, u16>),
    Empty
}

struct MultiBmsBatteryBaseModelIter<'a> {
    ptr: &'a MultiBmsBatteryBaseModel,
    state: MultiBmsBatteryBaseModelIterState<'a>
}

impl<'a> MultiBmsBatteryBaseModelIter<'a> {
    pub fn new(value: &'a MultiBmsBatteryBaseModel) -> MultiBmsBatteryBaseModelIter<'a> {
        MultiBmsBatteryBaseModelIter {
            ptr: value, 
            state: MultiBmsBatteryBaseModelIterState::NameplateChargeCapacity(value.nameplate_charge_capacity.into_reg_iter())
        }
    }
}

impl<'a> IntoIterator for &'a MultiBmsBatteryBaseModel {
    type Item = &'a u16;
    type IntoIter = MultiBmsBatteryBaseModelIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MultiBmsBatteryBaseModelIter::new(self)
    }
}

impl<'a> Iterator for MultiBmsBatteryBaseModelIter<'a> {
    type Item = &'a u16;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.state {
                MultiBmsBatteryBaseModelIterState::NameplateChargeCapacity(iter) => {
                    match iter.next() {
                        Some(reff) => return Some(reff),
                        None => {
                            self.state = MultiBmsBatteryBaseModelIterState::NameplateEnergyCapacity(self.ptr.nameplate_energy_capacity.into_reg_iter())
                        } 
                    };
                },
                MultiBmsBatteryBaseModelIterState::NameplateEnergyCapacity(iter) => {
                    match iter.next() {
                        Some(reff) => return Some(reff),
                        None => {
                            self.state = MultiBmsBatteryBaseModelIterState::NameplateMaxChargeRate(self.ptr.nameplate_max_charge_rate.into_reg_iter())
                        } 
                    };
                },
                MultiBmsBatteryBaseModelIterState::NameplateMaxChargeRate(iter) => {
                    match iter.next() {
                        Some(reff) => return Some(reff),
                        None => {
                            self.state = MultiBmsBatteryBaseModelIterState::NameplateMaxDischargeRate(self.ptr.nameplate_max_discharge_rate.into_reg_iter())
                        } 
                    };
                }
                MultiBmsBatteryBaseModelIterState::NameplateMaxDischargeRate(iter) => {
                    match iter.next() {
                        Some(reff) => return Some(reff),
                        None => {
                            self.state = MultiBmsBatteryBaseModelIterState::Empty
                        } 
                    };
                },
                MultiBmsBatteryBaseModelIterState::Empty => return None,
            };
        }
    }
}



// enum MultiBmsBatteryBaseModelIterMutState<'a> {
//     NameplateChargeCapacity(core::slice::IterMut<'a, u16>),
//     NameplateEnergyCapacity(core::slice::IterMut<'a, u16>),
//     NameplateMaxChargeRate(core::slice::IterMut<'a, u16>),
//     NameplateMaxDischargeRate(core::slice::IterMut<'a, u16>),
//     Empty
// }

// struct MultiBmsBatteryBaseModelIterMut<'a> {
//     ptr: &'a MultiBmsBatteryBaseModel,
//     state: MultiBmsBatteryBaseModelIterState<'a>
// }


struct U32 {
    array: [u16; 2]
}

struct ViewU32<'a> {
    ptr: &'a U32,
    value: u32
}

impl<'a> ViewU32<'a> {
    pub const fn new(value: &'a U32) -> ViewU32<'a> {
        let v = ((value.array[0] as u32) << 16) | (value.array[1] as u32);
        ViewU32 {
            ptr: value,
            value: v
        }
    }
}


struct ViewMutU32<'a> {
    ptr: &'a mut U32,
    value: u32
}

impl<'a> ViewMutU32<'a> {
    pub const fn new(value: &'a mut U32) -> ViewMutU32<'a> {
        let v = ((value.array[0] as u32) << 16) | (value.array[1] as u32);
        ViewMutU32 {
            ptr: value,
            value: v
        }
    }
}




impl<'a> Deref for ViewU32<'a> {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Deref for ViewMutU32<'a> {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> DerefMut for ViewMutU32<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'a> Drop for ViewMutU32<'a> {
    fn drop(&mut self) {
        let high_word = (self.value >> 16) as u16;
        let low_word = (self.value) as u16;

        self.ptr.array[0] = high_word;
        self.ptr.array[1] = low_word;
    }
}


fn main() {
    let mut value = U32 { array: [0, 42] };

    let mut reg = modbus_stack::register::RO::new(modbus_stack::register::RegF32::new(20.01));
    println!("reff={:?}", reg.get_value());

    let ptr = ViewU32::new(&value);
    
    let new_value = *ptr;
    println!("new_value={}", new_value);
    
    // {
    //     let new_ptr = ViewMutU32::new(&mut value);
    // }

    // let new_value = *ptr;

    println!("array={:?}", value.array);


    // let mut battery_base_model = MultiBmsBatteryBaseModel::default();
    // *battery_base_model.nameplate_charge_capacity.get_mut() = 42.into();

    // let length = battery_base_model.into_reg_iter().count();
    // assert_eq!(length, 4);

    // for value in battery_base_model.into_reg_iter() {
    //     println!("value={}", value);
    // }
}