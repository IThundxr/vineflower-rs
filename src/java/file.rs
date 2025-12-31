use crate::java::types::JNIResult;
use crate::{impl_as_jobject, jvm};
use jni::objects::{JObject, JObjectArray, JValue};
use std::path::Path;

impl_as_jobject!(JavaFile);

impl JavaFile {
    pub fn new<P: AsRef<Path>>(path: P) -> JNIResult<Self> {
        let mut env = jvm::attach_current_thread_permanently();

        let file_cls = env.find_class("java/io/File")?;
        let s = env.new_string(path.as_ref().to_string_lossy())?;
        let file = env.new_object(file_cls, "(Ljava/lang/String;)V", &[JValue::from(&s)])?;
        let global_ref = env.new_global_ref(file)?;

        Ok(JavaFile(global_ref))
    }

    pub fn new_array<'local, P: AsRef<Path>>(paths: &[P]) -> JNIResult<JObjectArray<'local>> {
        let mut env = jvm::attach_current_thread_permanently();

        let file_cls = env.find_class("java/io/File")?;
        let arr = env.new_object_array(paths.len() as i32, file_cls, JObject::null())?;

        for (i, p) in paths.iter().enumerate() {
            let file = JavaFile::new(p)?;
            env.set_object_array_element(&arr, i as i32, file)?;
        }

        Ok(arr)
    }
}
