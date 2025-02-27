use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::export::NativeClass;

static CLASS_REGISTRY: Lazy<RwLock<HashMap<TypeId, ClassInfo>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub(crate) struct ClassInfo {
    pub name: Cow<'static, str>,
}

/// Access the [`ClassInfo`] of the class `C`.
#[inline]
pub(crate) fn with_class_info<C: NativeClass, F, R>(f: F) -> Option<R>
where
    F: FnOnce(&ClassInfo) -> R,
{
    CLASS_REGISTRY.read().get(&TypeId::of::<C>()).map(f)
}

/// Returns the NativeScript name of the class `C` if it is registered.
/// Can also be used to validate whether or not `C` has been added using `InitHandle::add_class<C>()`
#[inline]
pub(crate) fn class_name<C: NativeClass>() -> Option<Cow<'static, str>> {
    with_class_info::<C, _, _>(|i| i.name.clone())
}

/// Returns the NativeScript name of the class `C` if it is registered, or a best-effort description
/// of the type otherwise.
///
/// The returned string should only be used for diagnostic purposes, not for types that the user
/// has to name explicitly. The format is not guaranteed.
#[inline]
pub(crate) fn class_name_or_default<C: NativeClass>() -> Cow<'static, str> {
    class_name::<C>().unwrap_or_else(|| Cow::Borrowed(std::any::type_name::<C>()))
}

/// Registers the class `C` in the class registry, using a custom name.
/// Returns the old `ClassInfo` if `C` was already added.
#[inline]
pub(crate) fn register_class_as<C: NativeClass>(name: Cow<'static, str>) -> Option<ClassInfo> {
    let type_id = TypeId::of::<C>();
    CLASS_REGISTRY.write().insert(type_id, ClassInfo { name })
}

/// Clears the registry
#[inline]
pub(crate) fn cleanup() {
    CLASS_REGISTRY.write().clear();
}
