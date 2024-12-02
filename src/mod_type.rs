use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum ModType {
    Mod,
    ResourcePack,
    ShaderPack,
    ModPack,
}

impl TryFrom<i64> for ModType {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(ModType::Mod),
            12 => Ok(ModType::ResourcePack),
            6552 => Ok(ModType::ShaderPack),
            4471 => Ok(ModType::ModPack),
            _ => Err("Invalid value for ModType"),
        }
    }
}

impl<'de> Deserialize<'de> for ModType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        ModType::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl From<ModType> for i64 {
    fn from(value: ModType) -> Self {
        match value {
            ModType::Mod => 6,
            ModType::ResourcePack => 12,
            ModType::ShaderPack => 6552,
            ModType::ModPack => 4471,
        }
    }
}

impl Serialize for ModType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value: i64 = (*self).clone().into();
        serializer.serialize_i64(value)
    }
}

pub trait ModTypeExt {
    fn is_mod(&self) -> bool;
    fn is_resource_pack(&self) -> bool;
    fn is_shader_pack(&self) -> bool;
    fn is_mod_pack(&self) -> bool;
    fn to_path(&self) -> PathBuf;
}

impl ModTypeExt for ModType {
    fn is_mod(&self) -> bool {
        matches!(self, ModType::Mod)
    }

    fn is_resource_pack(&self) -> bool {
        matches!(self, ModType::ResourcePack)
    }

    fn is_shader_pack(&self) -> bool {
        matches!(self, ModType::ShaderPack)
    }

    fn is_mod_pack(&self) -> bool {
        matches!(self, ModType::ModPack)
    }

    fn to_path(&self) -> PathBuf {
        match self {
            ModType::Mod => PathBuf::from("mods"),
            ModType::ResourcePack => PathBuf::from("resourcepacks"),
            ModType::ShaderPack => PathBuf::from("shaderpacks"),
            ModType::ModPack => PathBuf::from("modpacks"),
        }
    }
}
