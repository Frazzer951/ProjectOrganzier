#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Sql(#[from] turbosql::Error),

    #[error(transparent)]
    TomlDes(#[from] toml::de::Error),

    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),

    #[error("config missing needed value for `{0}`")]
    ConfigMissingValue(String),

    #[error(transparent)]
    Indicatif(#[from] indicatif::style::TemplateError),
}
