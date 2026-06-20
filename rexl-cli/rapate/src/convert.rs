use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Result, Seek, SeekFrom, Write};

pub static MAX_MASK_LENGTH: i32 = 2147483647 / 7; // almost 300mb

pub static JPG_HEAD: &[u8] = &[0xff, 0xd8, 0xff, 0xe1];

pub static MOV_HEAD: &[u8] = &[0x6d, 0x6f, 0x6f, 0x76];

pub static MP4_HEAD: &[u8] = &[
    0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x6D, 0x00, 0x00, 0x02, 0x00,
    0x69, 0x73, 0x6F, 0x6D, 0x69, 0x73, 0x6F, 0x32, 0x61, 0x76, 0x63, 0x31, 0x6D, 0x70, 0x34, 0x31,
];

pub static EXE_HEAD: &[u8] = &[
    0x4D, 0x5A, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00,
    0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
    0x0E, 0x1F, 0xBA, 0x0E, 0x00, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01, 0x4C, 0xCD, 0x21, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x20, 0x63, 0x61, 0x6E, 0x6E, 0x6F,
    0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20, 0x69, 0x6E, 0x20, 0x44, 0x4F, 0x53, 0x20,
    0x6D, 0x6F, 0x64, 0x65, 0x2E, 0x0D, 0x0D, 0x0A, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

static MASK_INDICATOR_LENGTH: i32 = 4; // byte count of file length, max is 4GB

pub fn disguise_with_mask_file(
    file_path: &str, mask_file_path: &str, dry_run: bool,
) -> Result<usize> {
    let mask_head = file_to_bytes(mask_file_path)?;
    disguise(file_path, &*mask_head, dry_run)
}

/// 伪装文件：替换文件头并保存原始头到文件末尾
///
/// # 参数
/// * `file_path`: 真实文件路径
/// * `mask_head`: 面具头部数据
///
/// # 返回值
/// 成功返回 Ok(new_file_length)，失败返回 Err
pub fn disguise(file_path: &str, mask_head: &[u8], dry_run: bool) -> Result<usize> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(!dry_run)
        .open(file_path)?;

    let file_length = file.metadata()?.len() as usize;
    // mask_head 长度
    let length_bytes = int_to_bytes(mask_head.len());

    // 读取原始头部数据
    let original_head = if file_length >= mask_head.len() {
        let mut buffer = vec![0; mask_head.len()];
        file.read_exact(&mut buffer)?;
        buffer
    } else {
        let mut buffer = vec![0; file_length];
        file.read_exact(&mut buffer)?;
        buffer
    };

    let new_file_length = file_length + mask_head.len() + original_head.len() + length_bytes.len();
    if dry_run {
        return Ok(new_file_length);
    }

    // 写入新的头部（mask_head）
    file.seek(SeekFrom::Start(0))?;
    file.write_all(mask_head)?;

    // 在文件末尾追加反转后的原始头部
    file.seek(SeekFrom::End(0))?;
    let reversed_original = reverse_byte_array(&original_head);
    file.write_all(&reversed_original)?;

    // 写入 mask_head 长度信息（最后4个字节）
    file.write_all(&length_bytes)?;
    Ok(new_file_length)
}

/// 还原文件：恢复被伪装的文件
///
/// # 参数
/// * `file_path`: 经过伪装的文件路径
///
/// # 返回值
/// 成功返回 Ok(new_file_length)，失败返回 Err
pub fn reveal(file_path: &str, dry_run: bool) -> Result<usize> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(!dry_run)
        .open(file_path)?;

    let file_length = file.metadata()?.len() as usize;

    // 读取面具长度信息（最后4个字节）
    file.seek(SeekFrom::Start((file_length - MASK_INDICATOR_LENGTH as usize) as u64))?;
    let mut length_buffer = [0u8; 4];
    file.read_exact(&mut length_buffer)?;
    let mask_head_length = u32::from_le_bytes(length_buffer) as usize;

    // 读取原始头部数据
    let original_head =
        if mask_head_length <= (file_length - MASK_INDICATOR_LENGTH as usize - mask_head_length) {
            // 正常情况下，面具长度小于真实文件长度
            file.seek(SeekFrom::Start(
                (file_length - MASK_INDICATOR_LENGTH as usize - mask_head_length) as u64,
            ))?;
            let mut buffer = vec![0u8; mask_head_length];
            file.read_exact(&mut buffer)?;
            buffer
        } else {
            // 非正常情况下，面具长度大于真实文件长度
            file.seek(SeekFrom::Start(mask_head_length as u64))?;
            let remaining_length = file_length - MASK_INDICATOR_LENGTH as usize - mask_head_length;
            let mut buffer = vec![0u8; remaining_length];
            file.read_exact(&mut buffer)?;
            buffer
        };

    let new_length = file_length - mask_head_length - MASK_INDICATOR_LENGTH as usize;
    let new_file_length = new_length + original_head.len();
    if dry_run {
        return Ok(new_file_length);
    }

    // 截断文件，移除附加的数据
    file.set_len(new_length as u64)?;

    // 写入原始头部数据到文件开头
    file.seek(SeekFrom::Start(0))?;
    let reversed_original = reverse_byte_array(&original_head);
    file.write_all(&reversed_original)?;
    Ok(new_file_length)
}

/// 将整数转换为字节数组（小端序）
fn int_to_bytes(value: usize) -> [u8; 4] {
    (value as u32).to_le_bytes()
}

/// 反转字节数组
fn reverse_byte_array(data: &[u8]) -> Vec<u8> {
    data.iter().rev().copied().collect()
}

/// 将文件转换为字节数组，大小受限于MAX_MASK_LENGTH的限制，
/// 如果超出大小限制，则返回空数组
///
/// # 参数
/// * `file_path`: 目标文件路径
///
/// # 返回值
/// 目标文件转换的字节数组，如果超出大小限制，则返回空数组
fn file_to_bytes(file_path: &str) -> Result<Vec<u8>> {
    let file = OpenOptions::new()
        .read(true)
        .open(file_path)?;

    let file_length = file.metadata()?.len() as usize;

    if file_length > 0 && (file_length as i32) < MAX_MASK_LENGTH {
        fs::read(file_path)
    } else {
        Ok(Vec::new())
    }
}
