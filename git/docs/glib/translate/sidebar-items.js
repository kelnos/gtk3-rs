initSidebarItems({"enum":[["GlibNoneOrInvalidError","Error type for [`TryFromGlib`] when the Glib value can be None or invalid."]],"fn":[["c_ptr_array_len",""],["const_override","Overrides pointer constness."],["from_glib","Translate a simple type."],["from_glib_borrow","Translate from a pointer type, borrowing the pointer."],["from_glib_full","Translate from a pointer type, transfer: full (assume ownership)."],["from_glib_none","Translate from a pointer type, transfer: none."],["mut_override","Overrides pointer mutability."],["uninitialized","Returns an uninitialized value."]],"struct":[["Borrowed","Wrapper around values representing borrowed C memory."],["GlibNoneError","Error type for [`TryFromGlib`] when the Glib value is None."],["HashTable",""],["List",""],["PtrArray",""],["SList",""],["Stash","Helper type that stores temporary values used for translation."],["StashMut",""]],"trait":[["FromGlib","Translate a simple type."],["FromGlibContainer","Translate from a container."],["FromGlibContainerAsVec",""],["FromGlibPtrArrayContainerAsVec",""],["FromGlibPtrBorrow","Translate from a pointer type by borrowing, without affecting the refcount."],["FromGlibPtrContainer","Translate from a container of pointers."],["FromGlibPtrFull","Translate from a pointer type which is annotated with `transfer full`. This transfers the ownership of the value to the Rust side."],["FromGlibPtrNone","Translate from a pointer type which is annotated with `transfer none`. The resulting value is referenced at least once, by the bindings."],["GlibPtrDefault","Provides the default pointer type to be used in some container conversions."],["OptionToGlib","A Rust type `T` for which `Option<T>` translates to the same glib type as T."],["Ptr","A pointer"],["ToGlib","Translate a simple type."],["ToGlibContainerFromSlice",""],["ToGlibPtr","Translate to a pointer."],["ToGlibPtrMut","Translate to a pointer with a mutable borrow."],["TryFromGlib","Translate from a Glib type which can result in an undefined and/or invalid value."],["Uninitialized","A trait for creating an uninitialized value. Handy for receiving outparams."]]});