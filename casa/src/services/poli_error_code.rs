pub struct PoliErrorCode(String);

impl PoliErrorCode {
    pub fn get_type(): Option<PoliErrorCodeType> {
        let val = try!(u32::parse(self.0));
        unimplemented!()
    }
}
