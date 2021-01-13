//! Stripped-down repro case for a strange bug, wherein adding a new struct or enum variant can
//! surface type inference errors in completely unrelated code, even in different modules.

mod pb {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataClass {
        Unknown = 0,
        Scalar = 1,
        Tensor = 2,
        BlobSequence = 3,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SummaryMetadata {
        #[prost(enumeration = "DataClass", tag = "4")]
        pub data_class: i32,
    }
}

pub fn test() {
    let md = pb::SummaryMetadata::default();
    assert_eq!(md.data_class, pb::DataClass::BlobSequence.into());
}

// Uncomment this line to create an inference error at line 19:
//pub struct Foo(serde_json::Error);
