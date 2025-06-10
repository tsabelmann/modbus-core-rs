use std::{io::{Read, Write}, net::TcpListener};
use modbus_stack::{server::{request::{ReadHoldingRegistersRequest, WriteSingleRegisterRequest}, response::ReadHoldingRegistersResponseBuilder}, tcp::{ModbusTcpFrame, ModbusTcpFrameDecoder}, PduData, PduDataMut};


// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0u8; 1024];
//     if let Ok(length)  = stream.read(&mut buffer) {
//         let slice = &buffer[0..length];
//         if let Ok(string) = std::str::from_utf8(slice) {
//             println!("string={}", string);
//         }
//     }
// }

fn main() -> std::io::Result<()> {
    let mut buffer = [0u8; 260];
    let mut out_buffer = [0u8; 260];
    let mut decoder = ModbusTcpFrameDecoder::new();


    let listener = TcpListener::bind("0.0.0.0:1502")?;

    // accept connections and process them serially
    let socket = listener.accept();
    let mut stream = match socket {
        Ok((stream, _)) => {
                stream    
            },
        _ => return Ok(())
    };

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                for val in &buffer[..size] {
                    if let Ok(mut frame) = decoder.push_data(*val) {
                        println!("Frame");
                        println!("Transation-Identifier: {}", frame.transaction_identifier());
                        println!("Protocol-Identifier: {}", frame.protocol_identifier());
                        println!("Length: {}", frame.length());
                        println!("Unit-Identifier: {}", frame.unit_identifier());
                        println!("Function-Code: {:?}", frame.function_code());

                        if let Ok(read_holding_register_request) = ReadHoldingRegistersRequest::new(&frame) {
                            println!("Starting-Address: {:04X}", read_holding_register_request.starting_address());
                            println!("Quantity-Of-Register: {}", read_holding_register_request.quantity_of_registers());

                            let mut out_frame = unsafe {
                                ModbusTcpFrame::new_unchecked(&mut out_buffer)
                            };
                            frame.copy_to(&mut out_frame);
                            
                            let range= (0..0xFFFFu16).into_iter();
                            let mut builder = ReadHoldingRegistersResponseBuilder::new(&mut out_frame, range, read_holding_register_request.quantity_of_registers());
                            if let Ok(written_size) = builder.encode() {
                                println!("Writte-Size: {}", written_size);
                                
                                out_frame.set_length(written_size + 1);

                                let length = (6 + out_frame.length())as usize;


                                println!("out_buffer[..length] = {:?}", &out_buffer[..length]);

                                let result = stream.write(&out_buffer[..length]);
                                println!("Written-Data: {:?}", result);
                            }
                        }

                        if let Ok(write_single_request) = WriteSingleRegisterRequest::new(&frame) {
                            println!("Register-Address: {:04X}", write_single_request.register_address());
                            println!("Register-Value: {:04X}", write_single_request.register_value());
                        }
                    }
                }
            },
            Err(_) => {}
        };
    }

    Ok(())
}