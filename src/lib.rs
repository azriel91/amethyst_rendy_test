#[cfg(test)]
mod tests {
    use amethyst::Error;
    use amethyst_test::{AmethystApplication, RenderBaseAppExt};

    #[test]
    fn it_works() -> Result<(), Error> {
        AmethystApplication::render_base()
            .with_assertion(|_world| panic!("rarara"))
            .run()
    }
}
