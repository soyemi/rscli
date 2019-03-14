use nom::{IResult,HexDisplay,be_u8,le_u8,le_u16,le_u32,ErrorKind};

use std::str;

pub type RVA = u32;

#[derive(Debug)]
pub struct DataInfo {
    pub rva: RVA,
    pub size: u32,
}

impl DataInfo{
    named!(pub parse<&[u8],DataInfo>,do_parse!(
        rva: le_u32 >>
        size: le_u32 >>
        (DataInfo{rva:rva,size:size})
    ));
}

named!(pub parse_str_pad<&[u8],&str>,do_parse!(
    str: take_till!(|ch| ch == 0_u8) >>
    take!((4 - (str.len()+1) % 4) % 4 + 1) >>
    (
        str::from_utf8(str).unwrap()
    )
));

pub fn calculate_bits_1(v:u8)->u8{
    let mut x =v;
    let mut c:u8 = 0;
    while x > 0 {
        x &= x-1;
        c = c+1;
    };
    c
}

pub fn calculate_bits_u64(v:u64)->u8{
    let mut x =v;
    let mut c:u8 = 0;
    while x > 0 {
        x &= x-1;
        c = c+1;
    };
    c
}


pub fn calculate_bits_vec(v:&Vec<u8>)->u8{
    let mut c = 0;
    for &i in v.iter() {
        c += calculate_bits_1(i);
    };
    c
}

pub fn resolve_result<'a,T>(suc:&mut bool,result:IResult<&'a [u8],T>)->Option<(&'a [u8],T)>{

    let ret= match result {
        Ok(v)=>{Some(v)},
        Err(e)=>{
            Option::None
        }
    };
    *suc = ret.is_some();
    ret
}

pub fn return_err<'a,T>(i:&'a [u8],code:u32)->IResult<&'a [u8],T>{
    Result::Err(nom::Err::Failure(nom::Context::Code(i,ErrorKind::Custom(0))))
}