use crate::java::types::JNIResult;
use crate::jvm;
use jni::objects::{GlobalRef, JObject, JValue};
use std::path::Path;

pub struct JavaFile(GlobalRef);

impl JavaFile {
    pub fn new(path: &Path) -> JNIResult<Self> {
        let mut env = jvm::attach_current_thread_permanently();

        let file_cls = env.find_class("java/io/File")?;
        let s = env.new_string(path.to_string_lossy())?;
        let file = env.new_object(file_cls, "(Ljava/lang/String;)V", &[JValue::from(&s)])?;
        let global_ref = env.new_global_ref(file)?;

        Ok(JavaFile(global_ref))
    }
}

impl<'local> AsRef<JObject<'local>> for JavaFile {
    fn as_ref(&self) -> &JObject<'local> {
        &self.0
    }
}
