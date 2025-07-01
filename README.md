Hello world in Rust, for Android

This is a minimal test application based on `NativeActivity` that just
outputs a line into a file (see below why it does that instead of log!).

30 June 2024, south of France, it's too hot to turn on a PC,
I want to do some Rust hacking directly on my Android tablet.
How hard can it be?

# Gradle Build on an Android device

This is an updated version of an old comment on [a termux github issue](https://github.com/termux/termux-packages/pull/7227)

## Install termux

To be able to type commands and install packages, install termux from F-Droid (NOT from Google Play!!!) or from [github](https://github.com/termux/termux-app/releases). Ignore the big fat warning about the android version, there’s a small « install anyway » button/link.

```
pkg upgrade
pkg install git vim rust
pkg install openjdk-17 gradle android-tools
```

## Install cmdline-tools

First, open [https://developer.android.com/studio](dhttps://developer.android.com/studio) and scroll down to command line tools to find the last release and update the version number into the URL below.

```
curl -L https://dl.google.com/android/repository/commandlinetools-linux-13114758_latest.zip -o commandlinetools-linux-latest.zip
mkdir -p ~/Android/sdk/cmdline-tools
unzip commandlinetools-linux-latest.zip -d ~/Android/sdk/cmdline-tools
mv ~/Android/sdk/cmdline-tools/cmdline-tools ~/Android/sdk/cmdline-tools/latest
```

## Install android-sdk-tools

Open [https://github.com/lzhiyong/android-sdk-tools/releases](https://github.com/lzhiyong/android-sdk-tools/releases) to find out the version number for the URL below.
```
curl -L https://github.com/lzhiyong/android-sdk-tools/releases/download/35.0.2/android-sdk-tools-static-aarch64.zip -o tools.zip
unzip tools.zip
# check you can run it (that’s what doesn’t work with the Google Play version of termux)
build-tools/aapt2

rm -rf ~/Android/sdk/build-tools/latest
mkdir -p ~/Android/sdk/build-tools
mv build-tools ~/Android/sdk/build-tools/latest
rm -rf ~/Android/sdk/platform-tools-latest
mkdir -p ~/Android/sdk
mv platform-tools ~/Android/sdk/platform-tools-latest
```

## Set up environment variables
```
export ANDROID_HOME=~/Android/sdk/
export JAVA_HOME=$PREFIX/lib/jvm/java-17-openjdk
export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$PATH
export PATH=$ANDROID_HOME/build-tools/latest:$PATH
```

## Install Android Platforms
```
yes | sdkmanager –licenses
yes | sdkmanager "platforms;android-30"
```

## Force using the ARM version of aapt2
By default gradle will try to run the x86\_64 version of aapt2, while we obviously want to run the ARM version.

```
echo "android.aapt2FromMavenOverride=$ANDROID_HOME/build-tools/latest/aapt2" >> ~/.gradle/gradle.properties
```

## Self-signing on Android
```
keytool -genkey -v -keystore $HOME/debug.keystore -storepass android -alias androiddebugkey -keypass android -keyalg RSA -keysize 2048 -validity 10000
```
Then edit `app/build.gradle` (NOT the toplevel one!) and add inside the android section:
```
 signingConfigs {
        debug {
            storeFile = file("/data/data/com.termux/files/home/debug.keystore")
            storePassword = "android"
            keyAlias = "androiddebugkey"
            keyPassword = "android"
        }
    }
```
and in the debug section, add `signingConfig signingConfigs.debug`.

## Building on Android

Now we can finally build this hello world example.

```
cd rust-android-hello-world
cargo build --target aarch64-linux-android --release
cp target/aarch64-linux-android/release/*.so app/src/main/jniLibs/arm64-v8a/
./gradlew assembleDebug
cp app/build/outputs/apk/debug/app-debug.apk /sdcard/Download/
```

Tip: create an alias for those last 4 commands separated by `&&`.

## Installing and running

I didn't get `termux-open app-debug.apk` to work (it gives an unspecific error about the package).
What works is to copy it to a folder like Download (as done in the previous section),
launching a file manager to install the APK by clicking on it (after Android analyzes it).
Finally, it offers to run it.

## Logging

This example doesn't use `log!()` because when building
and running the application directly on an Android device,
and typing `logcat` in termux, nothing appeared.
It turns out the app runs under a different UID than Termux
so logcat in Termux can't see the app's logs for security
reasons. This is why logging to a file is used instead.

# Gradle Build on a Linux host (not tested)

According to the [rust-mobile example](https://github.com/rust-mobile/rust-android-examples/tree/main/na-mainloop) I used as inspiration, this should work:
```
export ANDROID_NDK_HOME="path/to/ndk"
export ANDROID_HOME="path/to/sdk"

rustup target add aarch64-linux-android
cargo install cargo-ndk

cargo ndk -t arm64-v8a -o app/src/main/jniLibs/  build
./gradlew build
./gradlew installDebug
```

# Cargo APK Build on a Linux host (not tested)

The [rust-mobile example](https://github.com/rust-mobile/rust-android-examples/tree/main/na-mainloop) I used as inspiration, says:

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

# Thoughts

Does this mean cargo-ndk should gain support for building on Android?
Currently it says `You cannot build cargo-ndk _for_ Android`.
We could write a similar tool that just runs cargo and copies libs to the specified directory maybe...
