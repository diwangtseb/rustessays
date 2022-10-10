
#[derive(Debug)]
pub struct User{
    pub(crate) id: i8,
    pub(crate) name: String
}

impl User{
    pub fn get(&self) -> i8 {
        return self.id
    }
    
    pub fn set(&mut self,id:i8,name:String){
        self.id = id;
        self.name = name
    }
}

pub trait Event {
    fn run(&self)-> Result<String,bool>;
    fn read(&self)-> bool;
}

impl Event for User {

    fn run(&self)-> Result<String,bool>{
        let rsp = String::from("run") + &self.name;
        return Ok(rsp) 
    }

    fn read(&self)-> bool {
        todo!()
    }
}

