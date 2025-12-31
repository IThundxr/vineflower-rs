pub mod preferences;

use crate::java::file::JavaFile;
use crate::{impl_as_jobject, jvm};
use jni::JNIEnv;
use jni::descriptors::Desc;
use jni::objects::{JClass, JObject, JValue};
use jni::strings::JNIString;
use std::path::Path;

impl_as_jobject!(Decompiler);
impl_as_jobject!(DecompilerBuilder);

impl Decompiler {
    pub fn builder() -> DecompilerBuilder {
        DecompilerBuilder::new()
    }

    pub fn decompile(&self) {
        let mut env = jvm::attach_current_thread_permanently();

        env.call_method(self, "decompile", "()V", &[]).unwrap();
    }
}

impl DecompilerBuilder {
    pub fn new() -> DecompilerBuilder {
        let mut env = jvm::attach_current_thread_permanently();

        let builder = env
            .new_object(
                "org/jetbrains/java/decompiler/api/Decompiler$Builder",
                "()V",
                &[],
            )
            .unwrap();

        let global = env.new_global_ref(builder).unwrap();
        DecompilerBuilder(global)
    }

    pub fn inputs<P: AsRef<Path>>(&self, paths: &[P]) -> &Self {
        let mut env = jvm::attach_current_thread_permanently();

        let arr = JavaFile::new_array(paths).unwrap();

        env.call_method(
            self,
            "inputs",
            "([Ljava/io/File;)Lorg/jetbrains/java/decompiler/api/Decompiler$Builder;",
            &[JValue::from(&arr)],
        )
        .unwrap();

        self
    }

    pub fn output_to_directory<P: AsRef<Path>>(&self, path: P) -> &Self {
        let file = JavaFile::new(path).unwrap();

        self.output(
            "org/jetbrains/java/decompiler/main/decompiler/DirectoryResultSaver",
            "(Ljava/io/File;)V",
            &[JValue::from(&file)],
        )
    }

    pub fn output_to_file<P: AsRef<Path>>(&self, path: P) -> &Self {
        let file = JavaFile::new(path).unwrap();

        self.output(
            "org/jetbrains/java/decompiler/main/decompiler/SingleFileSaver",
            "(Ljava/io/File;)V",
            &[JValue::from(&file)],
        )
    }

    fn output<'local, 'other_local, T, U>(
        &self,
        class: T,
        ctor_sig: U,
        ctor_args: &[JValue],
    ) -> &Self
    where
        T: Desc<'local, JClass<'other_local>>,
        U: Into<JNIString> + AsRef<str>,
    {
        let mut env = jvm::attach_current_thread_permanently();

        let saver = env.new_object(class, ctor_sig, ctor_args).unwrap();

        env.call_method(
            self,
            "output",
            "(Lorg/jetbrains/java/decompiler/main/extern/IResultSaver;)Lorg/jetbrains/java/decompiler/api/Decompiler$Builder;",
            &[JValue::from(&saver)],
        ).unwrap();

        self
    }

    pub fn option(&self, key: &str, value: PreferenceValue) -> &Self {
        let env = jvm::attach_current_thread_permanently();

        let key = env.new_string(key).unwrap();
        let key = JValue::from(&key);

        match value {
            PreferenceValue::Boolean(val) => call_method(self, env, key, JValue::from(val)),
            PreferenceValue::Integer(val) => call_method(self, env, key, JValue::from(val)),
            PreferenceValue::String(val) => {
                let str = env.new_string(val).unwrap();
                call_method(self, env, key, JValue::from(&str));
            }
        };

        return self;

        // ---

        fn call_method(builder: &DecompilerBuilder, mut env: JNIEnv, key: JValue, value: JValue) {
            env.call_method(
                builder,
                "inputs",
                "(Ljava/lang/String;Ljava/lang/Object;)Lorg/jetbrains/java/decompiler/api/Decompiler$Builder;",
                &[key, value],
            )
                .unwrap();
        }
    }

    pub fn libraries<P: AsRef<Path>>(&self, paths: &[P]) -> &Self {
        let mut env = jvm::attach_current_thread_permanently();

        let arr = JavaFile::new_array(paths).unwrap();

        env.call_method(
            self,
            "libraries",
            "([Ljava/io/File;)Lorg/jetbrains/java/decompiler/api/Decompiler$Builder;",
            &[JValue::from(&arr)],
        )
        .unwrap();

        self
    }

    pub fn allowed_prefixes<'a, S: AsRef<&'a str>>(&self, strings: &[S]) -> &Self {
        let mut env = jvm::attach_current_thread_permanently();

        let string_cls = env.find_class("java/lang/String").unwrap();
        let arr = env
            .new_object_array(strings.len() as i32, string_cls, JObject::null())
            .unwrap();

        for (i, p) in strings.iter().enumerate() {
            let str = env.new_string(p.as_ref()).unwrap();
            env.set_object_array_element(&arr, i as i32, str).unwrap();
        }

        env.call_method(
            self,
            "allowedPrefixes",
            "([Ljava/lang/String;)Lorg/jetbrains/java/decompiler/api/Decompiler$Builder;",
            &[JValue::from(&arr)],
        )
        .unwrap();

        self
    }

    pub fn build(&self) -> Decompiler {
        let mut env = jvm::attach_current_thread_permanently();

        let decompiler = env
            .call_method(
                self,
                "build",
                "()Lorg/jetbrains/java/decompiler/api/Decompiler;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();

        let global = env.new_global_ref(decompiler).unwrap();
        Decompiler(global)
    }
}

impl Default for DecompilerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub enum PreferenceValue {
    Boolean(bool),
    Integer(i32),
    String(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
            .is_test(true)
            .init();
    }

    #[test]
    fn test_decompiler_builder() {
        init_logger();

        let decompiler = Decompiler::builder()
            .inputs(&["/home/ithundxr/Downloads/vineflower-1.11.2.jar"])
            .output_to_directory("/home/ithundxr/Downloads/vineflower-1.11-2")
            .build();

        decompiler.decompile();

        assert!(true)
    }
}
