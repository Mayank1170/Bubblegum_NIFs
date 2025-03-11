#[rustler::nif]
fn test_nif() -> i64 {
    42
}


rustler::init!("Elixir.BubblegumElixir.Bubblegum", [test_nif]);
