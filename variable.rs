

#[derive(Debug)]
pub struct User{
    pub(crate) id: i8,
    pub(crate) name: String
}

impl User{
    pub fn get(&self) -> i8 {
        return self.id
    }
    
    pub fn set(&mut self,id:i8){
        self.id = id
    }
}

