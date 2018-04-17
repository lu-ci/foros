pub struct DiscordConfiguration {
    bot: bool,
    token: String,
    owners: [i64]
}

impl DiscordConfiguration {
    pub fn is_bot(&self) -> bool {
        (&self.bot).to_owned()
    }
    pub fn get_token(&self) -> String {
        (&self.token).to_owned()
    }
    pub fn get_owners(&self) -> &[i64] {
        &self.owners
    }
}

pub struct DatabaseConfiguration {
    handler: String,
    address: String,
    port: i16,
    username: String,
    password: String,
    authenticate: bool,
    database: String
}

impl DatabaseConfiguration {
    pub fn get_handler(&self) -> String {
        (&self.handler).to_owned()
    }
    pub fn get_address(&self) -> String {
        (&self.address).to_owned()
    }
    pub fn get_port(&self) -> i16 {
        (&self.port).to_owned()
    }
    pub fn get_username(&self) -> String {
        (&self.username).to_owned()
    }
    pub fn get_password(&self) -> String {
        (&self.password).to_owned()
    }
    pub fn authenticate(&self) -> bool {
        (&self.authenticate).to_owned()
    }
    pub fn get_database(&self) -> String {
        (&self.database).to_owned()
    }
}
