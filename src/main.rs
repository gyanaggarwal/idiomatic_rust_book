use idiomatic_rust::domain::trait_ch02::{Crdt, EWFlag, DWFlag, Ch02Trait};

fn main() {
    let crdt1: Crdt<EWFlag> = Crdt::new("EWFlag01".to_owned());
    let crdt2: Crdt<DWFlag> = Crdt::new("DWFlag01".to_owned());

    crdt1.func04();
    crdt2.func04();

}
