use bundcore::bundcore::Bund;

pub mod cmd;
pub mod stdlib;

fn main() {
    let mut bc = Bund::new();
    cmd::main(&mut bc);
}
