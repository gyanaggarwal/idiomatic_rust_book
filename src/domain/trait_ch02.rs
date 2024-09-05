pub trait Ch02Trait {
    fn func01(&self);
    fn func02(&self) {
        println!("this is default implementation");
    }
    fn func03(&self) {
        println!("this is final implemenattion");
    }
    fn func04(&self) {
        self.func01();
        self.func02();
        self.func03();
    }
}

pub struct DWFlag;
pub struct EWFlag;

pub struct Crdt<State> {
    pub name: String,
    pub state: std::marker::PhantomData<State>
}

impl <State> Crdt<State> {
    pub fn new(name: String) -> Self {
        Self{name, state: std::marker::PhantomData::<State>}
    }
}

impl Ch02Trait for Crdt<DWFlag> {
    fn func01(&self) {
        println!("DWFlag: {:?}", self.name);
    }

    fn func02(&self) {
        println!("DWFlag: this the final implementation");
    }
}

impl Ch02Trait for Crdt<EWFlag> {
    fn func01(&self) {
        println!("EWFlag: {:?}", self.name);
    }
}