use idiomatic_rust::domain::trait_ch02::{Crdt, CRDT, 
                                         EWFlag, DWFlag, 
                                         Ch02Trait, Ch02TraitGeneric};

fn main() {
    let crdt1: Crdt<EWFlag> = Crdt::new("EWFlag01".to_owned());
    let crdt2: Crdt<DWFlag> = Crdt::new("DWFlag01".to_owned());

    crdt1.func04();
    crdt2.func04();

    let crdt3: CRDT<String, EWFlag> = CRDT::new("EWF.name".to_owned(),
                                                "EWF.alt_name".to_owned(),
                                                "EWF.final_name".to_owned());
    let crdt4: CRDT<String, DWFlag> = CRDT::new("DWF.name".to_owned(),
                                                "DWF.alt_name".to_owned(),
                                                "DWF.final_name".to_owned());
    let r3 = crdt3.gfun04("XXX.new_name".to_owned());
    let r4 = crdt4.gfun04("YYY.new_name".to_owned());

    println!("r3 {:?}", r3);
    println!("r4 {:?}", r4);
}
