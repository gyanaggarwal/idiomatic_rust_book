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

pub trait Ch02TraitGeneric<T> {
    fn gfun01(&self) -> T;
    fn gfun02(&self, alt_name: T) -> T {
        alt_name
    }
    fn gfun03(&self) -> T;
    fn gfun04(&self, alt_name: T) -> Vec<T> {
        vec![self.gfun01(), self.gfun02(alt_name), self.gfun03()]
    }
}

#[derive(Clone)]
pub struct CRDT<T, State> {
    pub name: T,
    pub alt_name: T,
    pub f_name: T,
    pub state: std::marker::PhantomData<State>
}

impl <T: Clone, State> CRDT<T, State> {
    pub fn new(name: T, alt_name: T, f_name: T) -> Self {
        Self{name, alt_name, f_name, state: std::marker::PhantomData::<State>}
    }

    pub fn final_name(&self) -> T {
        self.f_name.clone()
    }
}

impl <T: Clone> Ch02TraitGeneric<T> for CRDT<T, EWFlag> {
    fn gfun01(&self) -> T {
        self.name.clone()
    }
    fn gfun03(&self) -> T {
        self.final_name()
    }
}

impl <T: Clone> Ch02TraitGeneric<T> for CRDT<T, DWFlag> {
    fn gfun01(&self) -> T {
        self.name.clone()
    }

    fn gfun02(&self, _alt_name: T) -> T {
        self.alt_name.clone()
    }
    
    fn gfun03(&self) -> T {
        self.final_name()
    }
}