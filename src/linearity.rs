use ir::*;

fn islinear(typ: &Type) -> bool {
    match typ {
        FakeLin => true,
        ArrowOnce(_, _) => true,
        _ => false,
    }
}
