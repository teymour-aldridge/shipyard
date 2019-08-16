use crate::entity::Key;
use crate::sparse_array::ViewMut;
use std::any::TypeId;

pub trait ViewAddEntity {
    type Component;
    fn add_entity(self, component: Self::Component, entity: Key);
}

impl ViewAddEntity for () {
    type Component = ();
    fn add_entity(self, _: Self::Component, _: Key) {}
}

impl<T> ViewAddEntity for ViewMut<'_, T> {
    type Component = T;
    fn add_entity(mut self, component: Self::Component, entity: Key) {
        self.insert(component, entity);
    }
}

impl<T> ViewAddEntity for &mut ViewMut<'_, T> {
    type Component = T;
    fn add_entity(self, component: Self::Component, entity: Key) {
        self.insert(component, entity);
    }
}

impl<T> ViewAddEntity for (ViewMut<'_, T>,) {
    type Component = (T,);
    fn add_entity(self, component: Self::Component, entity: Key) {
        self.0.add_entity(component.0, entity);
    }
}

impl<T> ViewAddEntity for (&mut ViewMut<'_, T>,) {
    type Component = (T,);
    fn add_entity(self, component: Self::Component, entity: Key) {
        self.0.add_entity(component.0, entity);
    }
}

macro_rules! impl_view_add_entity {
    ($(($type: ident, $index: tt))+) => {
        impl<'a, $($type: 'static + Send + Sync),+> ViewAddEntity for ($(ViewMut<'_, $type>,)+) {
            type Component = ($($type,)+);
            fn add_entity(mut self, component: Self::Component, entity: Key) {
                $(
                    self.$index.insert(component.$index, entity);
                )+

                let mut type_ids = [$(TypeId::of::<$type>()),+];
                type_ids.sort_unstable();

                let mut should_pack = Vec::with_capacity(type_ids.len());
                $(
                    let type_id = TypeId::of::<$type>();

                    if should_pack.contains(&type_id) {
                        self.$index.pack(entity);
                    } else {
                        let pack_types = self.$index.should_pack_owned(&type_ids);

                        should_pack.extend(pack_types.iter().filter(|&&x| x == type_id));
                        if !pack_types.is_empty() {
                            self.$index.pack(entity);
                        }
                    }
                )+
            }
        }
        impl<'a, $($type: 'static + Send + Sync),+> ViewAddEntity for ($(&mut ViewMut<'_, $type>,)+) {
            type Component = ($($type,)+);
            fn add_entity(self, component: Self::Component, entity: Key) {
                $(
                    self.$index.insert(component.$index, entity);
                )+
            }
        }
    }
}

macro_rules! view_add_entity {
    ($(($type: ident, $index: tt))*;($type1: ident, $index1: tt) $(($queue_type: ident, $queue_index: tt))*) => {
        impl_view_add_entity![$(($type, $index))*];
        view_add_entity![$(($type, $index))* ($type1, $index1); $(($queue_type, $queue_index))*];
    };
    ($(($type: ident, $index: tt))*;) => {
        impl_view_add_entity![$(($type, $index))*];
    }
}

view_add_entity![(A, 0) (B, 1); (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)];
