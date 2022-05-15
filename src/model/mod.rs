pub(crate) mod album;
pub(crate) mod artist;
pub(crate) mod context;
pub(crate) mod device;
pub(crate) mod dialog;
pub(crate) mod enums;
pub(crate) mod image;
pub(crate) mod login;
pub(crate) mod page;
pub(crate) mod playlist;
pub(crate) mod show;
pub(crate) mod table;
pub(crate) mod track;
pub(crate) mod user;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Id {
    pub id: usize,
}
