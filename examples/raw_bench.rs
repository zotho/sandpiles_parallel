use sandpiles_parallel::Field;

fn main() {
    let mut field: Field<u32> = Field::new(1000, 1000);
    field.data[500_000] = 100;
    for _ in 0..1000 {
        field.update_iter_branchless();
    }
}