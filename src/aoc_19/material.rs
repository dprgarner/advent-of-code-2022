use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

// TODO turn into iterator on something or other
pub static MATERIALS: [Material; 4] = [
    Material::Ore,
    Material::Clay,
    Material::Obsidian,
    Material::Geode,
];

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Material::Ore => "Ore",
                Material::Clay => "Clay",
                Material::Obsidian => "Obsidian",
                Material::Geode => "Geode",
            }
        )
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MaterialMap<T>
where
    T: Default,
{
    pub ore: T,
    pub clay: T,
    pub obsidian: T,
    pub geode: T,
}

// impl<T: Default> IntoIterator for MaterialMap<T> {
//     type Item = T;
//     type IntoIter = array::IntoIter<T, 4>;

//     fn into_iter(self) -> Self::IntoIter {
//         [self.ore, self.clay, self.obsidian, self.geode].into_iter()
//     }
// }

impl<T: Default> Default for MaterialMap<T> {
    fn default() -> Self {
        MaterialMap {
            ore: Default::default(),
            clay: Default::default(),
            obsidian: Default::default(),
            geode: Default::default(),
        }
    }
}

impl<T: Default> Index<&Material> for MaterialMap<T> {
    type Output = T;

    fn index(&self, index: &Material) -> &Self::Output {
        match index {
            Material::Ore => &self.ore,
            Material::Clay => &self.clay,
            Material::Obsidian => &self.obsidian,
            Material::Geode => &self.geode,
        }
    }
}

impl<T: Default> IndexMut<&Material> for MaterialMap<T> {
    fn index_mut(&mut self, index: &Material) -> &mut Self::Output {
        match index {
            Material::Ore => &mut self.ore,
            Material::Clay => &mut self.clay,
            Material::Obsidian => &mut self.obsidian,
            Material::Geode => &mut self.geode,
        }
    }
}
