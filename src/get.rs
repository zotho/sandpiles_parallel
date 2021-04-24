pub trait Get<Idx> {
    type Output;
    fn get(&self, index: Idx) -> Option<&Self::Output>;
}

pub trait GetMut<Idx>: Get<Idx> {
    fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output>;
}
