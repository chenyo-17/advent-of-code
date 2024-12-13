use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

struct DualFileReader {
    file: File,
    front_pos: i64,
    back_pos: i64,
}

impl DualFileReader {
    fn new(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let len = file.seek(SeekFrom::End(0)).unwrap();
        DualFileReader {
            file,
            front_pos: -1,
            back_pos: len as i64,
        }
    }

    fn next_front(&mut self) -> Option<i32> {
        while self.front_pos < self.back_pos {
            self.front_pos += 1;
            self.file
                .seek(SeekFrom::Start(self.front_pos as u64))
                .unwrap();
            let mut buffer = [0u8; 1];
            self.file.read_exact(&mut buffer).unwrap();
            return Some(buffer[0].checked_sub(b'0').unwrap() as i32);
        }
        None
    }

    fn next_back(&mut self) -> Option<i32> {
        while self.front_pos < self.back_pos {
            self.back_pos -= 1;
            self.file
                .seek(SeekFrom::Start(self.back_pos as u64))
                .unwrap();
            let mut buffer = [0u8; 1];
            self.file.read_exact(&mut buffer).unwrap();
            // skip newline
            if buffer[0] as char != '\n' && self.back_pos % 2 == 0 {
                return Some(buffer[0].checked_sub(b'0').unwrap() as i32);
            }
        }
        None
    }
}

struct BackIdToInterpolate {
    back_id: i64,
    back_id_remain_count: i32,
}

impl BackIdToInterpolate {
    fn new() -> Self {
        BackIdToInterpolate {
            back_id: -1,
            back_id_remain_count: 0,
        }
    }

    fn peek(&self) -> (i64, i32) {
        (self.back_id, self.back_id_remain_count)
    }

    fn get(&mut self, reader: &mut DualFileReader) -> Option<i64> {
        if self.back_id_remain_count > 0 {
            self.back_id_remain_count -= 1;
            return Some(self.back_id);
        } else {
            while let Some(new_count) = reader.next_back() {
                self.back_id = reader.back_pos / 2;
                self.back_id_remain_count = new_count;
                if self.back_id_remain_count > 0 {
                    self.back_id_remain_count -= 1;
                    return Some(self.back_id);
                }
            }
            return None;
        }
    }
}

fn interpolate_from_back_and_get_sol_1(reader: &mut DualFileReader) -> i64 {
    let mut result = 0;
    let mut next_pos = 0;
    let mut back_id_to_interpolate = BackIdToInterpolate::new();
    while let Some(front) = reader.next_front() {
        if reader.front_pos % 2 == 0 {
            // here we need to consider the case when part of items have been moved
            // when the front points to it, so we only need to put the remaining ones
            let (back_id, back_id_remain_count) = back_id_to_interpolate.peek();
            let front_id_remain_count = if back_id == reader.front_pos / 2 {
                back_id_remain_count
            } else {
                front
            };
            for _ in 0..front_id_remain_count {
                result += next_pos * reader.front_pos / 2;
                next_pos += 1;
            }
        } else {
            for _ in 0..front {
                if let Some(back_id) = back_id_to_interpolate.get(reader) {
                    result += next_pos * back_id;
                    next_pos += 1;
                } else {
                    return result;
                }
            }
        }
    }
    result
}

fn main() {
    let mut reader = DualFileReader::new("9/sample.txt");
    let sol_1 = interpolate_from_back_and_get_sol_1(&mut reader);
    println!("sol 1: {sol_1}");
}
