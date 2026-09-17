use std::{fs::{File, OpenOptions}, io::{self, Read, Seek, Write}, path::Path};

const PAGE_SIZE: u64 = 4096;

#[derive(Clone, Copy)]
pub struct PageId(u64);

impl PageId{
    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let current_page_size = heap_file.metadata()?.len();
        let next_page_id = current_page_size / PAGE_SIZE;
        Ok(Self { heap_file, next_page_id })
    }

    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
        Self::new(heap_file)
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = PageId(self.next_page_id);
        self.next_page_id += 1;
        page_id
    }

    pub fn read_page_data(&mut self, page_id: PageId, buf: &mut [u8]) -> io::Result<()> {
        let pos = page_id.to_u64() * PAGE_SIZE;
        self.heap_file.seek(io::SeekFrom::Start(pos))?;
        self.heap_file.read_exact(buf)?;
        Ok(())
    }

    pub fn write_page_data(&mut self, page_id: PageId, buf: &[u8]) -> io::Result<()>{
        let pos = page_id.to_u64() * PAGE_SIZE;
        self.heap_file.seek(io::SeekFrom::Start(pos))?;
        self.heap_file.write_all(buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_manager() {
        let path = "test.db";
        let _ = std::fs::remove_file(path);

        let mut dskmng = DiskManager::open(path).unwrap();

        // pattern 1
        let page0 = dskmng.allocate_page();
        let data1 = b"Hello, world!";
        dskmng.write_page_data(page0,data1).unwrap();
        let mut buf = vec![0u8; data1.len()];
        dskmng.read_page_data(page0, &mut buf).unwrap();
        assert_eq!(String::from_utf8_lossy(&buf), "Hello, world!");
        
        
        // pattern 2
        let page1 = dskmng.allocate_page();
        let data2 = b"I'm, P1ned0g!!";
        dskmng.write_page_data(page1,data2).unwrap();
        let mut buf = vec![0u8; data2.len()];
        dskmng.read_page_data(page1, &mut buf).unwrap();
        assert_eq!(String::from_utf8_lossy(&buf), "I'm, P1ned0g!!");


        // let _file = File::create(FILE_NAME).unwrap();

        // let _ = std::fs::remove_file(FILE_NAME);

        // let file = OpenOptions::new()
        //     .read(true)
        //     .write(true)
        //     .create(true)
        //     .open(FILE_NAME).unwrap();

        // let mut disk = DiskManager::new(file).unwrap();

        // // pattern 1
        // let data = b"Hello, world!";
        // disk.write_page_data(0, data).unwrap();
        // let mut buf = vec![0u8; data.len()];
        // disk.read_page_data(0, &mut buf).unwrap();
        // assert_eq!(String::from_utf8_lossy(&buf), "Hello, world!");

        // // pattern 2
        // let data = b"I'm, P1ned0g!!";
        // disk.write_page_data(1, data).unwrap();
        // let mut buf = vec![0u8; data.len()];
        // disk.read_page_data(1, &mut buf).unwrap();
        // assert_eq!(String::from_utf8_lossy(&buf), "I'm, P1ned0g!!");
    }
}