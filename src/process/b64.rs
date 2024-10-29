use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cli::Base64Format;
use anyhow::Result;
use std::{fs::File, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let encode = match format {
        Base64Format::Standard => STANDARD.encode(buffer), // &buffer 引用buffer的值 类似于 c++中的引用 避免拷贝复制减少内存的开销。 rust中该操作还会避免buffer的所有权转移
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buffer),
    };
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?; // 通过encode之后获得到的是base64编码后的字符串，所以需要使用read_to_string将buffer中的字符串读取出来而不是read_to_end
    let buffer = buffer.trim(); // 去除buffer中可能存在的空格和换行

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer),
    };

    let decode = String::from_utf8(decode?)?; // ？会对decode结果进行解包，如果出错会返回错误，否则返回正确的Vec值
    println!("{}", decode);

    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Cargo.toml";
        assert!(process_encode(input, Base64Format::UrlSafe).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/tmp.b64";
        assert!(process_decode(input, Base64Format::UrlSafe).is_ok());
    }
}
