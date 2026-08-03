use super::*;

#[derive(Subcommand)]
pub enum DevSubcommand {
}

impl DevSubcommand {
    pub fn process(self, server_list: &mut ServerList) -> Result<(), Error> {
        Ok(())
    }
}