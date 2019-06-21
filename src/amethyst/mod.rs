
use log::*;
use amethyst_rendy;

pub fn foo() {
    info!("derp");
}

#[cfg(test)]
mod tests {
    use log::*;

    #[test]
    fn it_works() {
        info!("Yay!");
    }
}
