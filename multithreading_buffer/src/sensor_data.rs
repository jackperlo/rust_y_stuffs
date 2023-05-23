use std::fmt::{Display, Formatter};


#[repr(C)]
#[derive(Debug, Clone)]
pub struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}
impl SensorData{
    pub fn new() -> Self{
        SensorData{
            seq: 0,
            values: [-0.00; 10],
            timestamp: 0
        }
    }
    pub fn get_seq(&self) -> u32 {self.seq}
    pub fn get_values(&self) -> &[f32; 10] {&self.values}
    pub fn get_timestamp(&self) -> u32 {self.timestamp}

    pub fn set_seq(&mut self, seq: u32) {self.seq = seq}
    pub fn set_values(&mut self, values: &[f32; 10]) {self.values=values.map(|val| val)}
    pub fn set_timestamp(&mut self, timestamp: u32) {self.timestamp = timestamp}
    pub fn from_bytes(content: &[u8]) -> Self {
        let mut sensor_data = SensorData::new();
        /*the struct size is 48 BYTES:
            - u32 -> 4 bytes
            - [f32; 10]  -> 4 bytes * 10
            - u32 -> 4 bytes
        */
        /*FIELD: seq*/
        let seq = u32::from_le_bytes([content[0], content[1], content[2], content[3]]);
        sensor_data.seq = seq;
        /*FIELD: values*/
        let mut index=4;
        for byte in 0..sensor_data.values.len() {
            let aus_vec = vec![content[index], content[index+1], content[index+2], content[index+3]];
            sensor_data.values[byte] = f32::from_le_bytes(<[u8; 4]>::try_from(aus_vec).unwrap());
            index+=4;
        }
        /*FIELD: timestamp*/
        let timestamp = u32::from_le_bytes([content[44], content[45], content[46], content[47]]);
        sensor_data.timestamp = timestamp;
        sensor_data
    }
    pub fn to_bytes(&self) -> Vec<u8>{
        let mut struct_as_byte: Vec<u8> = Vec::new();
        /*FIELD: seq*/
        for seq_byte in self.get_seq().to_le_bytes(){
            struct_as_byte.push(seq_byte);
        }
        /*FIELD: values*/
        let vector_values_bytes = self.get_values().map(|el| el.to_le_bytes());
        for values_bytes in 0..vector_values_bytes.len() {
            for values_byte in vector_values_bytes[values_bytes] {
                struct_as_byte.push(values_byte);
            }
        }
        /*FIELD: timestamp*/
        for timestamp_byte in self.get_timestamp().to_le_bytes(){
            struct_as_byte.push(timestamp_byte);
        }
        struct_as_byte
    }
}
impl Display for SensorData{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SensorData: seq={}, values={:?}, timestamp={}", self.seq, self.values, self.timestamp)
    }
}