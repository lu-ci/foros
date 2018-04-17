struct DiscordConfiguration {
    bot: bool,
    token: String,
    owners: [i64]
}

struct DatabaseConfiguration {
    host: String,
    port: i64,
    user: String,
    pass: String,
    auth: bool
}