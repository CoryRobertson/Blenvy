use std::any::TypeId;

use bevy::{prelude::*};
use bevy::platform_support::collections::HashSet;
use bevy::platform_support::hash::FixedHasher;
use blenvy::*;

/*use blenvy::*;
use blenvy::*; */

use crate::{ComponentAToFilterOut, ComponentBToFilterOut};

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        
        let mut hs = HashSet::with_hasher(FixedHasher);
        
        for a in [
            // this is using Bevy's build in SceneFilter, you can compose what components you want to allow/deny
            TypeId::of::<ComponentAToFilterOut>(),
            TypeId::of::<ComponentBToFilterOut>(),
            // and any other commponent you want to include/exclude
        ] {
            hs.insert(a);
        }
        
        app.add_plugins(
            BlenvyPlugin {
                registry_component_filter: SceneFilter::Denylist(hs),
                ..Default::default()
            }, /* ExportRegistryPlugin {
                   component_filter: SceneFilter::Denylist(HashSet::from([
                       // this is using Bevy's build in SceneFilter, you can compose what components you want to allow/deny
                       TypeId::of::<ComponentAToFilterOut>(),
                       TypeId::of::<ComponentBToFilterOut>(),
                       // and any other commponent you want to include/exclude
                   ])),
                   ..Default::default()
               },
               BlueprintsPlugin {
                   material_library: true,
                   aabbs: true,
                   ..Default::default()
               }, */
        );
    }
}
