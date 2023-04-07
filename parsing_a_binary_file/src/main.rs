#[allow(dead_code, unused_variables)]
use std::{env, mem};
use std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ValueStruct{
    my_type: i32,
    val: f32,
    timestamp: i32
}
impl ValueStruct{
    fn new() -> Self {
        ValueStruct{
            my_type: -1,
            val: -1.00,
            timestamp: -1
        }
    }
    fn create_struct_from_bytes(content: &[u8]) -> Self {
        let mut value_struct = ValueStruct::new();
        /*the struct size is 12 BYTES:
            - i32 -> 4 bytes
            - f32 -> 4 bytes
            - i32 -> 4 bytes
        */
        let i32_field = i32::from_le_bytes([content[0], content[1], content[2], content[3]]);
        value_struct.my_type = i32_field;

        let f32_field = f32::from_le_bytes([content[4], content[5], content[6], content[7]]);
        value_struct.val = f32_field;

        let i32_field = i32::from_le_bytes([content[8], content[9], content[10], content[11]]);
        value_struct.timestamp = i32_field;

        //println!("{}", value_struct);
        value_struct
    }
}
impl Display for ValueStruct{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValueStruct: type={}, val={}, timestamp={}", self.my_type, self.val, self.timestamp)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct MValueStruct{
    my_type: i32,
    val: [f32; 10],
    timestamp: i32
}
impl MValueStruct{
    fn new() -> Self {
        MValueStruct{
            my_type: -1,
            val: [-1.00; 10],
            timestamp: -1
        }
    }
    fn create_struct_from_bytes(content: &[u8]) -> Self {
        let mut m_value_struct = MValueStruct::new();
        /*the struct size is 48 BYTES:
            - i32       -> 4 bytes
            - [f32; 10] -> 4 bytes * 10
            - i32       -> 4 bytes
        */
        let i32_field = i32::from_le_bytes([content[0], content[1], content[2], content[3]]);
        m_value_struct.my_type = i32_field;

        let res = m_value_struct.val.iter().enumerate().map(|(i, &_x)| f32::from_le_bytes([content[4*(i+1)], content[(4*(i+1))+1], content[(4*(i+1))+2], content[(4*(i+1))+3]])).collect::<Vec<_>>();
        m_value_struct.val = <[f32; 10]>::try_from(res).unwrap();

        let i32_field = i32::from_le_bytes([content[44], content[45], content[46], content[47]]);
        m_value_struct.timestamp = i32_field;

        //println!("{}", m_value_struct);
        m_value_struct
    }
}
impl Display for MValueStruct{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MValueStruct: type={}, val={:?}, timestamp={}", self.my_type, self.val, self.timestamp)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct MessageStruct{
    my_type: i32,
    message: [u8; 21]
}
impl MessageStruct{
    fn new() -> Self {
        MessageStruct{
            my_type: -1,
            message: [0; 21]
        }
    }
    fn char_array_to_string(&self) -> String {
        let mut i=0;
        let mut res_string = String::new();
        while self.message[i] != 0 && i<self.message.len() {
            res_string.push(self.message[i] as char);
            i+=1;
        }
        res_string
    }
    fn create_struct_from_bytes(content: &[u8]) -> Self {
        let mut message_struct = MessageStruct::new();
        /*the struct size is 28 BYTES:
            - i32 -> 4 bytes
            - u8  -> 1 bytes * 21
        */
        let i32_field = i32::from_le_bytes([content[0], content[1], content[2], content[3]]);
        message_struct.my_type = i32_field;

        //println!("message len: {}", message_struct.message.len());

        let mut index=0;
        for _byte in message_struct.message {
            message_struct.message[index] = content[index+4];
            index+=1;
        }

        //println!("{}", message_struct);
        message_struct
    }
}
impl Display for MessageStruct{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MessageStruct: type={}, message='{}'", self.my_type, self.char_array_to_string())
    }
}


#[repr(C)]
#[derive(Clone, Copy)]
union content{
    val: ValueStruct,
    mval: MValueStruct,
    message: MessageStruct
}
impl content {
    fn new_val(val: ValueStruct) -> Self {
        content{
            val
        }
    }
    fn new_m_val(mval: MValueStruct) -> Self {
        content{
            mval
        }
    }
    fn new_message(message: MessageStruct) -> Self {
        content{
            message
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CData{
    my_type: i32,
    content: content
}
impl CData{
    fn new_val(my_type: i32, content: ValueStruct) -> Self {
        CData{
            my_type,
            content: content::new_val(content)
        }
    }
    fn new_m_val(my_type: i32, content: MValueStruct) -> Self {
        CData{
            my_type,
            content: content::new_m_val(content)
        }
    }
    fn new_message(my_type: i32, content: MessageStruct) -> Self {
        CData{
            my_type,
            content: content::new_message(content)
        }
    }
}
impl Display for CData{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe{
            match self.my_type{
                1=> write!(f, "CDATA: \ttype={}\n\t\tcontent={{{}}}", self.my_type, self.content.val),
                2=> write!(f, "CDATA: \ttype={}\n\t\tcontent={{{}}}", self.my_type, self.content.mval),
                3=> write!(f, "CDATA: \ttype={}\n\t\tcontent={{{}}}", self.my_type, self.content.message),
                _=> panic!("Cannot display CData")
            }
        }
    }
}

//cargo run -- filename.bin
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];
    println!("File Name: {}", file_name);

    println!("int size={}, float size={}, long size={}", mem::size_of::<i32>(), mem::size_of::<f32>(), mem::size_of::<i32>());
    println!("ValueStruct size={}", mem::size_of::<ValueStruct>());
    println!("MValueStruct size={}", mem::size_of::<MValueStruct>());
    println!("MessageStruct size={}, message size={}, char size={}", mem::size_of::<MessageStruct>(),  mem::size_of::<[u8; 21]>(), mem::size_of::<u8>());
    println!("Content size={}", mem::size_of::<content>());
    println!("data size: {}", mem::size_of::<CData>());

    let data = read_from_file(file_name);
    for el in data{
        println!("{}", el);
    }
}

fn read_from_file(file_name:  &str) -> Vec<CData>{
    let bytes = std::fs::read(file_name).unwrap();
    let mut collected_data: Vec<CData> = vec![];

    let mut n_bytes=0;
    while n_bytes < bytes.len() {
        //println!("bytes di type: {:?}", &bytes[n_bytes..n_bytes+4]);
        let byte = &bytes[n_bytes..=n_bytes+4];
        let structure_type = i32::from_le_bytes([byte[0], byte[1], byte[2], byte[3]]);
        n_bytes+=4;
        //if n_bytes%4 != 0 { print!("type_i32: {} | ", structure_type); }
        //else { println!("type_i32: {}", structure_type); }

        match structure_type {
            1 => {
                //println!("bytes di content: {:?}", &bytes[n_bytes..n_bytes + 12]);
                let value_struct = ValueStruct::create_struct_from_bytes(&bytes[n_bytes..n_bytes + 12]);
                collected_data.push(CData::new_val(structure_type, value_struct));
            },
            2 => {
                //println!("bytes di content: {:?}", &bytes[n_bytes..n_bytes + 44]);
                let m_value_struct = MValueStruct::create_struct_from_bytes(&bytes[n_bytes..n_bytes + 48]);
                collected_data.push(CData::new_m_val(structure_type, m_value_struct));
            },
            3 => {
                //println!("bytes di content: {:?}", &bytes[n_bytes..n_bytes + 28]);
                let message_struct = MessageStruct::create_struct_from_bytes(&bytes[n_bytes..n_bytes + 28]);
                collected_data.push(CData::new_message(structure_type, message_struct));
            }
            _ => println!("Error, CData TYPE not valid")
        }
        n_bytes += 48; //proceed by the content size (effective size+padding)
    }

    println!("n_bytes: {}, n_entries: {}", n_bytes, (n_bytes)/52);
    collected_data
}

