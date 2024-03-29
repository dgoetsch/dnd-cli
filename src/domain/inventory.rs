use crate::render::Render;
use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Inventory {
    #[serde(default)]
    items: HashMap<String, InventoryItem>,
}

pub enum RemoveItemResult {
    InsufficientInventory {
        name: String,
        requested: isize,
        available: isize,
    },
    ContainerNotRemoved {
        name: String,
    },
    NoSuchItem {
        name: String,
        requested: isize,
    },
    Success {
        name: String,
        requested: isize,
        available: isize,
    },
}

impl Render for (&String, &InventoryItem) {
    fn render(&self, indent: usize, out: &mut dyn Write) -> Result<()> {
        match self {
            (name, InventoryItem::Object { count }) => {
                out.write_fmt(format_args!(
                    "{}x{} ... {}\n",
                    Render::tab(indent),
                    count,
                    name
                ))?;
            }
            (name, InventoryItem::Container { items }) => {
                out.write_fmt(format_args!("{}{}:\n", Render::tab(indent), name))?;
                for name_item in items.iter().sorted_by_key(|(n, i)| n.clone()) {
                    name_item.render(indent + 1, out)?;
                }
            }
        };
        Ok(())
    }
}
impl Render for Inventory {
    fn render(&self, indent: usize, out: &mut Write) -> Result<()> {
        out.write_fmt(format_args!("{}Inventory\n", Render::tab(indent)))?;
        for named_item in self.items.iter().sorted_by_key(|(n, i)| n.clone()) {
            named_item.render(indent + 1, out)?;
        }
        out.flush();
        Ok(())
    }
}

fn path_string(path: &Vec<String>) -> String {
    path.iter().join(" / ")
}
impl Render for AddItemResult {
    fn render(&self, indent: usize, out: &mut dyn Write) -> Result<()> {
        match self {
            AddItemResult::InsufficientInventory {
                path,
                requested,
                available,
            } => {
                if available <= &0 {
                    out.write_fmt(format_args!(
                        "{}{}: don't have any\n",
                        Render::tab(indent),
                        path_string(path)
                    ))?;
                } else {
                    out.write_fmt(format_args!(
                        "{}{}: Tried to use {}, but only have {}\n",
                        Render::tab(indent),
                        path_string(path),
                        -requested,
                        available
                    ))?;
                }
            }
            AddItemResult::Success {
                path,
                requested,
                available,
            } => {
                if requested < &0 {
                    out.write_fmt(format_args!(
                        "{}{} : used {}, {} remaining\n",
                        Render::tab(indent),
                        path_string(path),
                        -requested,
                        available
                    ))?;
                } else {
                    out.write_fmt(format_args!(
                        "{}{}: added {}, {} remaining\n",
                        Render::tab(indent),
                        path_string(path),
                        requested,
                        available
                    ))?;
                }
            }
            AddItemResult::NoSuchItem { path } => {
                out.write_fmt(format_args!(
                    "{}{}: there was nothing there\n",
                    Render::tab(indent),
                    path_string(path)
                ))?;
            }
            AddItemResult::InvalidPath { path } => {
                out.write_fmt(format_args!(
                    "{}{}: path is invalid\n",
                    Render::tab(indent),
                    path_string(path)
                ))?;
            }
            AddItemResult::CannotAddOrRemoveContainer { path } => {
                out.write_fmt(format_args!(
                    "{}{}: The specified path is a container\n",
                    Render::tab(indent),
                    path_string(path)
                ))?;
            }
            AddItemResult::ContainerDoesNotExistFor { path } => {
                out.write_fmt(format_args!(
                    "{}{}: The specified container does not exist and needs to be created\n",
                    Render::tab(indent),
                    path_string(path)
                ))?;
            }
            AddItemResult::ObjectAtSubpath { path } => {
                out.write_fmt(format_args!("{}{}: The specified container does not exist, there was an object at a subpath\n", Render::tab(indent), path_string(path)))?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AddItemResult {
    Success {
        path: Vec<String>,
        requested: isize,
        available: isize,
    },
    InvalidPath {
        path: Vec<String>,
    },
    NoSuchItem {
        path: Vec<String>,
    },
    CannotAddOrRemoveContainer {
        path: Vec<String>,
    },
    ContainerDoesNotExistFor {
        path: Vec<String>,
    },
    InsufficientInventory {
        path: Vec<String>,
        requested: isize,
        available: isize,
    },
    ObjectAtSubpath {
        path: Vec<String>,
    },
}

impl AddItemResult {
    fn with_path(&self, path: Vec<String>) -> AddItemResult {
        match self {
            AddItemResult::Success {
                requested,
                available,
                ..
            } => AddItemResult::Success {
                path,
                requested: requested.clone(),
                available: available.clone(),
            },
            AddItemResult::InvalidPath { .. } => AddItemResult::InvalidPath { path },
            AddItemResult::CannotAddOrRemoveContainer { .. } => {
                AddItemResult::CannotAddOrRemoveContainer { path }
            }
            AddItemResult::ContainerDoesNotExistFor { .. } => {
                AddItemResult::ContainerDoesNotExistFor { path }
            }
            AddItemResult::NoSuchItem { .. } => AddItemResult::NoSuchItem { path },
            AddItemResult::ObjectAtSubpath { .. } => AddItemResult::ObjectAtSubpath { path },
            AddItemResult::InsufficientInventory {
                requested,
                available,
                ..
            } => AddItemResult::InsufficientInventory {
                path,
                requested: requested.clone(),
                available: available.clone(),
            },
        }
    }
}

pub enum AddContainerResult {
    ExpectedContainer { path: Vec<String> },
    Success { path: Vec<String> },
    Collision { path: Vec<String> },
    NoSuchParent { path: Vec<String> },
    PathIsEmpty { path: Vec<String> }
}

impl AddContainerResult {
    fn with_path(&self, path: Vec<String>) -> AddContainerResult {
        match self {
            AddContainerResult::ExpectedContainer { .. } => AddContainerResult::ExpectedContainer { path },
            AddContainerResult::Success { .. } => AddContainerResult::Success { path },
            AddContainerResult::Collision { .. } => AddContainerResult::Collision { path },
            AddContainerResult::NoSuchParent { .. } => AddContainerResult::NoSuchParent { path },
            AddContainerResult::PathIsEmpty { .. } => AddContainerResult::PathIsEmpty { path }
        }

    }
}

impl Render for AddContainerResult {
    fn render(&self, indent: usize, out: &mut dyn Write) -> Result<()> {
        match self {
            AddContainerResult::ExpectedContainer { path } => {
                out.write_fmt(format_args!("{}{}: Found an object in subpath\n", Render::tab(indent), path_string(path)))?;
                Ok(())
            }
            AddContainerResult::Success { path } => {
                out.write_fmt(format_args!("{}{}: Successfully created container\n", Render::tab(indent), path_string(path)))?;
                Ok(())
            }
            AddContainerResult::Collision { path } => {
                out.write_fmt(format_args!("{}{}: Cannot create an item that already exists\n", Render::tab(indent), path_string(path)))?;
                Ok(())
            }
            AddContainerResult::NoSuchParent { path } => {
                out.write_fmt(format_args!("{}{}: The container to put this in doesn't exist\n", Render::tab(indent), path_string(path)))?;
                Ok(())
            }
            AddContainerResult::PathIsEmpty { path } => {
                out.write_fmt(format_args!("{}{}: No path was provided\n", Render::tab(indent), path_string(path)))?;
                Ok(())
            }
        }

    }
}
impl Inventory {

    pub fn new(items: HashMap<String, InventoryItem>) -> Inventory {
        Inventory { items }
    }
    pub fn items(&self) -> &HashMap<String, InventoryItem> {
        &self.items
    }

    pub fn add_item(&mut self, path: Vec<String>, count: isize) -> Result<AddItemResult> {
        if let Some(name) = path.first() {
            let child_path = path
                .iter()
                .skip(1)
                .map(|s| s.clone())
                .collect::<Vec<String>>();
            let requested = count;

            let result = self
                .items
                .get_mut(name)
                .map(|inventory_item| {
                    let path = path.clone();
                    let child_path = child_path.clone();
                    if child_path.is_empty() {
                        match inventory_item {
                            InventoryItem::Object { count } => {
                                if count.clone() + requested < 0 {
                                    Ok(AddItemResult::InsufficientInventory {
                                        path,
                                        requested,
                                        available: count.clone(),
                                    })
                                } else {
                                    *count += requested;
                                    Ok(AddItemResult::Success {
                                        path,
                                        requested,
                                        available: count.clone(),
                                    })
                                }
                            }
                            _ => Ok(AddItemResult::CannotAddOrRemoveContainer { path }),
                        }
                    } else {
                        let result = inventory_item.add_item(child_path, count)?.with_path(path);
                        Ok(result)
                    }
                })
                .unwrap_or_else(|| {
                    let path = path.clone();
                    if (child_path.clone().is_empty()) {
                        self.items
                            .insert(name.clone(), InventoryItem::Object { count });
                        Ok(AddItemResult::Success {
                            path,
                            requested: count,
                            available: count,
                        })
                    } else {
                        Ok(AddItemResult::ContainerDoesNotExistFor { path })
                    }
                });

            match &result {
                Ok(AddItemResult::Success { available, .. }) => {
                    // if (available <= &0 && child_path.is_empty()) {
                    //     self.items.remove(name);
                    // }
                }
                _ => {}
            }
            result
        } else {
            Ok(AddItemResult::InvalidPath { path })
        }
    }

    pub fn add_container(&mut self, path: Vec<String>) -> Result<AddContainerResult> {
        if let Some(first) = path.first() {
            let child_path = path
                .iter()
                .skip(1)
                .map(|s| s.clone())
                .collect::<Vec<String>>();

            match self.items.get_mut(first) {
                Some(item) => {
                 if(child_path.is_empty()) {
                     Ok(AddContainerResult::Collision { path })
                 } else {
                     let result = item.add_container(child_path)?.with_path(path);
                     Ok(result)
                 }
                },
                None => {
                    if(child_path.is_empty()) {
                        self.items.insert(first.clone(), InventoryItem::Container { items: HashMap::new() });
                        Ok(AddContainerResult::Success { path })
                    } else {
                        Ok(AddContainerResult::NoSuchParent { path })
                    }
                }
            }
        } else {
            Ok(AddContainerResult::PathIsEmpty { path })
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum InventoryItem {
    Object {
        count: isize,
    },
    Container {
        items: HashMap<String, InventoryItem>,
    },
}

impl InventoryItem {
    pub fn add_item(&mut self, path: Vec<String>, count: isize) -> Result<AddItemResult> {
        if let Some(first) = path.first() {
            let child_path = path
                .iter()
                .skip(1)
                .map(|s| s.clone())
                .collect::<Vec<String>>();
            let requested = count;

            match self {
                InventoryItem::Object { .. } => Ok(AddItemResult::ObjectAtSubpath { path: path }),
                InventoryItem::Container { items } => {
                    let result = items
                        .get_mut(first)
                        .map(|item| item.add_item(child_path.clone(), requested))
                        .unwrap_or_else(|| {
                            let path = path.clone();
                            if (child_path.clone().is_empty()) {
                                items.insert(
                                    first.clone(),
                                    InventoryItem::Object { count: requested },
                                );
                                Ok(AddItemResult::Success {
                                    path,
                                    requested: requested,
                                    available: requested,
                                })
                            } else {
                                Ok(AddItemResult::ContainerDoesNotExistFor { path: path })
                            }
                        });
                    match &result {
                        Ok(AddItemResult::Success {
                            requested,
                            available,
                            ..
                        }) => {
                            // if (available <= &0 && child_path.is_empty()) {
                            //     items.remove(first);
                            // }
                        }
                        _ => {}
                    }

                    result
                }
            }
        } else {
            let requested = count;
            match self {
                InventoryItem::Object { count } => {
                    if count.clone() + requested < 0 {
                        Ok(AddItemResult::InsufficientInventory {
                            path,
                            requested,
                            available: count.clone(),
                        })
                    } else {
                        *count += requested;
                        Ok(AddItemResult::Success {
                            path,
                            requested,
                            available: count.clone(),
                        })
                    }
                }
                InventoryItem::Container { .. } => {
                    Ok(AddItemResult::CannotAddOrRemoveContainer { path })
                }
            }
        }
    }

    fn add_container(&mut self, path: Vec<String>) -> Result<AddContainerResult> {
        match self {
            InventoryItem::Container { items } => {
                if let Some(name) = path.first() {
                    let child_path = path.iter().skip(1).map(|s| s.clone()).collect::<Vec<String>>();
                    if(child_path.is_empty()) {
                        match items.get(name) {
                            None => {
                                items.insert(name.clone(), InventoryItem::Container { items: HashMap::new() });
                                Ok(AddContainerResult::Success { path })
                            },
                            _ => Ok(AddContainerResult::Collision { path })

                        }
                    } else {
                        match items.get_mut(name) {
                            None => Ok(AddContainerResult::NoSuchParent { path }),
                            Some(item) => item.add_container(child_path)
                        }
                    }
                } else {
                    Ok(AddContainerResult::PathIsEmpty { path })
                }
            },
            InventoryItem::Object { .. } => Ok(AddContainerResult::ExpectedContainer { path })
        }
    }
}