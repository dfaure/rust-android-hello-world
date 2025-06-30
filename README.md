Hello world in Rust, for Android

This is a minimal test application based on `NativeActivity` that just
outputs a line into a file. It doesn't use logcat so that it can even
be built and run on an Android device directly, without a PC.
In that scenario, the app runs under a different UID than Termux
so logcat in Termux can't see the app's logs. This is why
logging to a file is used instead.


# Gradle Build on an Android device

See HOWTO for the initial setup (TODO insert URL here)

```
cargo build --target aarch64-linux-android --release
cp target/aarch64-linux-android/release/*.so app/src/main/jniLibs/arm64-v8a/
gradle assembleDebug
cp app/build/outputs/apk/debug/app-debug.apk /sdcard/Download/
```

# Gradle Build on a Linux host
```
export ANDROID_NDK_HOME="path/to/ndk"
export ANDROID_HOME="path/to/sdk"

rustup target add aarch64-linux-android
cargo install cargo-ndk

cargo ndk -t arm64-v8a -o app/src/main/jniLibs/  build
./gradlew build
./gradlew installDebug
```

# Cargo APK Build
Since this test doesn't require a custom `Activity` subclass it's
optionally possible to build this example with `cargo apk`.

```
export ANDROID_NDK_HOME="path/to/ndk"
export ANDROID_SDK_HOME="path/to/sdk"

rustup target add aarch64-linux-android
cargo install cargo-apk

cargo apk build
cargo apk run
```
