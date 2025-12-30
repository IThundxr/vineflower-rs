#[macro_export]
macro_rules! impl_as_jobject {
    ($name:ident) => {
        pub struct $name(jni::objects::GlobalRef);

        impl<'local> AsRef<jni::objects::JObject<'local>> for $name {
            fn as_ref(&self) -> &jni::objects::JObject<'local> {
                &self.0
            }
        }
    };
    ($vis:vis $name:ident) => {
        $vis struct $name(jni::objects::GlobalRef);

        impl<'local> AsRef<jni::objects::JObject<'local>> for $name {
            fn as_ref(&self) -> &jni::objects::JObject<'local> {
                &self.0
            }
        }
    };
}
