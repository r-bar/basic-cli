// ⚠️ GENERATED CODE ⚠️ - this entire file was generated by the `roc glue` CLI command

#![allow(unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::clone_on_copy)]


#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, )]
#[repr(C)]
pub struct GetMetadataErr_Unrecognized {
    pub f1: roc_std::RocStr,
    pub f0: i32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, )]
#[repr(u8)]
pub enum discriminant_GetMetadataErr {
    PathDoesNotExist = 0,
    PermissionDenied = 1,
    Unrecognized = 2,
}

impl core::fmt::Debug for discriminant_GetMetadataErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PathDoesNotExist => f.write_str("discriminant_GetMetadataErr::PathDoesNotExist"),
            Self::PermissionDenied => f.write_str("discriminant_GetMetadataErr::PermissionDenied"),
            Self::Unrecognized => f.write_str("discriminant_GetMetadataErr::Unrecognized"),
        }
    }
}

#[repr(C, align(8))]
pub union union_GetMetadataErr {
    PathDoesNotExist: (),
    PermissionDenied: (),
    Unrecognized: core::mem::ManuallyDrop<GetMetadataErr_Unrecognized>,
}


impl GetMetadataErr {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_GetMetadataErr {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_GetMetadataErr>(*bytes.as_ptr().add(32))
        }
    }

    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_GetMetadataErr) {
        let discriminant_ptr: *mut discriminant_GetMetadataErr = (self as *mut GetMetadataErr).cast();

        unsafe {
            *(discriminant_ptr.add(32)) = discriminant;
        }
    }
}

#[repr(C)]
pub struct GetMetadataErr {
    payload: union_GetMetadataErr,
    discriminant: discriminant_GetMetadataErr,
}

impl Clone for GetMetadataErr {
    fn clone(&self) -> Self {
        use discriminant_GetMetadataErr::*;

        let payload = unsafe {
            match self.discriminant {
                PathDoesNotExist => union_GetMetadataErr {
                    PathDoesNotExist: self.payload.PathDoesNotExist.clone(),
                },
                PermissionDenied => union_GetMetadataErr {
                    PermissionDenied: self.payload.PermissionDenied.clone(),
                },
                Unrecognized => union_GetMetadataErr {
                    Unrecognized: self.payload.Unrecognized.clone(),
                },
            }
        };

        Self {
            discriminant: self.discriminant,
            payload,
        }
    }
}

impl core::fmt::Debug for GetMetadataErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use discriminant_GetMetadataErr::*;

        unsafe {
            match self.discriminant {
                PathDoesNotExist => {
                    let field: &() = &self.payload.PathDoesNotExist;
                    f.debug_tuple("GetMetadataErr::PathDoesNotExist").field(field).finish()
                },
                PermissionDenied => {
                    let field: &() = &self.payload.PermissionDenied;
                    f.debug_tuple("GetMetadataErr::PermissionDenied").field(field).finish()
                },
                Unrecognized => {
                    let field: &GetMetadataErr_Unrecognized = &self.payload.Unrecognized;
                    f.debug_tuple("GetMetadataErr::Unrecognized").field(field).finish()
                },
            }
        }
    }
}

impl Eq for GetMetadataErr {}

impl PartialEq for GetMetadataErr {
    fn eq(&self, other: &Self) -> bool {
        use discriminant_GetMetadataErr::*;

        if self.discriminant != other.discriminant {
            return false;
        }

        unsafe {
            match self.discriminant {
                PathDoesNotExist => self.payload.PathDoesNotExist == other.payload.PathDoesNotExist,
                PermissionDenied => self.payload.PermissionDenied == other.payload.PermissionDenied,
                Unrecognized => self.payload.Unrecognized == other.payload.Unrecognized,
            }
        }
    }
}

impl Ord for GetMetadataErr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for GetMetadataErr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use discriminant_GetMetadataErr::*;

        use std::cmp::Ordering::*;

        match self.discriminant.cmp(&other.discriminant) {
            Less => Option::Some(Less),
            Greater => Option::Some(Greater),
            Equal => unsafe {
                match self.discriminant {
                    PathDoesNotExist => self.payload.PathDoesNotExist.partial_cmp(&other.payload.PathDoesNotExist),
                    PermissionDenied => self.payload.PermissionDenied.partial_cmp(&other.payload.PermissionDenied),
                    Unrecognized => self.payload.Unrecognized.partial_cmp(&other.payload.Unrecognized),
                }
            },
        }
    }
}

impl core::hash::Hash for GetMetadataErr {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use discriminant_GetMetadataErr::*;

        unsafe {
            match self.discriminant {
                PathDoesNotExist => self.payload.PathDoesNotExist.hash(state),
                PermissionDenied => self.payload.PermissionDenied.hash(state),
                Unrecognized => self.payload.Unrecognized.hash(state),
            }
        }
    }
}

impl GetMetadataErr {

    pub fn is_PathDoesNotExist(&self) -> bool {
        matches!(self.discriminant, discriminant_GetMetadataErr::PathDoesNotExist)
    }

    pub fn is_PermissionDenied(&self) -> bool {
        matches!(self.discriminant, discriminant_GetMetadataErr::PermissionDenied)
    }

    pub fn unwrap_Unrecognized(mut self) -> GetMetadataErr_Unrecognized {
        debug_assert_eq!(self.discriminant, discriminant_GetMetadataErr::Unrecognized);
        unsafe { core::mem::ManuallyDrop::take(&mut self.payload.Unrecognized) }
    }

    pub fn is_Unrecognized(&self) -> bool {
        matches!(self.discriminant, discriminant_GetMetadataErr::Unrecognized)
    }
}



impl GetMetadataErr {

    pub fn PathDoesNotExist() -> Self {
        Self {
            discriminant: discriminant_GetMetadataErr::PathDoesNotExist,
            payload: union_GetMetadataErr {
                PathDoesNotExist: (),
            }
        }
    }

    pub fn PermissionDenied() -> Self {
        Self {
            discriminant: discriminant_GetMetadataErr::PermissionDenied,
            payload: union_GetMetadataErr {
                PermissionDenied: (),
            }
        }
    }

    pub fn Unrecognized(payload: GetMetadataErr_Unrecognized) -> Self {
        Self {
            discriminant: discriminant_GetMetadataErr::Unrecognized,
            payload: union_GetMetadataErr {
                Unrecognized: core::mem::ManuallyDrop::new(payload),
            }
        }
    }
}

impl Drop for GetMetadataErr {
    fn drop(&mut self) {
        // Drop the payloads
        match self.discriminant() {
            discriminant_GetMetadataErr::PathDoesNotExist => {}
            discriminant_GetMetadataErr::PermissionDenied => {}
            discriminant_GetMetadataErr::Unrecognized => unsafe { core::mem::ManuallyDrop::drop(&mut self.payload.Unrecognized) },
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, )]
#[repr(C)]
pub struct InternalPathType {
    pub isDir: bool,
    pub isFile: bool,
    pub isSymLink: bool,
}
