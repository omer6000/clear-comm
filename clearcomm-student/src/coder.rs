use async_std::prelude::*;
use color_eyre::eyre::Result;

pub(super) async fn encode(
    mut stream: impl Stream<Item = u8> + Unpin,
) -> Result<impl Stream<Item = u8>> {
    let mut data = vec![];
    while let Some(byte) = stream.next().await {
        data.push(byte);

    }
    let output = async_std::stream::from_iter(data).map(|num| encoded_tuple(num)).flatten();
    Ok(output)
}
pub(super) async fn decode(
    mut stream: impl Stream<Item = u8> + Unpin,
) -> Result<impl Stream<Item = u8>> {
    let mut data = vec![];
    while let Some(res1) = stream.next().await {
        if let Some(res2) = stream.next().await {
            let repair = repair_msg(res1,res2);
            data.push(repair);
        }
    }
    let output = async_std::stream::from_iter(data);
    Ok(output)
}

fn parity_bitarray(num: u8) -> [u8; 7] {
    let mut bits: [u8; 7] = [0;7];
    
    // Information bits
    bits[2] = (num >> 3) & 1;
    bits[4] = (num >> 2) & 1;
    bits[5] = (num >> 1) & 1;
    bits[6] = num & 1;
    
    // Parity bits
    bits[0] = (bits[2] + bits[4] + bits[6]) % 2;
    bits[1] = (bits[2] + bits[5] + bits[6]) % 2;
    bits[3] = (bits[4] + bits[5] + bits[6]) % 2;

    return bits
}

fn bitarray(num: u8) -> [u8;8] {
    let mut bits: [u8; 8] = [0;8];
    for i in 0..8 {
        bits[7-i] = (num >> i) & 1
    }
    return bits
}

fn bitarray_to_int (bitarray: [u8; 8]) -> u8 {
    let mut result = 0;
    for bit in bitarray {
        result = (result << 1) | bit;
    }
    return result
}

fn encoded_tuple(num: u8) -> impl Stream<Item = u8> {

    // HC(7,4) encoded
    let right_bits = parity_bitarray(num & 0x0F); // 0 to 3 bits
    let left_bits = parity_bitarray(num >> 4); // 4 to 7 bits
    
    //encoded information but not interleaved yet
    let mut combined_arr = [0;16];
    combined_arr[..7].copy_from_slice(&left_bits);
    combined_arr[7..14].copy_from_slice(&right_bits);

    //interleaved into two integers below
    let mut inter1: [u8; 8] = [0;8];
    let mut inter2: [u8; 8] = [0;8];
    for (i, j) in (0..8).zip([0, 4, 8, 12, 1, 5, 9, 13].iter()) {
        inter1[i] = combined_arr[*j as usize];
    }
    for (i, j) in (0..8).zip([2, 6, 10, 14, 3, 7, 11, 15].iter()) {
        inter2[i] = combined_arr[*j as usize];
    }
    
    let res1: u8 = bitarray_to_int(inter1);
    let res2: u8 = bitarray_to_int(inter2);

    return async_std::stream::from_iter(vec![res1, res2])
}

fn repair_msg(res1:u8,res2:u8) -> u8 {
    
    let inter1 = bitarray(res1); //first two columns
    let inter2 = bitarray(res2); //last two columns
    let mut combined_arr: [u8;16]= [0;16];

    for (i, j) in (0..8).zip([0, 4, 8, 12, 1, 5, 9, 13].iter()) {
        // inter1[i] = combined_arr[*j as usize];
        combined_arr[*j as usize] = inter1[i];
    }
    for (i, j) in (0..8).zip([2, 6, 10, 14, 3, 7, 11, 15].iter()) {
        // inter1[i] = combined_arr[*j as usize];
        combined_arr[*j as usize] = inter2[i];
    }
    let mut left_bits = [0;7];
    let mut right_bits = [0;7];
    left_bits.copy_from_slice(&combined_arr[0..7]);
    right_bits.copy_from_slice(&combined_arr[7..14]);


    //Repairing
    left_bits = repair_bits(left_bits);
    right_bits = repair_bits(right_bits);

    let final_bitarray = [left_bits[2],left_bits[4],left_bits[5],left_bits[6],right_bits[2],right_bits[4],right_bits[5],right_bits[6]];
    return bitarray_to_int(final_bitarray)
}

fn repair_bits(mut bitarray: [u8; 7]) -> [u8; 7] {
    
    // s1=(p1+c3+c5+c7)mod2=0
    // s2=(p2+c3+c6+c7)mod2=1
    // s3=(p3+c5+c6+c7)mod2=1

    let s1 = (bitarray[0]+bitarray[2]+bitarray[4]+bitarray[6])%2; 
    let s2 = (bitarray[1]+bitarray[2]+bitarray[5]+bitarray[6])%2;
    let s3 = (bitarray[3]+bitarray[4]+bitarray[5]+bitarray[6])%2; 
    let s = s1 | s2 | s3;
    if s != 0 {
        let index = (s1*1+s2*2+s3*4) as usize;
        bitarray[index-1] ^= 1;
    }
    return bitarray
}