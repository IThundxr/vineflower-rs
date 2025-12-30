use jni::{AttachGuard, InitArgsBuilder, JNIEnv, JNIVersion, JavaVM};
use once_cell::sync::OnceCell;
use std::sync::Arc;

// TODO - Better handle getting the JNIEnv and creating the JVM instance
fn jvm() -> &'static Arc<JavaVM> {
    static JVM: OnceCell<Arc<JavaVM>> = OnceCell::new();

    JVM.get_or_init(|| {
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option(format!("-Djava.class.path={}", env!("BUNDLED_JAR_PATH")))
            .build()
            .unwrap_or_else(|e| panic!("JNI init args error: {e:#?}"));

        let vm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("Failed to create JVM: {e:#?}"));
        Arc::new(vm)
    })
}

pub(crate) fn attach_current_thread() -> AttachGuard<'static> {
    jvm()
        .attach_current_thread()
        .expect("Failed to attach jvm thread")
}

pub(crate) fn attach_current_thread_as_daemon() -> JNIEnv<'static> {
    jvm()
        .attach_current_thread_as_daemon()
        .expect("Failed to attach jvm daemon thread")
}

pub(crate) fn attach_current_thread_permanently() -> JNIEnv<'static> {
    jvm()
        .attach_current_thread_permanently()
        .expect("Failed to attach jvm thread permanently")
}
