pub trait RangeReader {
    fn read_range(
        &mut self,
        offset: u32,
        length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, Box<dyn std::error::Error>>>;
}

pub trait RangeReaderSync {
    fn read_range(
        &mut self,
        offset: u32,
        length: usize,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}
