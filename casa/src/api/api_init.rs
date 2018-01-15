use router::Router;

pub trait ApiInit {
    fn init_api(&mut self, router: &mut Router); 
}
