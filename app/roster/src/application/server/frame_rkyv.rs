use std::convert::TryInto;
use std::fmt;
use std::io::Cursor;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use bytes::{Buf, Bytes};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug)]
pub enum FrameRkyv {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<FrameRkyv>),
}

// ========================================
// Recursive expansion of the Archive macro
// ========================================

#[automatically_derived]
///An archived [`FrameRkyv`]
#[derive(::rkyv::bytecheck::CheckBytes)]
#[check_bytes(crate = "::rkyv::bytecheck")]
#[check_bytes(bound = "__C: rkyv::validation::ArchiveContext, <__C as \
                       rkyv::Fallible>::Error: std::error::Error")]
#[repr(u8)]
pub enum ArchivedFrameRkyv
where
    String: ::rkyv::Archive,
    String: ::rkyv::Archive,
    u64: ::rkyv::Archive,
    Bytes: ::rkyv::Archive,
{
    ///The archived counterpart of [`FrameRkyv::Simple`]
    #[allow(dead_code)]
    Simple(
        ///The archived counterpart of [`FrameRkyv::Simple::0`]
        ::rkyv::Archived<String>,
    ),
    ///The archived counterpart of [`FrameRkyv::Error`]
    #[allow(dead_code)]
    Error(
        ///The archived counterpart of [`FrameRkyv::Error::0`]
        ::rkyv::Archived<String>,
    ),
    ///The archived counterpart of [`FrameRkyv::Integer`]
    #[allow(dead_code)]
    Integer(
        ///The archived counterpart of [`FrameRkyv::Integer::0`]
        ::rkyv::Archived<u64>,
    ),
    ///The archived counterpart of [`FrameRkyv::Bulk`]
    #[allow(dead_code)]
    Bulk(
        ///The archived counterpart of [`FrameRkyv::Bulk::0`]
        ::rkyv::Archived<Bytes>,
    ),
    ///The archived counterpart of [`FrameRkyv::Null`]
    #[allow(dead_code)]
    Null,
    ///The archived counterpart of [`FrameRkyv::Array`]
    #[allow(dead_code)]
    Array(
        ///The archived counterpart of [`FrameRkyv::Array::0`]
        #[omit_bounds]
        ::rkyv::Archived<Vec<FrameRkyv>>,
    ),
}
#[automatically_derived]
///The resolver for an archived [`FrameRkyv`]
pub enum FrameRkyvResolver
where
    String: ::rkyv::Archive,
    String: ::rkyv::Archive,
    u64: ::rkyv::Archive,
    Bytes: ::rkyv::Archive,
{
    ///The resolver for [`FrameRkyv::Simple`]
    #[allow(dead_code)]
    Simple(
        ///The resolver for [`FrameRkyv::Simple::0`]
        ::rkyv::Resolver<String>,
    ),
    ///The resolver for [`FrameRkyv::Error`]
    #[allow(dead_code)]
    Error(
        ///The resolver for [`FrameRkyv::Error::0`]
        ::rkyv::Resolver<String>,
    ),
    ///The resolver for [`FrameRkyv::Integer`]
    #[allow(dead_code)]
    Integer(
        ///The resolver for [`FrameRkyv::Integer::0`]
        ::rkyv::Resolver<u64>,
    ),
    ///The resolver for [`FrameRkyv::Bulk`]
    #[allow(dead_code)]
    Bulk(
        ///The resolver for [`FrameRkyv::Bulk::0`]
        ::rkyv::Resolver<Bytes>,
    ),
    ///The resolver for [`FrameRkyv::Null`]
    #[allow(dead_code)]
    Null,
    ///The resolver for [`FrameRkyv::Array`]
    #[allow(dead_code)]
    Array(
        ///The resolver for [`FrameRkyv::Array::0`]
        ::rkyv::Resolver<Vec<FrameRkyv>>,
    ),
}
#[automatically_derived]
const _: () = {
    use ::core::marker::PhantomData;
    use ::rkyv::{out_field, Archive, Archived};
    #[repr(u8)]
    enum ArchivedTag {
        Simple = b'+',
        Error,
        Integer,
        Bulk,
        Null,
        Array,
    }
    #[repr(C)]
    struct ArchivedVariantSimple(
        ArchivedTag,
        Archived<String>,
        PhantomData<FrameRkyv>,
    )
    where
        String: ::rkyv::Archive,
        String: ::rkyv::Archive,
        u64: ::rkyv::Archive,
        Bytes: ::rkyv::Archive;
    #[repr(C)]
    struct ArchivedVariantError(
        ArchivedTag,
        Archived<String>,
        PhantomData<FrameRkyv>,
    )
    where
        String: ::rkyv::Archive,
        String: ::rkyv::Archive,
        u64: ::rkyv::Archive,
        Bytes: ::rkyv::Archive;
    #[repr(C)]
    struct ArchivedVariantInteger(
        ArchivedTag,
        Archived<u64>,
        PhantomData<FrameRkyv>,
    )
    where
        String: ::rkyv::Archive,
        String: ::rkyv::Archive,
        u64: ::rkyv::Archive,
        Bytes: ::rkyv::Archive;
    #[repr(C)]
    struct ArchivedVariantBulk(
        ArchivedTag,
        Archived<Bytes>,
        PhantomData<FrameRkyv>,
    )
    where
        String: ::rkyv::Archive,
        String: ::rkyv::Archive,
        u64: ::rkyv::Archive,
        Bytes: ::rkyv::Archive;
    #[repr(C)]
    struct ArchivedVariantArray(
        ArchivedTag,
        Archived<Vec<FrameRkyv>>,
        PhantomData<FrameRkyv>,
    )
    where
        String: ::rkyv::Archive,
        String: ::rkyv::Archive,
        u64: ::rkyv::Archive,
        Bytes: ::rkyv::Archive;
    impl Archive for FrameRkyv
    where
        String: ::rkyv::Archive,
        String: ::rkyv::Archive,
        u64: ::rkyv::Archive,
        Bytes: ::rkyv::Archive,
    {
        type Archived = ArchivedFrameRkyv;
        type Resolver = FrameRkyvResolver;
        #[allow(clippy::unit_arg)]
        #[inline]
        unsafe fn resolve(
            &self,
            pos: usize,
            resolver: <Self as Archive>::Resolver,
            out: *mut <Self as Archive>::Archived,
        ) {
            match resolver {
                FrameRkyvResolver::Simple(resolver_0) => match self {
                    FrameRkyv::Simple(self_0) => {
                        dbg!(&self_0);
                        let out = out.cast::<ArchivedVariantSimple>();
                        ::core::ptr::addr_of_mut!((*out).0)
                            .write(ArchivedTag::Simple);
                        let (fp, fo) = out_field!(out.1);
                        ::rkyv::Archive::resolve(
                            self_0,
                            pos + fp,
                            resolver_0,
                            fo,
                        );
                    }
                    #[allow(unreachable_patterns)]
                    _ => ::core::hint::unreachable_unchecked(),
                },
                FrameRkyvResolver::Error(resolver_0) => match self {
                    FrameRkyv::Error(self_0) => {
                        let out = out.cast::<ArchivedVariantError>();
                        ::core::ptr::addr_of_mut!((*out).0)
                            .write(ArchivedTag::Error);
                        let (fp, fo) = out_field!(out.1);
                        ::rkyv::Archive::resolve(
                            self_0,
                            pos + fp,
                            resolver_0,
                            fo,
                        );
                    }
                    #[allow(unreachable_patterns)]
                    _ => ::core::hint::unreachable_unchecked(),
                },
                FrameRkyvResolver::Integer(resolver_0) => match self {
                    FrameRkyv::Integer(self_0) => {
                        let out = out.cast::<ArchivedVariantInteger>();
                        ::core::ptr::addr_of_mut!((*out).0)
                            .write(ArchivedTag::Integer);
                        let (fp, fo) = out_field!(out.1);
                        ::rkyv::Archive::resolve(
                            self_0,
                            pos + fp,
                            resolver_0,
                            fo,
                        );
                    }
                    #[allow(unreachable_patterns)]
                    _ => ::core::hint::unreachable_unchecked(),
                },
                FrameRkyvResolver::Bulk(resolver_0) => match self {
                    FrameRkyv::Bulk(self_0) => {
                        let out = out.cast::<ArchivedVariantBulk>();
                        ::core::ptr::addr_of_mut!((*out).0)
                            .write(ArchivedTag::Bulk);
                        let (fp, fo) = out_field!(out.1);
                        ::rkyv::Archive::resolve(
                            self_0,
                            pos + fp,
                            resolver_0,
                            fo,
                        );
                    }
                    #[allow(unreachable_patterns)]
                    _ => ::core::hint::unreachable_unchecked(),
                },
                FrameRkyvResolver::Null => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Null);
                }
                FrameRkyvResolver::Array(resolver_0) => match self {
                    FrameRkyv::Array(self_0) => {
                        let out = out.cast::<ArchivedVariantArray>();
                        ::core::ptr::addr_of_mut!((*out).0)
                            .write(ArchivedTag::Array);
                        let (fp, fo) = out_field!(out.1);
                        ::rkyv::Archive::resolve(
                            self_0,
                            pos + fp,
                            resolver_0,
                            fo,
                        );
                    }
                    #[allow(unreachable_patterns)]
                    _ => ::core::hint::unreachable_unchecked(),
                },
            }
        }
    }
};

const _: () = {
    use ::rkyv::{Archive, Fallible, Serialize};
    impl<__S: Fallible + ?Sized> Serialize<__S> for FrameRkyv
    where
        __S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer,
        String: Serialize<__S>,
        String: Serialize<__S>,
        u64: Serialize<__S>,
        Bytes: Serialize<__S>,
    {
        #[inline]
        fn serialize(
            &self,
            serializer: &mut __S,
        ) -> ::core::result::Result<<Self as Archive>::Resolver, __S::Error>
        {
            dbg!(&self);
            Ok(match self {
                Self::Simple(_0) => FrameRkyvResolver::Simple(
                    Serialize::<__S>::serialize(_0, serializer)?,
                ),
                Self::Error(_0) => FrameRkyvResolver::Error(
                    Serialize::<__S>::serialize(_0, serializer)?,
                ),
                Self::Integer(_0) => {
                    FrameRkyvResolver::Integer(Serialize::<__S>::serialize(
                        _0, serializer,
                    )?)
                }
                Self::Bulk(_0) => FrameRkyvResolver::Bulk(
                    Serialize::<__S>::serialize(_0, serializer)?,
                ),
                Self::Null => FrameRkyvResolver::Null,
                Self::Array(_0) => FrameRkyvResolver::Array(
                    Serialize::<__S>::serialize(_0, serializer)?,
                ),
            })
        }
    }
};

#[cfg(test)]
mod test {
    use rkyv::check_archived_root;

    use super::FrameRkyv;

    #[test]
    fn test_rkyv() {
        let f = FrameRkyv::Simple("PONG".to_string());
        let buf = rkyv::to_bytes::<_, 1024>(&f).unwrap();

        let a = buf.to_vec();
        let b = String::from_utf8(a.clone()).unwrap();
        dbg!(&b);
        // assert_eq!(b, "+PONG");

        let ping_test = b"+OK\r\n";
        let a = check_archived_root::<FrameRkyv>(a.as_slice());
        a.unwrap();
    }
}
