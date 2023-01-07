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

static MATERIALS: [Material; 4] = [
    Material::Ore,
    Material::Clay,
    Material::Obsidian,
    Material::Geode,
];

impl Material {
    pub fn each() -> &'static [Material; 4] {
        &MATERIALS
    }
}

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

// This doesn't appear to be any slower than using MATERIALS and iterating over
// the key/value pairs.
pub struct MaterialMapIterator<'a, T: Default> {
    material_map: &'a MaterialMap<T>,
    idx: usize,
}

impl<'a, T: Default> Iterator for MaterialMapIterator<'a, T> {
    type Item = (&'static Material, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.idx {
            0 => Some((&MATERIALS[0], &self.material_map.ore)),
            1 => Some((&MATERIALS[1], &self.material_map.clay)),
            2 => Some((&MATERIALS[2], &self.material_map.obsidian)),
            3 => Some((&MATERIALS[3], &self.material_map.geode)),
            _ => None,
        };
        self.idx += 1;
        result
    }
}

impl<'a, T: Default> IntoIterator for &'a MaterialMap<T> {
    type Item = (&'static Material, &'a T);
    type IntoIter = MaterialMapIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MaterialMapIterator {
            material_map: self,
            idx: 0,
        }
    }
}
